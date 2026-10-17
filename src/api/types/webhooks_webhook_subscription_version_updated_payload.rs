pub use crate::prelude::*;

/// A subscription version was created, updated or published. Pricing changes are reported here.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct WebhookSubscriptionVersionUpdatedPayload {
    pub data: SubscriptionVersionPayload,
    /// When the event occurred, RFC 3339
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub timestamp: DateTime<FixedOffset>,
    /// Event type
    pub r#type: String,
}

impl WebhookSubscriptionVersionUpdatedPayload {
    pub fn builder() -> WebhookSubscriptionVersionUpdatedPayloadBuilder {
        <WebhookSubscriptionVersionUpdatedPayloadBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct WebhookSubscriptionVersionUpdatedPayloadBuilder {
    data: Option<SubscriptionVersionPayload>,
    timestamp: Option<DateTime<FixedOffset>>,
    r#type: Option<String>,
}

impl WebhookSubscriptionVersionUpdatedPayloadBuilder {
    pub fn data(mut self, value: SubscriptionVersionPayload) -> Self {
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

    /// Consumes the builder and constructs a [`WebhookSubscriptionVersionUpdatedPayload`].
    /// This method will fail if any of the following fields are not set:
    /// - [`data`](WebhookSubscriptionVersionUpdatedPayloadBuilder::data)
    /// - [`timestamp`](WebhookSubscriptionVersionUpdatedPayloadBuilder::timestamp)
    /// - [`r#type`](WebhookSubscriptionVersionUpdatedPayloadBuilder::r#type)
    pub fn build(self) -> Result<WebhookSubscriptionVersionUpdatedPayload, BuildError> {
        Ok(WebhookSubscriptionVersionUpdatedPayload {
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
