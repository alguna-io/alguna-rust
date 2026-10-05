pub use crate::prelude::*;

/// Credits were added to a customer account.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct WebhookAccountCreditsGrantedPayload {
    pub data: CreditGrantedPayload,
    /// When the event occurred, RFC 3339
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub timestamp: DateTime<FixedOffset>,
    /// Event type
    pub r#type: String,
}

impl WebhookAccountCreditsGrantedPayload {
    pub fn builder() -> WebhookAccountCreditsGrantedPayloadBuilder {
        <WebhookAccountCreditsGrantedPayloadBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct WebhookAccountCreditsGrantedPayloadBuilder {
    data: Option<CreditGrantedPayload>,
    timestamp: Option<DateTime<FixedOffset>>,
    r#type: Option<String>,
}

impl WebhookAccountCreditsGrantedPayloadBuilder {
    pub fn data(mut self, value: CreditGrantedPayload) -> Self {
        self.data = Some(value);
        self
    }

    pub fn timestamp(mut self, value: DateTime<FixedOffset>) -> Self {
        self.timestamp = Some(value);
        self
    }

    pub fn r#type(mut self, value: impl Into<String>) -> Self {
        self.r#type = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`WebhookAccountCreditsGrantedPayload`].
    /// This method will fail if any of the following fields are not set:
    /// - [`data`](WebhookAccountCreditsGrantedPayloadBuilder::data)
    /// - [`timestamp`](WebhookAccountCreditsGrantedPayloadBuilder::timestamp)
    /// - [`r#type`](WebhookAccountCreditsGrantedPayloadBuilder::r#type)
    pub fn build(self) -> Result<WebhookAccountCreditsGrantedPayload, BuildError> {
        Ok(WebhookAccountCreditsGrantedPayload {
            data: self.data.ok_or_else(|| BuildError::missing_field("data"))?,
            timestamp: self
                .timestamp
                .ok_or_else(|| BuildError::missing_field("timestamp"))?,
            r#type: self
                .r#type
                .ok_or_else(|| BuildError::missing_field("r#type"))?,
        })
    }
}
