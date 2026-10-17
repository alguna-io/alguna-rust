pub use crate::prelude::*;

/// A subscription resumed after a pause. Not enabled by default — contact us to turn it on.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct WebhookSubscriptionResumedPayload {
    pub data: SubscriptionPayload,
    /// When the event occurred, RFC 3339
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub timestamp: DateTime<FixedOffset>,
    /// Event type
    pub r#type: String,
}

impl WebhookSubscriptionResumedPayload {
    pub fn builder() -> WebhookSubscriptionResumedPayloadBuilder {
        <WebhookSubscriptionResumedPayloadBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct WebhookSubscriptionResumedPayloadBuilder {
    data: Option<SubscriptionPayload>,
    timestamp: Option<DateTime<FixedOffset>>,
    r#type: Option<String>,
}

impl WebhookSubscriptionResumedPayloadBuilder {
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

    /// Consumes the builder and constructs a [`WebhookSubscriptionResumedPayload`].
    /// This method will fail if any of the following fields are not set:
    /// - [`data`](WebhookSubscriptionResumedPayloadBuilder::data)
    /// - [`timestamp`](WebhookSubscriptionResumedPayloadBuilder::timestamp)
    /// - [`r#type`](WebhookSubscriptionResumedPayloadBuilder::r#type)
    pub fn build(self) -> Result<WebhookSubscriptionResumedPayload, BuildError> {
        Ok(WebhookSubscriptionResumedPayload {
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
