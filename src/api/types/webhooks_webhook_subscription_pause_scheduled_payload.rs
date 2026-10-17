pub use crate::prelude::*;

/// A pause was scheduled for a future date. Not enabled by default — contact us to turn it on.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct WebhookSubscriptionPauseScheduledPayload {
    pub data: SubscriptionPayload,
    /// When the event occurred, RFC 3339
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub timestamp: DateTime<FixedOffset>,
    /// Event type
    pub r#type: String,
}

impl WebhookSubscriptionPauseScheduledPayload {
    pub fn builder() -> WebhookSubscriptionPauseScheduledPayloadBuilder {
        <WebhookSubscriptionPauseScheduledPayloadBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct WebhookSubscriptionPauseScheduledPayloadBuilder {
    data: Option<SubscriptionPayload>,
    timestamp: Option<DateTime<FixedOffset>>,
    r#type: Option<String>,
}

impl WebhookSubscriptionPauseScheduledPayloadBuilder {
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

    /// Consumes the builder and constructs a [`WebhookSubscriptionPauseScheduledPayload`].
    /// This method will fail if any of the following fields are not set:
    /// - [`data`](WebhookSubscriptionPauseScheduledPayloadBuilder::data)
    /// - [`timestamp`](WebhookSubscriptionPauseScheduledPayloadBuilder::timestamp)
    /// - [`r#type`](WebhookSubscriptionPauseScheduledPayloadBuilder::r#type)
    pub fn build(self) -> Result<WebhookSubscriptionPauseScheduledPayload, BuildError> {
        Ok(WebhookSubscriptionPauseScheduledPayload {
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
