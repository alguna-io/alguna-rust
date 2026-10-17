pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct CreditGrantResponse {
    /// Grant amount
    pub amount: String,
    /// When the grant was created
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub created_at: DateTime<FixedOffset>,
    /// Customer ID
    pub customer_id: String,
    /// Expiration date
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<DateTime<FixedOffset>>,
    /// How the grant was created
    pub grant_type: CreditGrantResponseGrantType,
    /// Credit grant ID
    pub id: String,
    /// Reason for the grant
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    /// Subscription ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription_id: Option<String>,
}

impl CreditGrantResponse {
    pub fn builder() -> CreditGrantResponseBuilder {
        <CreditGrantResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreditGrantResponseBuilder {
    amount: Option<String>,
    created_at: Option<DateTime<FixedOffset>>,
    customer_id: Option<String>,
    expires_at: Option<DateTime<FixedOffset>>,
    grant_type: Option<CreditGrantResponseGrantType>,
    id: Option<String>,
    reason: Option<String>,
    subscription_id: Option<String>,
}

impl CreditGrantResponseBuilder {
    pub fn amount(mut self, value: impl Into<String>) -> Self {
        self.amount = Some(value.into());
        self
    }

    pub fn created_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.created_at = Some(value);
        self
    }

    pub fn customer_id(mut self, value: impl Into<String>) -> Self {
        self.customer_id = Some(value.into());
        self
    }

    pub fn expires_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.expires_at = Some(value);
        self
    }

    pub fn grant_type(mut self, value: CreditGrantResponseGrantType) -> Self {
        self.grant_type = Some(value);
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn reason(mut self, value: impl Into<String>) -> Self {
        self.reason = Some(value.into());
        self
    }

    pub fn subscription_id(mut self, value: impl Into<String>) -> Self {
        self.subscription_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CreditGrantResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`amount`](CreditGrantResponseBuilder::amount)
    /// - [`created_at`](CreditGrantResponseBuilder::created_at)
    /// - [`customer_id`](CreditGrantResponseBuilder::customer_id)
    /// - [`grant_type`](CreditGrantResponseBuilder::grant_type)
    /// - [`id`](CreditGrantResponseBuilder::id)
    pub fn build(self) -> Result<CreditGrantResponse, BuildError> {
        Ok(CreditGrantResponse {
            amount: self
                .amount
                .ok_or_else(|| BuildError::missing_field("amount"))?,
            created_at: self
                .created_at
                .ok_or_else(|| BuildError::missing_field("created_at"))?,
            customer_id: self
                .customer_id
                .ok_or_else(|| BuildError::missing_field("customer_id"))?,
            expires_at: self.expires_at,
            grant_type: self
                .grant_type
                .ok_or_else(|| BuildError::missing_field("grant_type"))?,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            reason: self.reason,
            subscription_id: self.subscription_id,
        })
    }
}
