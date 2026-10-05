pub use crate::prelude::*;

/// A payment was initiated.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct WebhookPaymentCreatedPayload {
    pub data: PaymentPayload,
    /// When the event occurred, RFC 3339
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub timestamp: DateTime<FixedOffset>,
    /// Event type
    pub r#type: String,
}

impl WebhookPaymentCreatedPayload {
    pub fn builder() -> WebhookPaymentCreatedPayloadBuilder {
        <WebhookPaymentCreatedPayloadBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct WebhookPaymentCreatedPayloadBuilder {
    data: Option<PaymentPayload>,
    timestamp: Option<DateTime<FixedOffset>>,
    r#type: Option<String>,
}

impl WebhookPaymentCreatedPayloadBuilder {
    pub fn data(mut self, value: PaymentPayload) -> Self {
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

    /// Consumes the builder and constructs a [`WebhookPaymentCreatedPayload`].
    /// This method will fail if any of the following fields are not set:
    /// - [`data`](WebhookPaymentCreatedPayloadBuilder::data)
    /// - [`timestamp`](WebhookPaymentCreatedPayloadBuilder::timestamp)
    /// - [`r#type`](WebhookPaymentCreatedPayloadBuilder::r#type)
    pub fn build(self) -> Result<WebhookPaymentCreatedPayload, BuildError> {
        Ok(WebhookPaymentCreatedPayload {
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
