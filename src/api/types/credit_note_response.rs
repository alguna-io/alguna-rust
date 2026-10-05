pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct CreditNoteResponse {
    /// Credit note ID in the accounting system
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accounting_credit_note_id: Option<String>,
    /// URL to the credit note in the accounting system
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accounting_credit_note_url: Option<String>,
    /// Accounting integration ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accounting_integration_id: Option<String>,
    /// Accounting provider name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accounting_provider: Option<String>,
    /// Date the credit note was applied
    #[serde(skip_serializing_if = "Option::is_none")]
    pub apply_date: Option<DateTime<FixedOffset>>,
    /// Attachments on this credit note
    pub attachments: Vec<CreditNoteAttachmentResponse>,
    /// End of the billing period
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_period_end: Option<DateTime<FixedOffset>>,
    /// Start of the billing period
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_period_start: Option<DateTime<FixedOffset>>,
    /// Timestamp when the credit note was created
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub created_at: DateTime<FixedOffset>,
    /// Date of the credit
    #[serde(skip_serializing_if = "Option::is_none")]
    pub credit_date: Option<DateTime<FixedOffset>>,
    /// Currency code
    pub currency: String,
    /// Customer this credit note belongs to
    pub customer_id: String,
    /// Description of the credit note
    pub description: String,
    /// External reference identifier
    pub external_reference: String,
    /// Unique identifier for the credit note
    pub id: String,
    /// Associated invoice ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoice_id: Option<String>,
    /// Whether a PDF is available for download
    pub is_pdf_available: bool,
    /// Date of the last refund
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_refund_date: Option<DateTime<FixedOffset>>,
    /// ID of the last refund
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_refund_id: Option<String>,
    /// Legal entity ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub legal_entity_id: Option<String>,
    /// Line items on this credit note
    pub line_items: Vec<CreditNoteLineItemResponse>,
    /// Sub-merchant the credited invoice was issued for; inherited from that invoice
    #[serde(skip_serializing_if = "Option::is_none")]
    pub merchant_id: Option<String>,
    /// Purchase order number
    #[serde(skip_serializing_if = "Option::is_none")]
    pub purchase_order_number: Option<String>,
    /// Current status of the credit note
    pub status: CreditNoteStatus,
    /// Subtotal before tax
    pub subtotal: String,
    /// Whether accounting sync is enabled
    pub sync_enabled: bool,
    /// Tax amount
    pub tax: String,
    /// Total amount including tax
    pub total: String,
    /// Timestamp when the credit note was last updated
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub updated_at: DateTime<FixedOffset>,
    /// Date the credit note was voided
    #[serde(skip_serializing_if = "Option::is_none")]
    pub void_date: Option<DateTime<FixedOffset>>,
}

