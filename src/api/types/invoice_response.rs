pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct InvoiceResponse {
    /// Amount that has been paid
    pub amount_paid: String,
    /// Remaining amount due
    pub amount_remaining: String,
    /// Formatted billing period string
    pub billing_period: String,
    /// End of the billing period
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_period_end: Option<DateTime<FixedOffset>>,
    /// Start of the billing period
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_period_start: Option<DateTime<FixedOffset>>,
    /// Timestamp when the invoice was created
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub created_at: DateTime<FixedOffset>,
    /// ISO 4217 currency code
    pub currency: String,
    /// Customer this invoice belongs to
    pub customer_id: String,
    /// Invoice description or memo
    pub description: String,
    /// Date the invoice payment is due
    #[serde(skip_serializing_if = "Option::is_none")]
    pub due_date: Option<DateTime<FixedOffset>>,
    /// Identifier in the external payment provider
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_id: Option<String>,
    /// External payment provider name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_provider: Option<String>,
    /// Customer-facing invoice reference number
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_reference: Option<String>,
    /// URL to the invoice in the external system
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_url: Option<String>,
    /// Unique identifier for the invoice
    pub id: String,
    /// Date the invoice is scheduled for processing
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoicing_date: Option<DateTime<FixedOffset>>,
    /// Date the invoice was issued
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issue_date: Option<DateTime<FixedOffset>>,
    /// Line items on this invoice
    pub line_items: Vec<LineItemResponse>,
    /// Sub-merchant the invoice was issued for; you remain the seller of record
    #[serde(skip_serializing_if = "Option::is_none")]
    pub merchant_id: Option<String>,
    /// Date the invoice was paid
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paid_date: Option<DateTime<FixedOffset>>,
    /// Purchase order number
    #[serde(skip_serializing_if = "Option::is_none")]
    pub purchase_order_number: Option<String>,
    /// Current status of the invoice
    pub status: InvoiceStatus,
    /// Sum of all line item totals before tax
    pub subtotal: String,
    /// Total tax amount
    pub tax: String,
    /// Id of the tax calculation this invoice was created from, if any
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_calculation_id: Option<String>,
    /// Total amount including tax
    pub total: String,
    /// Timestamp when the invoice was last updated
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub updated_at: DateTime<FixedOffset>,
}

