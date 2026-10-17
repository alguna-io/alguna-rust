pub use crate::prelude::*;

/// Returned by the API gateway when a request carries no valid API key, before the API handles it.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct GatewayErrorResponse {
    /// Machine-readable error code, e.g. `unauthenticated`
    pub code: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<HashMap<String, serde_json::Value>>,
    /// Explanation of what went wrong
    pub message: String,
}

impl GatewayErrorResponse {
    pub fn builder() -> GatewayErrorResponseBuilder {
        <GatewayErrorResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GatewayErrorResponseBuilder {
    code: Option<String>,
    details: Option<HashMap<String, serde_json::Value>>,
    message: Option<String>,
}

impl GatewayErrorResponseBuilder {
    pub fn code(mut self, value: impl Into<String>) -> Self {
        self.code = Some(value.into());
        self
    }

    pub fn details(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.details = Some(value);
        self
    }

    pub fn message(mut self, value: impl Into<String>) -> Self {
        self.message = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`GatewayErrorResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`code`](GatewayErrorResponseBuilder::code)
    /// - [`message`](GatewayErrorResponseBuilder::message)
    pub fn build(self) -> Result<GatewayErrorResponse, BuildError> {
        Ok(GatewayErrorResponse {
            code: self.code.ok_or_else(|| BuildError::missing_field("code"))?,
            details: self.details,
            message: self
                .message
                .ok_or_else(|| BuildError::missing_field("message"))?,
        })
    }
}
