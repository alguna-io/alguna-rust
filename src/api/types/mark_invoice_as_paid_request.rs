pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct MarkInvoiceAsPaidRequest {
    /// Amount paid; defaults to the full invoice total
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount_paid: Option<String>,
    /// Free-text note about the payment
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_payment_note: Option<String>,
    /// Payment source
    pub external_payment_source: ExternalPaymentSource,
    /// Date of payment; defaults to the current date
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paid_date: Option<DateTime<FixedOffset>>,
}

impl MarkInvoiceAsPaidRequest {
    pub fn builder() -> MarkInvoiceAsPaidRequestBuilder {
        <MarkInvoiceAsPaidRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct MarkInvoiceAsPaidRequestBuilder {
    amount_paid: Option<String>,
    external_payment_note: Option<String>,
    external_payment_source: Option<ExternalPaymentSource>,
    paid_date: Option<DateTime<FixedOffset>>,
}

impl MarkInvoiceAsPaidRequestBuilder {
    pub fn amount_paid(mut self, value: impl Into<String>) -> Self {
        self.amount_paid = Some(value.into());
        self
    }

    pub fn external_payment_note(mut self, value: impl Into<String>) -> Self {
        self.external_payment_note = Some(value.into());
        self
    }

    pub fn external_payment_source(mut self, value: ExternalPaymentSource) -> Self {
        self.external_payment_source = Some(value);
        self
    }

    pub fn paid_date(mut self, value: DateTime<FixedOffset>) -> Self {
        self.paid_date = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`MarkInvoiceAsPaidRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`external_payment_source`](MarkInvoiceAsPaidRequestBuilder::external_payment_source)
    pub fn build(self) -> Result<MarkInvoiceAsPaidRequest, BuildError> {
        Ok(MarkInvoiceAsPaidRequest {
            amount_paid: self.amount_paid,
            external_payment_note: self.external_payment_note,
            external_payment_source: self
                .external_payment_source
                .ok_or_else(|| BuildError::missing_field("external_payment_source"))?,
            paid_date: self.paid_date,
        })
    }
}
