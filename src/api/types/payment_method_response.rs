pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PaymentMethodResponse {
    /// Timestamp when the payment method was saved
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub created_at: DateTime<FixedOffset>,
    /// Customer the payment method belongs to
    pub customer_id: String,
    /// Redacted details of the payment method. Exactly one key is set, and it matches type.
    pub details: PaymentMethodDetails,
    /// Unique identifier for the payment method
    pub id: String,
    /// Whether this is the customer's default payment method. Auto-pay charges the default unless the subscription has its own payment method.
    pub is_default: bool,
    /// Payment provider that stores the payment method
    pub provider: String,
    /// Kind of payment method. Selects which key of details is set.
    pub r#type: String,
}

impl PaymentMethodResponse {
    pub fn builder() -> PaymentMethodResponseBuilder {
        <PaymentMethodResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PaymentMethodResponseBuilder {
    created_at: Option<DateTime<FixedOffset>>,
    customer_id: Option<String>,
    details: Option<PaymentMethodDetails>,
    id: Option<String>,
    is_default: Option<bool>,
    provider: Option<String>,
    r#type: Option<String>,
}

impl PaymentMethodResponseBuilder {
    pub fn created_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.created_at = Some(value);
        self
    }

    pub fn customer_id(mut self, value: impl Into<String>) -> Self {
        self.customer_id = Some(value.into());
        self
    }

    pub fn details(mut self, value: PaymentMethodDetails) -> Self {
        self.details = Some(value);
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn is_default(mut self, value: bool) -> Self {
        self.is_default = Some(value);
        self
    }

    pub fn provider(mut self, value: impl Into<String>) -> Self {
        self.provider = Some(value.into());
        self
    }

    pub fn r#type(mut self, value: impl Into<String>) -> Self {
        self.r#type = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PaymentMethodResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`created_at`](PaymentMethodResponseBuilder::created_at)
    /// - [`customer_id`](PaymentMethodResponseBuilder::customer_id)
    /// - [`details`](PaymentMethodResponseBuilder::details)
    /// - [`id`](PaymentMethodResponseBuilder::id)
    /// - [`is_default`](PaymentMethodResponseBuilder::is_default)
    /// - [`provider`](PaymentMethodResponseBuilder::provider)
    /// - [`r#type`](PaymentMethodResponseBuilder::r#type)
    pub fn build(self) -> Result<PaymentMethodResponse, BuildError> {
        Ok(PaymentMethodResponse {
            created_at: self
                .created_at
                .ok_or_else(|| BuildError::missing_field("created_at"))?,
            customer_id: self
                .customer_id
                .ok_or_else(|| BuildError::missing_field("customer_id"))?,
            details: self
                .details
                .ok_or_else(|| BuildError::missing_field("details"))?,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            is_default: self
                .is_default
                .ok_or_else(|| BuildError::missing_field("is_default"))?,
            provider: self
                .provider
                .ok_or_else(|| BuildError::missing_field("provider"))?,
            r#type: self
                .r#type
                .ok_or_else(|| BuildError::missing_field("r#type"))?,
        })
    }
}
