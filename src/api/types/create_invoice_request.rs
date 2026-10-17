pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreateInvoiceRequest {
    /// End of the invoice's billing period, exclusive: for a period through 31 January send 2026-02-01T00:00:00Z. Send with billing_period_start.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_period_end: Option<DateTime<FixedOffset>>,
    /// Start of the invoice's billing period, inclusive. Send with billing_period_end.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_period_start: Option<DateTime<FixedOffset>>,
    /// ISO 4217 currency code; required unless tax_calculation_id is set
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    /// Existing customer to bill. On the tax_calculation_id path, leave empty to mint a guest customer from the calculation's buyer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_id: Option<String>,
    /// Invoice description or memo
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Date to issue the invoice
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issue_date: Option<DateTime<FixedOffset>>,
    /// Line items to include on the invoice
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line_items: Option<Vec<CreateInvoiceLineItemRequest>>,
    /// Existing sub-merchant the invoice is issued for; you remain the seller of record. Create the merchant before referencing it here. Optional alongside tax_calculation_id, where it must match the merchant the calculation was made for. Requires the merchants entitlement
    #[serde(skip_serializing_if = "Option::is_none")]
    pub merchant_id: Option<String>,
    /// Record the customer's already-settled payment and mark the invoice paid in the same call; requires tax_calculation_id
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment: Option<InvoicePaymentRequest>,
    /// Purchase order number
    #[serde(skip_serializing_if = "Option::is_none")]
    pub purchase_order_number: Option<String>,
    /// Initial invoice status. Defaults to draft. Setting issued releases the invoice for issuance, which completes asynchronously: the response returns the invoice in processing, and it settles to issued shortly after — poll the Get Invoice endpoint to observe the final status. paid is only valid on the tax_calculation_id path and must be sent together with a payment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<CreateInvoiceRequestStatus>,
    /// Id of a persisted tax calculation; its stored line items, tax amounts, and buyer become the invoice, which is issued synchronously with no recalculation. Mutually exclusive with line_items (the calculation supplies them). Leave status unset or set it to issued to issue the invoice; set status to paid together with a payment to also mark it paid.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_calculation_id: Option<String>,
}

impl CreateInvoiceRequest {
    pub fn builder() -> CreateInvoiceRequestBuilder {
        <CreateInvoiceRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateInvoiceRequestBuilder {
    billing_period_end: Option<DateTime<FixedOffset>>,
    billing_period_start: Option<DateTime<FixedOffset>>,
    currency: Option<String>,
    customer_id: Option<String>,
    description: Option<String>,
    issue_date: Option<DateTime<FixedOffset>>,
    line_items: Option<Vec<CreateInvoiceLineItemRequest>>,
    merchant_id: Option<String>,
    payment: Option<InvoicePaymentRequest>,
    purchase_order_number: Option<String>,
    status: Option<CreateInvoiceRequestStatus>,
    tax_calculation_id: Option<String>,
}

impl CreateInvoiceRequestBuilder {
    pub fn billing_period_end(mut self, value: DateTime<FixedOffset>) -> Self {
        self.billing_period_end = Some(value);
        self
    }

    pub fn billing_period_start(mut self, value: DateTime<FixedOffset>) -> Self {
        self.billing_period_start = Some(value);
        self
    }

    pub fn currency(mut self, value: impl Into<String>) -> Self {
        self.currency = Some(value.into());
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

    pub fn issue_date(mut self, value: DateTime<FixedOffset>) -> Self {
        self.issue_date = Some(value);
        self
    }

    pub fn line_items(mut self, value: Vec<CreateInvoiceLineItemRequest>) -> Self {
        self.line_items = Some(value);
        self
    }

    pub fn merchant_id(mut self, value: impl Into<String>) -> Self {
        self.merchant_id = Some(value.into());
        self
    }

    pub fn payment(mut self, value: InvoicePaymentRequest) -> Self {
        self.payment = Some(value);
        self
    }

    pub fn purchase_order_number(mut self, value: impl Into<String>) -> Self {
        self.purchase_order_number = Some(value.into());
        self
    }

    pub fn status(mut self, value: CreateInvoiceRequestStatus) -> Self {
        self.status = Some(value);
        self
    }

    pub fn tax_calculation_id(mut self, value: impl Into<String>) -> Self {
        self.tax_calculation_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CreateInvoiceRequest`].
    pub fn build(self) -> Result<CreateInvoiceRequest, BuildError> {
        Ok(CreateInvoiceRequest {
            billing_period_end: self.billing_period_end,
            billing_period_start: self.billing_period_start,
            currency: self.currency,
            customer_id: self.customer_id,
            description: self.description,
            issue_date: self.issue_date,
            line_items: self.line_items,
            merchant_id: self.merchant_id,
            payment: self.payment,
            purchase_order_number: self.purchase_order_number,
            status: self.status,
            tax_calculation_id: self.tax_calculation_id,
        })
    }
}
