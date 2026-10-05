pub use crate::prelude::*;

/// A line item was removed from an invoice.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct WebhookLineitemDeletedPayload {
    pub data: LineItemPayload,
    /// When the event occurred, RFC 3339
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub timestamp: DateTime<FixedOffset>,
    /// Event type
    pub r#type: String,
}

impl WebhookLineitemDeletedPayload {
    pub fn builder() -> WebhookLineitemDeletedPayloadBuilder {
        <WebhookLineitemDeletedPayloadBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct WebhookLineitemDeletedPayloadBuilder {
    data: Option<LineItemPayload>,
    timestamp: Option<DateTime<FixedOffset>>,
    r#type: Option<String>,
}

impl WebhookLineitemDeletedPayloadBuilder {
    pub fn data(mut self, value: LineItemPayload) -> Self {
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

    /// Consumes the builder and constructs a [`WebhookLineitemDeletedPayload`].
    /// This method will fail if any of the following fields are not set:
    /// - [`data`](WebhookLineitemDeletedPayloadBuilder::data)
    /// - [`timestamp`](WebhookLineitemDeletedPayloadBuilder::timestamp)
    /// - [`r#type`](WebhookLineitemDeletedPayloadBuilder::r#type)
    pub fn build(self) -> Result<WebhookLineitemDeletedPayload, BuildError> {
        Ok(WebhookLineitemDeletedPayload {
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
