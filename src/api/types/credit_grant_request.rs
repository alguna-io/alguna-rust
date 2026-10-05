pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreditGrantRequest {
    /// Credit amount to grant
    pub amount: String,
    /// Type of credit
    #[serde(skip_serializing_if = "Option::is_none")]
    pub credit_type: Option<CreditGrantRequestCreditType>,
    /// When the credit grant expires
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<DateTime<FixedOffset>>,
    /// Reason for the credit grant
    pub reason: String,
}

impl CreditGrantRequest {
    pub fn builder() -> CreditGrantRequestBuilder {
        <CreditGrantRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreditGrantRequestBuilder {
    amount: Option<String>,
    credit_type: Option<CreditGrantRequestCreditType>,
    expires_at: Option<DateTime<FixedOffset>>,
    reason: Option<String>,
}

impl CreditGrantRequestBuilder {
    pub fn amount(mut self, value: impl Into<String>) -> Self {
        self.amount = Some(value.into());
        self
    }

    pub fn credit_type(mut self, value: CreditGrantRequestCreditType) -> Self {
        self.credit_type = Some(value);
        self
    }

    pub fn expires_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.expires_at = Some(value);
        self
    }

    pub fn reason(mut self, value: impl Into<String>) -> Self {
        self.reason = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CreditGrantRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`amount`](CreditGrantRequestBuilder::amount)
    /// - [`reason`](CreditGrantRequestBuilder::reason)
    pub fn build(self) -> Result<CreditGrantRequest, BuildError> {
        Ok(CreditGrantRequest {
            amount: self
                .amount
                .ok_or_else(|| BuildError::missing_field("amount"))?,
            credit_type: self.credit_type,
            expires_at: self.expires_at,
            reason: self
                .reason
                .ok_or_else(|| BuildError::missing_field("reason"))?,
        })
    }
}
