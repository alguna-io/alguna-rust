pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct InvoicePaymentRequest {
    /// Free-text note about the payment
    #[serde(skip_serializing_if = "Option::is_none")]
    pub note: Option<String>,
    /// Business date of the payment; defaults to now
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paid_at: Option<DateTime<FixedOffset>>,
    /// Your payment reference (e.g. PSP charge id)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference: Option<String>,
    /// How the customer paid
    pub source: InvoicePaymentRequestSource,
}

impl InvoicePaymentRequest {
    pub fn builder() -> InvoicePaymentRequestBuilder {
        <InvoicePaymentRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct InvoicePaymentRequestBuilder {
    note: Option<String>,
    paid_at: Option<DateTime<FixedOffset>>,
    reference: Option<String>,
    source: Option<InvoicePaymentRequestSource>,
}

impl InvoicePaymentRequestBuilder {
    pub fn note(mut self, value: impl Into<String>) -> Self {
        self.note = Some(value.into());
        self
    }

    pub fn paid_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.paid_at = Some(value);
        self
    }

    pub fn reference(mut self, value: impl Into<String>) -> Self {
        self.reference = Some(value.into());
        self
    }

    pub fn source(mut self, value: InvoicePaymentRequestSource) -> Self {
        self.source = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`InvoicePaymentRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`source`](InvoicePaymentRequestBuilder::source)
    pub fn build(self) -> Result<InvoicePaymentRequest, BuildError> {
        Ok(InvoicePaymentRequest {
            note: self.note,
            paid_at: self.paid_at,
            reference: self.reference,
            source: self
                .source
                .ok_or_else(|| BuildError::missing_field("source"))?,
        })
    }
}