impl InvoiceResponse {
    pub fn builder() -> InvoiceResponseBuilder {
        <InvoiceResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct InvoiceResponseBuilder {
    amount_paid: Option<String>,
    amount_remaining: Option<String>,
    billing_period: Option<String>,
    billing_period_end: Option<DateTime<FixedOffset>>,
    billing_period_start: Option<DateTime<FixedOffset>>,
    created_at: Option<DateTime<FixedOffset>>,
    currency: Option<String>,
    customer_id: Option<String>,
    description: Option<String>,
    due_date: Option<DateTime<FixedOffset>>,
    external_id: Option<String>,
    external_provider: Option<String>,
    external_reference: Option<String>,
    external_url: Option<String>,
    id: Option<String>,
    invoicing_date: Option<DateTime<FixedOffset>>,
    issue_date: Option<DateTime<FixedOffset>>,
    line_items: Option<Vec<LineItemResponse>>,
    merchant_id: Option<String>,
    paid_date: Option<DateTime<FixedOffset>>,
    purchase_order_number: Option<String>,
    status: Option<InvoiceStatus>,
    subtotal: Option<String>,
    tax: Option<String>,
    tax_calculation_id: Option<String>,
    total: Option<String>,
    updated_at: Option<DateTime<FixedOffset>>,
}

impl InvoiceResponseBuilder {
    pub fn amount_paid(mut self, value: impl Into<String>) -> Self {
        self.amount_paid = Some(value.into());
        self
    }

    pub fn amount_remaining(mut self, value: impl Into<String>) -> Self {
        self.amount_remaining = Some(value.into());
        self
    }

    pub fn billing_period(mut self, value: impl Into<String>) -> Self {
        self.billing_period = Some(value.into());
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

    pub fn created_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.created_at = Some(value);
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

    pub fn due_date(mut self, value: DateTime<FixedOffset>) -> Self {
        self.due_date = Some(value);
        self
    }

    pub fn external_id(mut self, value: impl Into<String>) -> Self {
        self.external_id = Some(value.into());
        self
    }

    pub fn external_provider(mut self, value: impl Into<String>) -> Self {
        self.external_provider = Some(value.into());
        self
    }

    pub fn external_reference(mut self, value: impl Into<String>) -> Self {
        self.external_reference = Some(value.into());
        self
    }

    pub fn external_url(mut self, value: impl Into<String>) -> Self {
        self.external_url = Some(value.into());
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn invoicing_date(mut self, value: DateTime<FixedOffset>) -> Self {
        self.invoicing_date = Some(value);
        self
    }

    pub fn issue_date(mut self, value: DateTime<FixedOffset>) -> Self {
        self.issue_date = Some(value);
        self
    }

    pub fn line_items(mut self, value: Vec<LineItemResponse>) -> Self {
        self.line_items = Some(value);
        self
    }

    pub fn merchant_id(mut self, value: impl Into<String>) -> Self {
        self.merchant_id = Some(value.into());
        self
    }

    pub fn paid_date(mut self, value: DateTime<FixedOffset>) -> Self {
        self.paid_date = Some(value);
        self
    }

    pub fn purchase_order_number(mut self, value: impl Into<String>) -> Self {
        self.purchase_order_number = Some(value.into());
        self
    }

    pub fn status(mut self, value: InvoiceStatus) -> Self {
        self.status = Some(value);
        self
    }

    pub fn subtotal(mut self, value: impl Into<String>) -> Self {
        self.subtotal = Some(value.into());
        self
    }

    pub fn tax(mut self, value: impl Into<String>) -> Self {
        self.tax = Some(value.into());
        self
    }

    pub fn tax_calculation_id(mut self, value: impl Into<String>) -> Self {
        self.tax_calculation_id = Some(value.into());
        self
    }

    pub fn total(mut self, value: impl Into<String>) -> Self {
        self.total = Some(value.into());
        self
    }

    pub fn updated_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.updated_at = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`InvoiceResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`amount_paid`](InvoiceResponseBuilder::amount_paid)
    /// - [`amount_remaining`](InvoiceResponseBuilder::amount_remaining)
    /// - [`billing_period`](InvoiceResponseBuilder::billing_period)
    /// - [`created_at`](InvoiceResponseBuilder::created_at)
    /// - [`currency`](InvoiceResponseBuilder::currency)
    /// - [`customer_id`](InvoiceResponseBuilder::customer_id)
    /// - [`description`](InvoiceResponseBuilder::description)
    /// - [`id`](InvoiceResponseBuilder::id)
    /// - [`line_items`](InvoiceResponseBuilder::line_items)
    /// - [`status`](InvoiceResponseBuilder::status)
    /// - [`subtotal`](InvoiceResponseBuilder::subtotal)
    /// - [`tax`](InvoiceResponseBuilder::tax)
    /// - [`total`](InvoiceResponseBuilder::total)
    /// - [`updated_at`](InvoiceResponseBuilder::updated_at)
    pub fn build(self) -> Result<InvoiceResponse, BuildError> {
        Ok(InvoiceResponse {
            amount_paid: self
                .amount_paid
                .ok_or_else(|| BuildError::missing_field("amount_paid"))?,
            amount_remaining: self
                .amount_remaining
                .ok_or_else(|| BuildError::missing_field("amount_remaining"))?,
            billing_period: self
                .billing_period
                .ok_or_else(|| BuildError::missing_field("billing_period"))?,
            billing_period_end: self.billing_period_end,
            billing_period_start: self.billing_period_start,
            created_at: self
                .created_at
                .ok_or_else(|| BuildError::missing_field("created_at"))?,
            currency: self
                .currency
                .ok_or_else(|| BuildError::missing_field("currency"))?,
            customer_id: self
                .customer_id
                .ok_or_else(|| BuildError::missing_field("customer_id"))?,
            description: self
                .description
                .ok_or_else(|| BuildError::missing_field("description"))?,
            due_date: self.due_date,
            external_id: self.external_id,
            external_provider: self.external_provider,
            external_reference: self.external_reference,
            external_url: self.external_url,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            invoicing_date: self.invoicing_date,
            issue_date: self.issue_date,
            line_items: self
                .line_items
                .ok_or_else(|| BuildError::missing_field("line_items"))?,
            merchant_id: self.merchant_id,
            paid_date: self.paid_date,
            purchase_order_number: self.purchase_order_number,
            status: self
                .status
                .ok_or_else(|| BuildError::missing_field("status"))?,
            subtotal: self
                .subtotal
                .ok_or_else(|| BuildError::missing_field("subtotal"))?,
            tax: self.tax.ok_or_else(|| BuildError::missing_field("tax"))?,
            tax_calculation_id: self.tax_calculation_id,
            total: self
                .total
                .ok_or_else(|| BuildError::missing_field("total"))?,
            updated_at: self
                .updated_at
                .ok_or_else(|| BuildError::missing_field("updated_at"))?,
        })
    }
}
