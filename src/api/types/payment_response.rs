pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct PaymentResponse {
    /// Per-invoice attribution of the payment
    pub allocations: Vec<PaymentAllocation>,
    /// Payment amount
    pub amount: String,
    /// Amount received
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount_received: Option<String>,
    /// Amount refunded
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount_refunded: Option<String>,
    /// Timestamp when the payment was created
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub created_at: DateTime<FixedOffset>,
    /// ISO 4217 currency code
    pub currency: String,
    /// Customer ID
    pub customer_id: String,
    /// Payment processor external ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_id: Option<String>,
    /// URL to view the payment in the processor's dashboard
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_url: Option<String>,
    /// Timestamp when the payment failed
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failed_at: Option<DateTime<FixedOffset>>,
    /// Unique identifier for the payment
    pub id: String,
    /// Deprecated: use allocations. Associated invoice ID, set only for single-invoice payments.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoice_id: Option<String>,
    /// Timestamp when the payment was completed
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paid_at: Option<DateTime<FixedOffset>>,
    /// Payment status
    pub status: PaymentStatus,
    /// Timestamp when the payment was last updated
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub updated_at: DateTime<FixedOffset>,
}

impl PaymentResponse {
    pub fn builder() -> PaymentResponseBuilder {
        <PaymentResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PaymentResponseBuilder {
    allocations: Option<Vec<PaymentAllocation>>,
    amount: Option<String>,
    amount_received: Option<String>,
    amount_refunded: Option<String>,
    created_at: Option<DateTime<FixedOffset>>,
    currency: Option<String>,
    customer_id: Option<String>,
    external_id: Option<String>,
    external_url: Option<String>,
    failed_at: Option<DateTime<FixedOffset>>,
    id: Option<String>,
    invoice_id: Option<String>,
    paid_at: Option<DateTime<FixedOffset>>,
    status: Option<PaymentStatus>,
    updated_at: Option<DateTime<FixedOffset>>,
}

impl PaymentResponseBuilder {
    pub fn allocations(mut self, value: Vec<PaymentAllocation>) -> Self {
        self.allocations = Some(value);
        self
    }

    pub fn amount(mut self, value: impl Into<String>) -> Self {
        self.amount = Some(value.into());
        self
    }

    pub fn amount_received(mut self, value: impl Into<String>) -> Self {
        self.amount_received = Some(value.into());
        self
    }

    pub fn amount_refunded(mut self, value: impl Into<String>) -> Self {
        self.amount_refunded = Some(value.into());
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

    pub fn external_id(mut self, value: impl Into<String>) -> Self {
        self.external_id = Some(value.into());
        self
    }

    pub fn external_url(mut self, value: impl Into<String>) -> Self {
        self.external_url = Some(value.into());
        self
    }

    pub fn failed_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.failed_at = Some(value);
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

    pub fn paid_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.paid_at = Some(value);
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

    /// Consumes the builder and constructs a [`PaymentResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`allocations`](PaymentResponseBuilder::allocations)
    /// - [`amount`](PaymentResponseBuilder::amount)
    /// - [`created_at`](PaymentResponseBuilder::created_at)
    /// - [`currency`](PaymentResponseBuilder::currency)
    /// - [`customer_id`](PaymentResponseBuilder::customer_id)
    /// - [`id`](PaymentResponseBuilder::id)
    /// - [`status`](PaymentResponseBuilder::status)
    /// - [`updated_at`](PaymentResponseBuilder::updated_at)
    pub fn build(self) -> Result<PaymentResponse, BuildError> {
        Ok(PaymentResponse {
            allocations: self
                .allocations
                .ok_or_else(|| BuildError::missing_field("allocations"))?,
            amount: self
                .amount
                .ok_or_else(|| BuildError::missing_field("amount"))?,
            amount_received: self.amount_received,
            amount_refunded: self.amount_refunded,
            created_at: self
                .created_at
                .ok_or_else(|| BuildError::missing_field("created_at"))?,
            currency: self
                .currency
                .ok_or_else(|| BuildError::missing_field("currency"))?,
            customer_id: self
                .customer_id
                .ok_or_else(|| BuildError::missing_field("customer_id"))?,
            external_id: self.external_id,
            external_url: self.external_url,
            failed_at: self.failed_at,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            invoice_id: self.invoice_id,
            paid_at: self.paid_at,
            status: self
                .status
                .ok_or_else(|| BuildError::missing_field("status"))?,
            updated_at: self
                .updated_at
                .ok_or_else(|| BuildError::missing_field("updated_at"))?,
        })
    }
}
