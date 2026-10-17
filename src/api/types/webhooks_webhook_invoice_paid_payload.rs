pub use crate::prelude::*;

/// An invoice was paid in full.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct WebhookInvoicePaidPayload {
    pub data: InvoicePayload,
    /// When the event occurred, RFC 3339
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub timestamp: DateTime<FixedOffset>,
    /// Event type
    pub r#type: String,
}

impl WebhookInvoicePaidPayload {
    pub fn builder() -> WebhookInvoicePaidPayloadBuilder {
        <WebhookInvoicePaidPayloadBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct WebhookInvoicePaidPayloadBuilder {
    data: Option<InvoicePayload>,
    timestamp: Option<DateTime<FixedOffset>>,
    r#type: Option<String>,
}

impl WebhookInvoicePaidPayloadBuilder {
    pub fn data(mut self, value: InvoicePayload) -> Self {
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

    /// Consumes the builder and constructs a [`WebhookInvoicePaidPayload`].
    /// This method will fail if any of the following fields are not set:
    /// - [`data`](WebhookInvoicePaidPayloadBuilder::data)
    /// - [`timestamp`](WebhookInvoicePaidPayloadBuilder::timestamp)
    /// - [`r#type`](WebhookInvoicePaidPayloadBuilder::r#type)
    pub fn build(self) -> Result<WebhookInvoicePaidPayload, BuildError> {
        Ok(WebhookInvoicePaidPayload {
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
