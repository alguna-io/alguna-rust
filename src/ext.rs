//! The hand-written layer over the generated client. Generation declares it in
//! the generated lib.rs, and targets/rust/patch makes the generated code call it.

use crate::core::join_url;
use crate::{ApiClientBuilder, ApiError, ClientConfig, RequestOptions};
use reqwest::header::{HeaderMap, HeaderValue, RETRY_AFTER};
use reqwest::{Client, Method, Request, Response, StatusCode, Url};
use std::time::Duration;

// Generated from the API spec.
mod identified_by_body;

pub(crate) const VERSION_HEADER: &str = "Alguna-Version";
const IDEMPOTENCY_KEY: &str = "Idempotency-Key";
/// The longest wait before a retry, whatever `Retry-After` asks for.
const MAX_WAIT: Duration = Duration::from_secs(60);

impl ApiClientBuilder {
    /// Pins the API version, sent as `Alguna-Version` on every request. The default is
    /// the version this SDK was generated from.
    pub fn api_version(self, version: impl Into<String>) -> Self {
        self.custom_header(VERSION_HEADER, version)
    }
}

impl RequestOptions {
    /// Sends this request with another API version.
    pub fn api_version(self, version: impl Into<String>) -> Self {
        self.additional_header(VERSION_HEADER, version)
    }

    /// Sent as `Idempotency-Key` in place of any key the SDK makes. A request repeated
    /// with the same key returns the original result instead of acting twice.
    pub fn idempotency_key(self, key: impl Into<String>) -> Self {
        self.additional_header(IDEMPOTENCY_KEY, key)
    }
}

impl ApiError {
    /// The HTTP status of an API error response, or `None` for a failure before or after
    /// one (network, serialization, configuration).
    pub fn status(&self) -> Option<u16> {
        self.response().map(|(status, _)| status)
    }

    /// The API's reason for an error response: the body's `detail`, else its `title`,
    /// else its `message`, else the HTTP status text.
    pub fn detail(&self) -> Option<&str> {
        self.response().map(|(_, reason)| reason)
    }

    /// True when the request ran past its timeout.
    pub fn is_timeout(&self) -> bool {
        matches!(self, Self::Network(error) if error.is_timeout())
    }

    /// The status and reason of an API error response. Every variant is listed on purpose:
    /// a new error in the spec fails to compile here until it is given a status.
    fn response(&self) -> Option<(u16, &str)> {
        match self {
            Self::BadRequestError { message, .. } => Some((400, message)),
            Self::UnauthorizedError { message, .. } => Some((401, message)),
            Self::ForbiddenError { message, .. } => Some((403, message)),
            Self::NotFoundError { message, .. } => Some((404, message)),
            Self::ConflictError { message, .. } => Some((409, message)),
            Self::UnprocessableEntityError { message, .. } => Some((422, message)),
            Self::TooManyRequestsError { message, .. } => Some((429, message)),
            Self::InternalServerError { message, .. } => Some((500, message)),
            Self::BadGatewayError { message } => Some((502, message)),
            Self::ServiceUnavailableError { message, .. } => Some((503, message)),
            Self::GatewayTimeoutError { message, .. } => Some((504, message)),
            Self::Http { status, message } => Some((*status, message)),
            Self::Network(_)
            | Self::Executor(_)
            | Self::Serialization(_)
            | Self::Configuration(_)
            | Self::InvalidHeader
            | Self::RequestClone
            | Self::StreamTerminated
            | Self::StreamTimeout
            | Self::SseParseError(_) => None,
        }
    }
}

/// Fails `Alguna::new`, and so `ApiClientBuilder::build`, without an API key: every
/// request would get a 401.
pub(crate) fn require_api_key(config: &ClientConfig) -> Result<(), ApiError> {
    match config.api_key.as_deref() {
        Some(key) if !key.is_empty() => Ok(()),
        _ => Err(ApiError::Configuration(
            "no API key: set ALGUNA_API_KEY or call ApiClientBuilder::api_key".to_string(),
        )),
    }
}

/// The `message` of an error response (`ApiError::from_response` calls this) from its
/// JSON body, if it has one. Alguna errors carry `detail` and `title`; the gateway's 401
/// carries `message`.
pub(crate) fn error_message(status: u16, body: Option<&serde_json::Value>) -> String {
    ["detail", "title", "message"]
        .iter()
        .find_map(|key| {
            body?
                .get(*key)?
                .as_str()
                .filter(|reason| !reason.is_empty())
        })
        .or_else(|| StatusCode::from_u16(status).ok()?.canonical_reason())
        .unwrap_or("Unknown error")
        .to_string()
}

