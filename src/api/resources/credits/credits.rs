use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, RequestOptions};
use reqwest::Method;

pub struct CreditsClient {
    pub http_client: HttpClient,
}

impl CreditsClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Prices work before it happens and, optionally, reserves credit for it. The request carries no credit amount: describe the work as the events it will produce, and the response's `charges` say what it prices to per metric.
    ///
    /// Send `events` alone to get a price with nothing recorded. Send `reserve` to open a hold, then close it with `POST /credits/track`. `"reserve": {}` with `events` holds enough for exactly that work; `"reserve": {"max_quantity": "100000"}` with `event_name` holds a ceiling in metric units. An `Idempotency-Key` is required whenever `reserve` is set. Each metric is held at the rate frozen when the hold opened, and the hold expires after 15 minutes unless `ttl_seconds` asks for longer (maximum 4 hours).
    ///
    /// Only credit-gated metrics are accepted: an event feeding no credit-consuming metric is rejected with `422` and belongs on `POST /events`. Insufficient credit returns `200` with `allowed: false`; an `allowed: true` response carrying no `reservation_id` opened no hold.
    ///
    /// The SDKs send no automatic `Idempotency-Key` on this call and retry it without one, because a price check records nothing. Send your own key only to open a hold, where it is required and identifies the hold, or to replay a specific response; a retry then sends the same key. The SDKs do not retry a `500`. `409` means the key is bound to a different reservation. Back off on `429`, which also covers a ledger at its concurrent-hold cap.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use alguna::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         api_key: Some("<token>".to_string()),
    ///         ..Default::default()
    ///     };
    ///     let client = Alguna::new(config).expect("Failed to build client");
    ///     client
    ///         .credits
    ///         .check(
    ///             &CheckCreditsRequest {
    ///                 account: "cust_abc123".to_string(),
    ///                 event_name: None,
    ///                 events: None,
    ///                 properties: None,
    ///                 reserve: None,
    ///                 ttl_seconds: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn check(
        &self,
        request: &CheckCreditsRequest,
        options: Option<RequestOptions>,
    ) -> Result<CheckCreditsResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "credits/check",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Reports a reservation's current state, what it covers, and every close already committed against it.
    ///
    /// `closes` exists for one recovery: a client that has lost its `Idempotency-Key` and cannot reproduce the events it sent can read what already landed, instead of resending and burning twice. This is a read and cannot settle anything. An unknown or foreign token returns `404`, indistinguishably.
    ///
    /// # Arguments
    ///
    /// * `reservation_id` - Unique identifier for the reservation
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use alguna::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         api_key: Some("<token>".to_string()),
    ///         ..Default::default()
    ///     };
    ///     let client = Alguna::new(config).expect("Failed to build client");
    ///     client
    ///         .credits
    ///         .get_reservation(&"reservation_id".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn get_reservation(
        &self,
        reservation_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<ReservationStatusResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("credits/reservations/{}", reservation_id),
                None,
                None,
                options,
            )
            .await
    }

    /// Records usage and charges credit for it. The request carries no credit amount: send what happened, and the response's `charges` carry what it priced to per metric, with `totals` as the sum.
    ///
    /// Two shapes. A direct draw (`account` + `events`) charges against the account's current credit. A settlement (`reservation_id` + `events` + `close`) closes a hold opened by `/credits/check`, pricing each metric at the rate frozen when the hold opened. One payload can feed several credit-consuming metrics, and the draw is all-or-nothing: a refusal for one metric leaves every metric uncharged.
    ///
    /// Events sent here are both metered and charged, so do not also send them to `POST /events`. An event feeding no credit-consuming metric is rejected with `422` and belongs there instead. Insufficient credit returns `200` with `allowed: false`.
    ///
    /// The events' own `unique_id`s identify the work, so a resent batch draws once. The SDKs send no automatic `Idempotency-Key` on this call and retry it without one. Send your own key only to replay a specific response; it then replaces the `unique_id`s as the identity, so reuse it when you resend the same work. The SDKs do not retry a `500`. `409` means the identity is bound to different work. Back off on `429`.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use alguna::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         api_key: Some("<token>".to_string()),
    ///         ..Default::default()
    ///     };
    ///     let client = Alguna::new(config).expect("Failed to build client");
    ///     client
    ///         .credits
    ///         .track(
    ///             &TrackCreditsRequest {
    ///                 events: vec![TrackCreditEvent {
    ///                     event_name: "agent_step".to_string(),
    ///                     unique_id: "run_88f_step1".to_string(),
    ///                     ..Default::default()
    ///                 }],
    ///                 account: None,
    ///                 close: None,
    ///                 on_envelope_exceeded: None,
    ///                 reservation_id: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn track(
        &self,
        request: &TrackCreditsRequest,
        options: Option<RequestOptions>,
    ) -> Result<TrackCreditsResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "credits/track",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }
}
