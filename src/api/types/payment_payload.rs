pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct PaymentPayload {
    /// Customer account the payment belongs to
    #[serde(rename = "accountId")]
    pub account_id: String,
    /// Invoices this payment was applied to, and how much went to each
    pub allocations: Vec<PaymentAllocationPayload>,
    /// Payment amount, formatted to the currency's precision
    pub amount: String,
    /// When the payment was created, RFC 3339
    #[serde(rename = "createdAt")]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub created_at: DateTime<FixedOffset>,
    /// ISO 4217 currency code
    pub currency: String,
    /// Unique identifier for the payment
    pub id: String,
    /// Deprecated. Populated only for single-invoice payments; null when the payment spans several invoices. Read allocations instead.
    #[serde(rename = "invoiceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoice_id: Option<String>,
    /// Payment status
    pub status: PaymentStatus,
    /// When the payment was last updated, RFC 3339
    #[serde(rename = "updatedAt")]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub updated_at: DateTime<FixedOffset>,
}

impl PaymentPayload {
    pub fn builder() -> PaymentPayloadBuilder {
        <PaymentPayloadBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PaymentPayloadBuilder {
    account_id: Option<String>,
    allocations: Option<Vec<PaymentAllocationPayload>>,
    amount: Option<String>,
    created_at: Option<DateTime<FixedOffset>>,
    currency: Option<String>,
    id: Option<String>,
    invoice_id: Option<String>,
    status: Option<PaymentStatus>,
    updated_at: Option<DateTime<FixedOffset>>,
}

impl PaymentPayloadBuilder {
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

    pub fn currency(mut self, value: impl Into<String>) -> Self {
        self.currency = Some(value.into());
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

    pub fn status(mut self, value: PaymentStatus) -> Self {
        self.status = Some(value);
        self
    }

    pub fn updated_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.updated_at = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PaymentPayload`].
    /// This method will fail if any of the following fields are not set:
    /// - [`account_id`](PaymentPayloadBuilder::account_id)
    /// - [`allocations`](PaymentPayloadBuilder::allocations)
    /// - [`amount`](PaymentPayloadBuilder::amount)
    /// - [`created_at`](PaymentPayloadBuilder::created_at)
    /// - [`currency`](PaymentPayloadBuilder::currency)
    /// - [`id`](PaymentPayloadBuilder::id)
    /// - [`status`](PaymentPayloadBuilder::status)
    /// - [`updated_at`](PaymentPayloadBuilder::updated_at)
    pub fn build(self) -> Result<PaymentPayload, BuildError> {
        Ok(PaymentPayload {
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
            currency: self
                .currency
                .ok_or_else(|| BuildError::missing_field("currency"))?,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            invoice_id: self.invoice_id,
            status: self
                .status
                .ok_or_else(|| BuildError::missing_field("status"))?,
            updated_at: self
                .updated_at
                .ok_or_else(|| BuildError::missing_field("updated_at"))?,
        })
    }
}
