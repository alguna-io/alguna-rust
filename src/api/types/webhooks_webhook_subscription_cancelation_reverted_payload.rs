pub use crate::prelude::*;

/// A scheduled cancellation was called off before taking effect.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct WebhookSubscriptionCancelationRevertedPayload {
    pub data: SubscriptionPayload,
    /// When the event occurred, RFC 3339
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub timestamp: DateTime<FixedOffset>,
    /// Event type
    pub r#type: String,
}

impl WebhookSubscriptionCancelationRevertedPayload {
    pub fn builder() -> WebhookSubscriptionCancelationRevertedPayloadBuilder {
        <WebhookSubscriptionCancelationRevertedPayloadBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct WebhookSubscriptionCancelationRevertedPayloadBuilder {
    data: Option<SubscriptionPayload>,
    timestamp: Option<DateTime<FixedOffset>>,
    r#type: Option<String>,
}

impl WebhookSubscriptionCancelationRevertedPayloadBuilder {
    pub fn data(mut self, value: SubscriptionPayload) -> Self {
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

    /// Consumes the builder and constructs a [`WebhookSubscriptionCancelationRevertedPayload`].
    /// This method will fail if any of the following fields are not set:
    /// - [`data`](WebhookSubscriptionCancelationRevertedPayloadBuilder::data)
    /// - [`timestamp`](WebhookSubscriptionCancelationRevertedPayloadBuilder::timestamp)
    /// - [`r#type`](WebhookSubscriptionCancelationRevertedPayloadBuilder::r#type)
    pub fn build(self) -> Result<WebhookSubscriptionCancelationRevertedPayload, BuildError> {
        Ok(WebhookSubscriptionCancelationRevertedPayload {
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
