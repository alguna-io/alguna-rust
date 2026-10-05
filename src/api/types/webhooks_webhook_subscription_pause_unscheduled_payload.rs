pub use crate::prelude::*;

/// A scheduled pause was called off before taking effect. Not enabled by default — contact us to turn it on.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct WebhookSubscriptionPauseUnscheduledPayload {
    pub data: SubscriptionPayload,
    /// When the event occurred, RFC 3339
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub timestamp: DateTime<FixedOffset>,
    /// Event type
    pub r#type: String,
}

impl WebhookSubscriptionPauseUnscheduledPayload {
    pub fn builder() -> WebhookSubscriptionPauseUnscheduledPayloadBuilder {
        <WebhookSubscriptionPauseUnscheduledPayloadBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct WebhookSubscriptionPauseUnscheduledPayloadBuilder {
    data: Option<SubscriptionPayload>,
    timestamp: Option<DateTime<FixedOffset>>,
    r#type: Option<String>,
}

impl WebhookSubscriptionPauseUnscheduledPayloadBuilder {
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

    /// Consumes the builder and constructs a [`WebhookSubscriptionPauseUnscheduledPayload`].
    /// This method will fail if any of the following fields are not set:
    /// - [`data`](WebhookSubscriptionPauseUnscheduledPayloadBuilder::data)
    /// - [`timestamp`](WebhookSubscriptionPauseUnscheduledPayloadBuilder::timestamp)
    /// - [`r#type`](WebhookSubscriptionPauseUnscheduledPayloadBuilder::r#type)
    pub fn build(self) -> Result<WebhookSubscriptionPauseUnscheduledPayload, BuildError> {
        Ok(WebhookSubscriptionPauseUnscheduledPayload {
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
