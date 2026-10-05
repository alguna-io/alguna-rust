pub use crate::prelude::*;

/// A checkout session's payment succeeded, for both hosted and embedded checkout.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct WebhookCheckoutSessionCompletedPayload {
    pub data: CheckoutSessionPayload,
    /// When the event occurred, RFC 3339
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub timestamp: DateTime<FixedOffset>,
    /// Event type
    pub r#type: String,
}

impl WebhookCheckoutSessionCompletedPayload {
    pub fn builder() -> WebhookCheckoutSessionCompletedPayloadBuilder {
        <WebhookCheckoutSessionCompletedPayloadBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct WebhookCheckoutSessionCompletedPayloadBuilder {
    data: Option<CheckoutSessionPayload>,
    timestamp: Option<DateTime<FixedOffset>>,
    r#type: Option<String>,
}

impl WebhookCheckoutSessionCompletedPayloadBuilder {
    pub fn data(mut self, value: CheckoutSessionPayload) -> Self {
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

    /// Consumes the builder and constructs a [`WebhookCheckoutSessionCompletedPayload`].
    /// This method will fail if any of the following fields are not set:
    /// - [`data`](WebhookCheckoutSessionCompletedPayloadBuilder::data)
    /// - [`timestamp`](WebhookCheckoutSessionCompletedPayloadBuilder::timestamp)
    /// - [`r#type`](WebhookCheckoutSessionCompletedPayloadBuilder::r#type)
    pub fn build(self) -> Result<WebhookCheckoutSessionCompletedPayload, BuildError> {
        Ok(WebhookCheckoutSessionCompletedPayload {
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
