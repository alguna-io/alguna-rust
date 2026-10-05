pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct CreditGrantedPayload {
    /// Customer account the credits were granted to
    #[serde(rename = "accountId")]
    pub account_id: String,
    /// Amount granted
    pub amount: String,
    /// Unique identifier for the credit grant
    #[serde(rename = "grantId")]
    pub grant_id: String,
    /// Reason recorded for the grant
    pub reason: String,
    /// Credit type
    pub r#type: SubscriptionCreditType,
}

impl CreditGrantedPayload {
    pub fn builder() -> CreditGrantedPayloadBuilder {
        <CreditGrantedPayloadBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreditGrantedPayloadBuilder {
    account_id: Option<String>,
    amount: Option<String>,
    grant_id: Option<String>,
    reason: Option<String>,
    r#type: Option<SubscriptionCreditType>,
}

impl CreditGrantedPayloadBuilder {
    pub fn account_id(mut self, value: impl Into<String>) -> Self {
        self.account_id = Some(value.into());
        self
    }

    pub fn amount(mut self, value: impl Into<String>) -> Self {
        self.amount = Some(value.into());
        self
    }

    pub fn grant_id(mut self, value: impl Into<String>) -> Self {
        self.grant_id = Some(value.into());
        self
    }

    pub fn reason(mut self, value: impl Into<String>) -> Self {
        self.reason = Some(value.into());
        self
    }

    pub fn r#type(mut self, value: SubscriptionCreditType) -> Self {
        self.r#type = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CreditGrantedPayload`].
    /// This method will fail if any of the following fields are not set:
    /// - [`account_id`](CreditGrantedPayloadBuilder::account_id)
    /// - [`amount`](CreditGrantedPayloadBuilder::amount)
    /// - [`grant_id`](CreditGrantedPayloadBuilder::grant_id)
    /// - [`reason`](CreditGrantedPayloadBuilder::reason)
    /// - [`r#type`](CreditGrantedPayloadBuilder::r#type)
    pub fn build(self) -> Result<CreditGrantedPayload, BuildError> {
        Ok(CreditGrantedPayload {
            account_id: self
                .account_id
                .ok_or_else(|| BuildError::missing_field("account_id"))?,
            amount: self
                .amount
                .ok_or_else(|| BuildError::missing_field("amount"))?,
            grant_id: self
                .grant_id
                .ok_or_else(|| BuildError::missing_field("grant_id"))?,
            reason: self
                .reason
                .ok_or_else(|| BuildError::missing_field("reason"))?,
            r#type: self
                .r#type
                .ok_or_else(|| BuildError::missing_field("r#type"))?,
        })
    }
}
