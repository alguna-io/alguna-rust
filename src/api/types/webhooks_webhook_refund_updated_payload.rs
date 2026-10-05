pub use crate::prelude::*;

/// A refund changed status.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct WebhookRefundUpdatedPayload {
    pub data: RefundPayload,
    /// When the event occurred, RFC 3339
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub timestamp: DateTime<FixedOffset>,
    /// Event type
    pub r#type: String,
}

impl WebhookRefundUpdatedPayload {
    pub fn builder() -> WebhookRefundUpdatedPayloadBuilder {
        <WebhookRefundUpdatedPayloadBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct WebhookRefundUpdatedPayloadBuilder {
    data: Option<RefundPayload>,
    timestamp: Option<DateTime<FixedOffset>>,
    r#type: Option<String>,
}

impl WebhookRefundUpdatedPayloadBuilder {
    pub fn data(mut self, value: RefundPayload) -> Self {
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

    /// Consumes the builder and constructs a [`WebhookRefundUpdatedPayload`].
    /// This method will fail if any of the following fields are not set:
    /// - [`data`](WebhookRefundUpdatedPayloadBuilder::data)
    /// - [`timestamp`](WebhookRefundUpdatedPayloadBuilder::timestamp)
    /// - [`r#type`](WebhookRefundUpdatedPayloadBuilder::r#type)
    pub fn build(self) -> Result<WebhookRefundUpdatedPayload, BuildError> {
        Ok(WebhookRefundUpdatedPayload {
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
