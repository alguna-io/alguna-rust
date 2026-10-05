pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct RefundPayload {
    /// Customer account the refund belongs to
    #[serde(rename = "accountId")]
    pub account_id: String,
    /// Invoices this refund was attributed to, and how much came off each
    pub allocations: Vec<PaymentAllocationPayload>,
    /// Refund amount
    pub amount: String,
    /// When the refund was created, RFC 3339
    #[serde(rename = "createdAt")]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub created_at: DateTime<FixedOffset>,
    /// Credit note raised for this refund, when one was
    #[serde(rename = "creditNoteId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub credit_note_id: Option<String>,
    /// ISO 4217 currency code
    pub currency: String,
    /// The payment processor's identifier for the refund
    #[serde(rename = "externalId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_id: Option<String>,
    /// Why the refund failed. Null unless status is failed.
    #[serde(rename = "failureReason")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_reason: Option<String>,
    /// Unique identifier for the refund
    pub id: String,
    /// Deprecated. Populated only when the refunded payment covered a single invoice; null otherwise. Read allocations instead.
    #[serde(rename = "invoiceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoice_id: Option<String>,
    /// Status before this change. Null for the first event on a refund.
    #[serde(rename = "prevStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prev_status: Option<RefundStatus>,
    /// Identifier of the processor account that handled the refund
    #[serde(rename = "processorId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub processor_id: Option<String>,
    /// Payment provider that processed the refund
    #[serde(rename = "processorName")]
    pub processor_name: PaymentProvider,
    /// Refund status
    pub status: RefundStatus,
    /// When the refund was last updated, RFC 3339
    #[serde(rename = "updatedAt")]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub updated_at: DateTime<FixedOffset>,
}

impl RefundPayload {
    pub fn builder() -> RefundPayloadBuilder {
        <RefundPayloadBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RefundPayloadBuilder {
    account_id: Option<String>,
    allocations: Option<Vec<PaymentAllocationPayload>>,
    amount: Option<String>,
    created_at: Option<DateTime<FixedOffset>>,
    credit_note_id: Option<String>,
    currency: Option<String>,
    external_id: Option<String>,
    failure_reason: Option<String>,
    id: Option<String>,
    invoice_id: Option<String>,
    prev_status: Option<RefundStatus>,
    processor_id: Option<String>,
    processor_name: Option<PaymentProvider>,
    status: Option<RefundStatus>,
    updated_at: Option<DateTime<FixedOffset>>,
}

impl RefundPayloadBuilder {
    pub fn account_id(mut self, value: impl Into<String>) -> Self {
        self.account_id = Some(value.into());
        self
    }

    pub fn allocations(mut self, value: Vec<PaymentAllocationPayload>) -> Self {
        self.allocations = Some(value);
        self
    }

    pub fn amount(mut self, value: impl Into<String>) -> Self {
        self.amount = Some(value.into());
        self
    }

    pub fn created_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.created_at = Some(value);
        self
    }

    pub fn credit_note_id(mut self, value: impl Into<String>) -> Self {
        self.credit_note_id = Some(value.into());
        self
    }

    pub fn currency(mut self, value: impl Into<String>) -> Self {
        self.currency = Some(value.into());
        self
    }

    pub fn external_id(mut self, value: impl Into<String>) -> Self {
        self.external_id = Some(value.into());
        self
    }

    pub fn failure_reason(mut self, value: impl Into<String>) -> Self {
        self.failure_reason = Some(value.into());
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

    pub fn prev_status(mut self, value: RefundStatus) -> Self {
        self.prev_status = Some(value);
        self
    }

    pub fn processor_id(mut self, value: impl Into<String>) -> Self {
        self.processor_id = Some(value.into());
        self
    }

    pub fn processor_name(mut self, value: PaymentProvider) -> Self {
        self.processor_name = Some(value);
        self
    }

    pub fn status(mut self, value: RefundStatus) -> Self {
        self.status = Some(value);
        self
    }

    pub fn updated_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.updated_at = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`RefundPayload`].
    /// This method will fail if any of the following fields are not set:
    /// - [`account_id`](RefundPayloadBuilder::account_id)
    /// - [`allocations`](RefundPayloadBuilder::allocations)
    /// - [`amount`](RefundPayloadBuilder::amount)
    /// - [`created_at`](RefundPayloadBuilder::created_at)
    /// - [`currency`](RefundPayloadBuilder::currency)
    /// - [`id`](RefundPayloadBuilder::id)
    /// - [`processor_name`](RefundPayloadBuilder::processor_name)
    /// - [`status`](RefundPayloadBuilder::status)
    /// - [`updated_at`](RefundPayloadBuilder::updated_at)
    pub fn build(self) -> Result<RefundPayload, BuildError> {
        Ok(RefundPayload {
            account_id: self
                .account_id
                .ok_or_else(|| BuildError::missing_field("account_id"))?,
            allocations: self
                .allocations
                .ok_or_else(|| BuildError::missing_field("allocations"))?,
            amount: self
                .amount
                .ok_or_else(|| BuildError::missing_field("amount"))?,
            created_at: self
                .created_at
                .ok_or_else(|| BuildError::missing_field("created_at"))?,
            credit_note_id: self.credit_note_id,
            currency: self
                .currency
                .ok_or_else(|| BuildError::missing_field("currency"))?,
            external_id: self.external_id,
            failure_reason: self.failure_reason,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            invoice_id: self.invoice_id,
            prev_status: self.prev_status,
            processor_id: self.processor_id,
            processor_name: self
                .processor_name
                .ok_or_else(|| BuildError::missing_field("processor_name"))?,
            status: self
                .status
                .ok_or_else(|| BuildError::missing_field("status"))?,
            updated_at: self
                .updated_at
                .ok_or_else(|| BuildError::missing_field("updated_at"))?,
        })
    }
}
