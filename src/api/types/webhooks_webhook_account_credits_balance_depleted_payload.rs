pub use crate::prelude::*;

/// A customer's credit balance reached zero.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct WebhookAccountCreditsBalanceDepletedPayload {
    pub data: CreditBalancePayload,
    /// When the event occurred, RFC 3339
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub timestamp: DateTime<FixedOffset>,
    /// Event type
    pub r#type: String,
}

impl WebhookAccountCreditsBalanceDepletedPayload {
    pub fn builder() -> WebhookAccountCreditsBalanceDepletedPayloadBuilder {
        <WebhookAccountCreditsBalanceDepletedPayloadBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct WebhookAccountCreditsBalanceDepletedPayloadBuilder {
    data: Option<CreditBalancePayload>,
    timestamp: Option<DateTime<FixedOffset>>,
    r#type: Option<String>,
}

impl WebhookAccountCreditsBalanceDepletedPayloadBuilder {
    pub fn data(mut self, value: CreditBalancePayload) -> Self {
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

    /// Consumes the builder and constructs a [`WebhookAccountCreditsBalanceDepletedPayload`].
    /// This method will fail if any of the following fields are not set:
    /// - [`data`](WebhookAccountCreditsBalanceDepletedPayloadBuilder::data)
    /// - [`timestamp`](WebhookAccountCreditsBalanceDepletedPayloadBuilder::timestamp)
    /// - [`r#type`](WebhookAccountCreditsBalanceDepletedPayloadBuilder::r#type)
    pub fn build(self) -> Result<WebhookAccountCreditsBalanceDepletedPayload, BuildError> {
        Ok(WebhookAccountCreditsBalanceDepletedPayload {
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