impl CreditNoteResponse {
    pub fn builder() -> CreditNoteResponseBuilder {
        <CreditNoteResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreditNoteResponseBuilder {
    accounting_credit_note_id: Option<String>,
    accounting_credit_note_url: Option<String>,
    accounting_integration_id: Option<String>,
    accounting_provider: Option<String>,
    apply_date: Option<DateTime<FixedOffset>>,
    attachments: Option<Vec<CreditNoteAttachmentResponse>>,
    billing_period_end: Option<DateTime<FixedOffset>>,
    billing_period_start: Option<DateTime<FixedOffset>>,
    created_at: Option<DateTime<FixedOffset>>,
    credit_date: Option<DateTime<FixedOffset>>,
    currency: Option<String>,
    customer_id: Option<String>,
    description: Option<String>,
    external_reference: Option<String>,
    id: Option<String>,
    invoice_id: Option<String>,
    is_pdf_available: Option<bool>,
    last_refund_date: Option<DateTime<FixedOffset>>,
    last_refund_id: Option<String>,
    legal_entity_id: Option<String>,
    line_items: Option<Vec<CreditNoteLineItemResponse>>,
    merchant_id: Option<String>,
    purchase_order_number: Option<String>,
    status: Option<CreditNoteStatus>,
    subtotal: Option<String>,
    sync_enabled: Option<bool>,
    tax: Option<String>,
    total: Option<String>,
    updated_at: Option<DateTime<FixedOffset>>,
    void_date: Option<DateTime<FixedOffset>>,
}

impl CreditNoteResponseBuilder {
    pub fn accounting_credit_note_id(mut self, value: impl Into<String>) -> Self {
        self.accounting_credit_note_id = Some(value.into());
        self
    }

    pub fn accounting_credit_note_url(mut self, value: impl Into<String>) -> Self {
        self.accounting_credit_note_url = Some(value.into());
        self
    }

    pub fn accounting_integration_id(mut self, value: impl Into<String>) -> Self {
        self.accounting_integration_id = Some(value.into());
        self
    }

    pub fn accounting_provider(mut self, value: impl Into<String>) -> Self {
        self.accounting_provider = Some(value.into());
        self
    }

    pub fn apply_date(mut self, value: DateTime<FixedOffset>) -> Self {
        self.apply_date = Some(value);
        self
    }

    pub fn attachments(mut self, value: Vec<CreditNoteAttachmentResponse>) -> Self {
        self.attachments = Some(value);
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

    pub fn credit_date(mut self, value: DateTime<FixedOffset>) -> Self {
        self.credit_date = Some(value);
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

    pub fn external_reference(mut self, value: impl Into<String>) -> Self {
        self.external_reference = Some(value.into());
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn invoice_id(mut self, value: impl Into<String>) -> Self {
        self.invoice_id = Some(value.into());
        self
    }

    pub fn is_pdf_available(mut self, value: bool) -> Self {
        self.is_pdf_available = Some(value);
        self
    }

    pub fn last_refund_date(mut self, value: DateTime<FixedOffset>) -> Self {
        self.last_refund_date = Some(value);
        self
    }

    pub fn last_refund_id(mut self, value: impl Into<String>) -> Self {
        self.last_refund_id = Some(value.into());
        self
    }

    pub fn legal_entity_id(mut self, value: impl Into<String>) -> Self {
        self.legal_entity_id = Some(value.into());
        self
    }

    pub fn line_items(mut self, value: Vec<CreditNoteLineItemResponse>) -> Self {
        self.line_items = Some(value);
        self
    }

    pub fn merchant_id(mut self, value: impl Into<String>) -> Self {
        self.merchant_id = Some(value.into());
        self
    }

    pub fn purchase_order_number(mut self, value: impl Into<String>) -> Self {
        self.purchase_order_number = Some(value.into());
        self
    }

    pub fn status(mut self, value: CreditNoteStatus) -> Self {
        self.status = Some(value);
        self
    }

    pub fn subtotal(mut self, value: impl Into<String>) -> Self {
        self.subtotal = Some(value.into());
        self
    }

    pub fn sync_enabled(mut self, value: bool) -> Self {
        self.sync_enabled = Some(value);
        self
    }

    pub fn tax(mut self, value: impl Into<String>) -> Self {
        self.tax = Some(value.into());
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

    pub fn void_date(mut self, value: DateTime<FixedOffset>) -> Self {
        self.void_date = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CreditNoteResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`attachments`](CreditNoteResponseBuilder::attachments)
    /// - [`created_at`](CreditNoteResponseBuilder::created_at)
    /// - [`currency`](CreditNoteResponseBuilder::currency)
    /// - [`customer_id`](CreditNoteResponseBuilder::customer_id)
    /// - [`description`](CreditNoteResponseBuilder::description)
    /// - [`external_reference`](CreditNoteResponseBuilder::external_reference)
    /// - [`id`](CreditNoteResponseBuilder::id)
    /// - [`is_pdf_available`](CreditNoteResponseBuilder::is_pdf_available)
    /// - [`line_items`](CreditNoteResponseBuilder::line_items)
    /// - [`status`](CreditNoteResponseBuilder::status)
    /// - [`subtotal`](CreditNoteResponseBuilder::subtotal)
    /// - [`sync_enabled`](CreditNoteResponseBuilder::sync_enabled)
    /// - [`tax`](CreditNoteResponseBuilder::tax)
    /// - [`total`](CreditNoteResponseBuilder::total)
    /// - [`updated_at`](CreditNoteResponseBuilder::updated_at)
    pub fn build(self) -> Result<CreditNoteResponse, BuildError> {
        Ok(CreditNoteResponse {
            accounting_credit_note_id: self.accounting_credit_note_id,
            accounting_credit_note_url: self.accounting_credit_note_url,
            accounting_integration_id: self.accounting_integration_id,
            accounting_provider: self.accounting_provider,
            apply_date: self.apply_date,
            attachments: self
                .attachments
                .ok_or_else(|| BuildError::missing_field("attachments"))?,
            billing_period_end: self.billing_period_end,
            billing_period_start: self.billing_period_start,
            created_at: self
                .created_at
                .ok_or_else(|| BuildError::missing_field("created_at"))?,
            credit_date: self.credit_date,
            currency: self
                .currency
                .ok_or_else(|| BuildError::missing_field("currency"))?,
            customer_id: self
                .customer_id
                .ok_or_else(|| BuildError::missing_field("customer_id"))?,
            description: self
                .description
                .ok_or_else(|| BuildError::missing_field("description"))?,
            external_reference: self
                .external_reference
                .ok_or_else(|| BuildError::missing_field("external_reference"))?,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            invoice_id: self.invoice_id,
            is_pdf_available: self
                .is_pdf_available
                .ok_or_else(|| BuildError::missing_field("is_pdf_available"))?,
            last_refund_date: self.last_refund_date,
            last_refund_id: self.last_refund_id,
            legal_entity_id: self.legal_entity_id,
            line_items: self
                .line_items
                .ok_or_else(|| BuildError::missing_field("line_items"))?,
            merchant_id: self.merchant_id,
            purchase_order_number: self.purchase_order_number,
            status: self
                .status
                .ok_or_else(|| BuildError::missing_field("status"))?,
            subtotal: self
                .subtotal
                .ok_or_else(|| BuildError::missing_field("subtotal"))?,
            sync_enabled: self
                .sync_enabled
                .ok_or_else(|| BuildError::missing_field("sync_enabled"))?,
            tax: self.tax.ok_or_else(|| BuildError::missing_field("tax"))?,
            total: self
                .total
                .ok_or_else(|| BuildError::missing_field("total"))?,
            updated_at: self
                .updated_at
                .ok_or_else(|| BuildError::missing_field("updated_at"))?,
            void_date: self.void_date,
        })
    }
}
