pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct RefundResponse {
    /// Refund amount as a decimal string
    pub amount: String,
    /// Timestamp when the refund was created
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub created_at: DateTime<FixedOffset>,
    /// Currency code
    pub currency: String,
    /// The customer that received the refund
    pub customer_id: String,
    /// External reference ID from the payment processor
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_id: Option<String>,
    /// Link to view the refund in the payment processor's dashboard
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_url: Option<String>,
    /// Reason for failure if status is failed
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_reason: Option<String>,
    /// Unique identifier for the refund
    pub id: String,
    /// The original payment that was refunded
    pub payment_id: String,
    /// Payment processor's internal refund ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub processor_id: Option<String>,
    /// Reason for the refund
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    /// Refund status
    pub status: RefundStatus,
    /// Timestamp when the refund was last updated
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub updated_at: DateTime<FixedOffset>,
}

impl RefundResponse {
    pub fn builder() -> RefundResponseBuilder {
        <RefundResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RefundResponseBuilder {
    amount: Option<String>,
    created_at: Option<DateTime<FixedOffset>>,
    currency: Option<String>,
    customer_id: Option<String>,
    external_id: Option<String>,
    external_url: Option<String>,
    failure_reason: Option<String>,
    id: Option<String>,
    payment_id: Option<String>,
    processor_id: Option<String>,
    reason: Option<String>,
    status: Option<RefundStatus>,
    updated_at: Option<DateTime<FixedOffset>>,
}

impl RefundResponseBuilder {
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

    pub fn failure_reason(mut self, value: impl Into<String>) -> Self {
        self.failure_reason = Some(value.into());
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn payment_id(mut self, value: impl Into<String>) -> Self {
        self.payment_id = Some(value.into());
        self
    }

    pub fn processor_id(mut self, value: impl Into<String>) -> Self {
        self.processor_id = Some(value.into());
        self
    }

    pub fn reason(mut self, value: impl Into<String>) -> Self {
        self.reason = Some(value.into());
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

    /// Consumes the builder and constructs a [`RefundResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`amount`](RefundResponseBuilder::amount)
    /// - [`created_at`](RefundResponseBuilder::created_at)
    /// - [`currency`](RefundResponseBuilder::currency)
    /// - [`customer_id`](RefundResponseBuilder::customer_id)
    /// - [`id`](RefundResponseBuilder::id)
    /// - [`payment_id`](RefundResponseBuilder::payment_id)
    /// - [`status`](RefundResponseBuilder::status)
    /// - [`updated_at`](RefundResponseBuilder::updated_at)
    pub fn build(self) -> Result<RefundResponse, BuildError> {
        Ok(RefundResponse {
            amount: self
                .amount
                .ok_or_else(|| BuildError::missing_field("amount"))?,
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
            failure_reason: self.failure_reason,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            payment_id: self
                .payment_id
                .ok_or_else(|| BuildError::missing_field("payment_id"))?,
            processor_id: self.processor_id,
            reason: self.reason,
            status: self
                .status
                .ok_or_else(|| BuildError::missing_field("status"))?,
            updated_at: self
                .updated_at
                .ok_or_else(|| BuildError::missing_field("updated_at"))?,
        })
    }
}
