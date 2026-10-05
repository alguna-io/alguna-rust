pub use crate::prelude::*;

/// An issued invoice passed its due date without being paid in full.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct WebhookInvoiceOverduePayload {
    pub data: InvoicePayload,
    /// When the event occurred, RFC 3339
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub timestamp: DateTime<FixedOffset>,
    /// Event type
    pub r#type: String,
}

impl WebhookInvoiceOverduePayload {
    pub fn builder() -> WebhookInvoiceOverduePayloadBuilder {
        <WebhookInvoiceOverduePayloadBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct WebhookInvoiceOverduePayloadBuilder {
    data: Option<InvoicePayload>,
    timestamp: Option<DateTime<FixedOffset>>,
    r#type: Option<String>,
}

impl WebhookInvoiceOverduePayloadBuilder {
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

    /// Consumes the builder and constructs a [`WebhookInvoiceOverduePayload`].
    /// This method will fail if any of the following fields are not set:
    /// - [`data`](WebhookInvoiceOverduePayloadBuilder::data)
    /// - [`timestamp`](WebhookInvoiceOverduePayloadBuilder::timestamp)
    /// - [`r#type`](WebhookInvoiceOverduePayloadBuilder::r#type)
    pub fn build(self) -> Result<WebhookInvoiceOverduePayload, BuildError> {
        Ok(WebhookInvoiceOverduePayload {
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
