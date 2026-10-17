pub use crate::prelude::*;

/// A top-level subscription attribute changed: name, contract dates, renewal or invoice settings, or metadata. Pricing changes are reported on subscription.version.updated.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct WebhookSubscriptionUpdatedPayload {
    pub data: SubscriptionPayload,
    /// When the event occurred, RFC 3339
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub timestamp: DateTime<FixedOffset>,
    /// Event type
    pub r#type: String,
}

impl WebhookSubscriptionUpdatedPayload {
    pub fn builder() -> WebhookSubscriptionUpdatedPayloadBuilder {
        <WebhookSubscriptionUpdatedPayloadBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct WebhookSubscriptionUpdatedPayloadBuilder {
    data: Option<SubscriptionPayload>,
    timestamp: Option<DateTime<FixedOffset>>,
    r#type: Option<String>,
}

impl WebhookSubscriptionUpdatedPayloadBuilder {
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

    /// Consumes the builder and constructs a [`WebhookSubscriptionUpdatedPayload`].
    /// This method will fail if any of the following fields are not set:
    /// - [`data`](WebhookSubscriptionUpdatedPayloadBuilder::data)
    /// - [`timestamp`](WebhookSubscriptionUpdatedPayloadBuilder::timestamp)
    /// - [`r#type`](WebhookSubscriptionUpdatedPayloadBuilder::r#type)
    pub fn build(self) -> Result<WebhookSubscriptionUpdatedPayload, BuildError> {
        Ok(WebhookSubscriptionUpdatedPayload {
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