/// Sends `request` (`HttpClient::send_request` calls this). A request other than a GET
/// gets an Idempotency-Key unless the caller set one, and every attempt sends the same
/// key, so the API acts once however many attempts reach it. That is what makes a write
/// as safe to retry as a read. The API ignores an empty key, so an empty key is replaced.
/// An operation that the API identifies by its body gets no key from the SDK: a retry
/// with the same body cannot act twice, and on `credits.track` a new key on each call
/// would replace the events' `unique_id`s as the identity of the draw.
pub(crate) async fn send_with_retries(
    client: &Client,
    mut request: Request,
    options: Option<&RequestOptions>,
    config: &ClientConfig,
) -> Result<Response, ApiError> {
    let has_key = request
        .headers()
        .get(IDEMPOTENCY_KEY)
        .is_some_and(|key| !key.is_empty());
    if request.method() != Method::GET
        && !has_key
        && !is_identified_by_body(&request, &config.base_url)
    {
        let key = HeaderValue::from_str(&uuid::Uuid::new_v4().to_string())
            .expect("a UUID is a valid header value");
        request.headers_mut().insert(IDEMPOTENCY_KEY, key);
    }
    let max_retries = options
        .and_then(|options| options.max_retries)
        .unwrap_or(config.max_retries);
    // `None` leaves the client's timeout in force.
    let timeout = options
        .and_then(|options| options.timeout_seconds)
        .map(Duration::from_secs);

    let mut retry = 0;
    loop {
        let mut attempt = request.try_clone().ok_or(ApiError::RequestClone)?;
        *attempt.timeout_mut() = timeout;
        let result = client.execute(attempt).await;
        let wait = match &result {
            Ok(response) => retry_wait(response.status(), response.headers(), retry),
            // A network error: refused, reset, closed before the response, or timed out.
            // Not a request reqwest could not build or a redirect it would not follow.
            Err(error) => error.is_request().then(|| backoff(retry)),
        };
        if let Some(wait) = wait.filter(|_| retry < max_retries) {
            tokio::time::sleep(wait).await;
            retry += 1;
            continue;
        }

        let response = result.map_err(ApiError::Network)?;
        if response.status().is_success() {
            return Ok(response);
        }
        let status = response.status().as_u16();
        let body = response.text().await.ok();
        return Err(ApiError::from_response(status, body.as_deref()));
    }
}

/// Whether `request` is an operation that the spec marks `x-alguna-identified-by-body`: its
/// URL path is the one that `join_url` makes for the operation under `base_url`.
fn is_identified_by_body(request: &Request, base_url: &str) -> bool {
    identified_by_body::OPERATIONS.iter().any(|(method, path)| {
        request.method().as_str() == *method
            && Url::parse(&join_url(base_url, path))
                .is_ok_and(|url| url.path() == request.url().path())
    })
}

/// How long to wait before retrying this response, or `None` when it is final.
fn retry_wait(status: StatusCode, headers: &HeaderMap, retry: u32) -> Option<Duration> {
    let retry_after = headers.get(RETRY_AFTER);
    let retryable = match status.as_u16() {
        408 | 429 | 502 | 503 | 504 => true,
        // The API sends Retry-After on a 409 only while a request with the same
        // Idempotency-Key is still running. Any other 409 is a real conflict.
        409 => retry_after.is_some(),
        _ => false,
    };
    retryable.then(|| {
        retry_after
            .and_then(parse_retry_after)
            .unwrap_or_else(|| backoff(retry))
    })
}

/// `Retry-After` as seconds or as an HTTP date. A date in the past means now.
fn parse_retry_after(value: &HeaderValue) -> Option<Duration> {
    let value = value.to_str().ok()?.trim();
    let wait = match value.parse::<u64>() {
        Ok(seconds) => Duration::from_secs(seconds),
        Err(_) => chrono::DateTime::parse_from_rfc2822(value)
            .ok()?
            .signed_duration_since(chrono::Utc::now())
            .to_std()
            .unwrap_or_default(),
    };
    Some(wait.min(MAX_WAIT))
}

/// 1 s, 2 s, 4 s and so on, each ±10%: the curve of the other Alguna SDKs.
fn backoff(retry: u32) -> Duration {
    let exponential = Duration::from_secs(1).saturating_mul(2_u32.saturating_pow(retry));
    exponential
        .mul_f64(0.9 + fastrand::f64() * 0.2)
        .min(MAX_WAIT)
}
