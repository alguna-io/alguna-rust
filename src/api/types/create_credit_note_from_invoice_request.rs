pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreateCreditNoteFromInvoiceRequest {
    /// Apply the credit note as part of creating it, synchronously, exactly as create followed by Apply Credit Note would. Defaults to false, leaving it a draft. If applying fails the credit note remains a draft and the error identifies it, so it can be retried via Apply Credit Note.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub apply: Option<bool>,
    /// Invoice lines to credit and how many units of each. Omit to credit everything still uncredited on the invoice.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line_items: Option<Vec<CreditInvoiceLineItemInput>>,
    /// Refund the customer as part of applying the credit note. Requires apply and an invoice that is paid; omit it to credit the customer without returning funds.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refund: Option<CreditNoteRefundRequest>,
}

impl CreateCreditNoteFromInvoiceRequest {
    pub fn builder() -> CreateCreditNoteFromInvoiceRequestBuilder {
        <CreateCreditNoteFromInvoiceRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateCreditNoteFromInvoiceRequestBuilder {
    apply: Option<bool>,
    line_items: Option<Vec<CreditInvoiceLineItemInput>>,
    refund: Option<CreditNoteRefundRequest>,
}

impl CreateCreditNoteFromInvoiceRequestBuilder {
    pub fn apply(mut self, value: bool) -> Self {
        self.apply = Some(value);
        self
    }

    pub fn line_items(mut self, value: Vec<CreditInvoiceLineItemInput>) -> Self {
        self.line_items = Some(value);
        self
    }

    pub fn refund(mut self, value: CreditNoteRefundRequest) -> Self {
        self.refund = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CreateCreditNoteFromInvoiceRequest`].
    pub fn build(self) -> Result<CreateCreditNoteFromInvoiceRequest, BuildError> {
        Ok(CreateCreditNoteFromInvoiceRequest {
            apply: self.apply,
            line_items: self.line_items,
            refund: self.refund,
        })
    }
}
