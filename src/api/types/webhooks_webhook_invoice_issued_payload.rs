pub use crate::prelude::*;

/// An invoice was finalized and issued to the customer.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct WebhookInvoiceIssuedPayload {
    pub data: InvoicePayload,
    /// When the event occurred, RFC 3339
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub timestamp: DateTime<FixedOffset>,
    /// Event type
    pub r#type: String,
}

impl WebhookInvoiceIssuedPayload {
    pub fn builder() -> WebhookInvoiceIssuedPayloadBuilder {
        <WebhookInvoiceIssuedPayloadBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct WebhookInvoiceIssuedPayloadBuilder {
    data: Option<InvoicePayload>,
    timestamp: Option<DateTime<FixedOffset>>,
    r#type: Option<String>,
}

impl WebhookInvoiceIssuedPayloadBuilder {
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

    /// Consumes the builder and constructs a [`WebhookInvoiceIssuedPayload`].
    /// This method will fail if any of the following fields are not set:
    /// - [`data`](WebhookInvoiceIssuedPayloadBuilder::data)
    /// - [`timestamp`](WebhookInvoiceIssuedPayloadBuilder::timestamp)
    /// - [`r#type`](WebhookInvoiceIssuedPayloadBuilder::r#type)
    pub fn build(self) -> Result<WebhookInvoiceIssuedPayload, BuildError> {
        Ok(WebhookInvoiceIssuedPayload {
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
