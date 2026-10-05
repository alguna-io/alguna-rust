pub use crate::prelude::*;

/// The body of every error the API returns.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ErrorResponse {
    /// Explanation of what went wrong
    pub detail: String,
    /// HTTP status code of the response
    pub status: i64,
    /// HTTP status text. Sent only by version, idempotency and credit-refusal errors.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// Always `about:blank`. Sent only by version and idempotency errors.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

impl ErrorResponse {
    pub fn builder() -> ErrorResponseBuilder {
        <ErrorResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ErrorResponseBuilder {
    detail: Option<String>,
    status: Option<i64>,
    title: Option<String>,
    r#type: Option<String>,
}

impl ErrorResponseBuilder {
    pub fn detail(mut self, value: impl Into<String>) -> Self {
        self.detail = Some(value.into());
        self
    }

    pub fn status(mut self, value: i64) -> Self {
        self.status = Some(value);
        self
    }

    pub fn title(mut self, value: impl Into<String>) -> Self {
        self.title = Some(value.into());
        self
    }

    pub fn r#type(mut self, value: impl Into<String>) -> Self {
        self.r#type = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ErrorResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`detail`](ErrorResponseBuilder::detail)
    /// - [`status`](ErrorResponseBuilder::status)
    pub fn build(self) -> Result<ErrorResponse, BuildError> {
        Ok(ErrorResponse {
            detail: self
                .detail
                .ok_or_else(|| BuildError::missing_field("detail"))?,
            status: self
                .status
                .ok_or_else(|| BuildError::missing_field("status"))?,
            title: self.title,
            r#type: self.r#type,
        })
    }
}
