pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreateCreditNoteRequest {
    /// Apply the credit note as part of creating it, synchronously, exactly as create followed by Apply Credit Note would. Defaults to false, leaving it a draft. If applying fails the credit note remains a draft and the error identifies it, so it can be retried via Apply Credit Note.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub apply: Option<bool>,
    /// End of the billing period
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_period_end: Option<DateTime<FixedOffset>>,
    /// Start of the billing period
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_period_start: Option<DateTime<FixedOffset>>,
    /// Customer the credit note is issued to. Required unless invoice_id is given, in which case it is taken from the invoice; when both are given they must agree.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_id: Option<String>,
    /// Credit note description
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Linked invoice. Required unless customer_id is given. Pre-payment credit notes are supported only for Alguna-managed invoices in issued or overdue
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoice_id: Option<String>,
    /// Line items on the credit note
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line_items: Option<Vec<CreateCreditNoteLineItemInput>>,
    /// Refund the customer as part of applying the credit note. Requires apply and a linked invoice that is paid; omit it to credit the customer without returning funds.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refund: Option<CreditNoteRefundRequest>,
}

impl CreateCreditNoteRequest {
    pub fn builder() -> CreateCreditNoteRequestBuilder {
        <CreateCreditNoteRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateCreditNoteRequestBuilder {
    apply: Option<bool>,
    billing_period_end: Option<DateTime<FixedOffset>>,
    billing_period_start: Option<DateTime<FixedOffset>>,
    customer_id: Option<String>,
    description: Option<String>,
    invoice_id: Option<String>,
    line_items: Option<Vec<CreateCreditNoteLineItemInput>>,
    refund: Option<CreditNoteRefundRequest>,
}

impl CreateCreditNoteRequestBuilder {
    pub fn apply(mut self, value: bool) -> Self {
        self.apply = Some(value);
        self
    }

    pub fn billing_period_end(mut self, value: DateTime<FixedOffset>) -> Self {
        self.billing_period_end = Some(value);
        self
    }

    pub fn billing_period_start(mut self, value: DateTime<FixedOffset>) -> Self {
        self.billing_period_start = Some(value);
        self
    }

    pub fn customer_id(mut self, value: impl Into<String>) -> Self {
        self.customer_id = Some(value.into());
        self
    }

    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
        self
    }

    pub fn invoice_id(mut self, value: impl Into<String>) -> Self {
        self.invoice_id = Some(value.into());
        self
    }

    pub fn line_items(mut self, value: Vec<CreateCreditNoteLineItemInput>) -> Self {
        self.line_items = Some(value);
        self
    }

    pub fn refund(mut self, value: CreditNoteRefundRequest) -> Self {
        self.refund = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CreateCreditNoteRequest`].
    pub fn build(self) -> Result<CreateCreditNoteRequest, BuildError> {
        Ok(CreateCreditNoteRequest {
            apply: self.apply,
            billing_period_end: self.billing_period_end,
            billing_period_start: self.billing_period_start,
            customer_id: self.customer_id,
            description: self.description,
            invoice_id: self.invoice_id,
            line_items: self.line_items,
            refund: self.refund,
        })
    }
}
