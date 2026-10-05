pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct CreditBalancePayload {
    /// Customer account whose balance was depleted
    #[serde(rename = "accountId")]
    pub account_id: String,
    /// Remaining balance, always zero for this event
    pub amount: String,
    /// Credit type
    pub r#type: SubscriptionCreditType,
}

impl CreditBalancePayload {
    pub fn builder() -> CreditBalancePayloadBuilder {
        <CreditBalancePayloadBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreditBalancePayloadBuilder {
    account_id: Option<String>,
    amount: Option<String>,
    r#type: Option<SubscriptionCreditType>,
}

impl CreditBalancePayloadBuilder {
    pub fn account_id(mut self, value: impl Into<String>) -> Self {
        self.account_id = Some(value.into());
        self
    }

    pub fn amount(mut self, value: impl Into<String>) -> Self {
        self.amount = Some(value.into());
        self
    }

    pub fn r#type(mut self, value: SubscriptionCreditType) -> Self {
        self.r#type = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CreditBalancePayload`].
    /// This method will fail if any of the following fields are not set:
    /// - [`account_id`](CreditBalancePayloadBuilder::account_id)
    /// - [`amount`](CreditBalancePayloadBuilder::amount)
    /// - [`r#type`](CreditBalancePayloadBuilder::r#type)
    pub fn build(self) -> Result<CreditBalancePayload, BuildError> {
        Ok(CreditBalancePayload {
            account_id: self
                .account_id
                .ok_or_else(|| BuildError::missing_field("account_id"))?,
            amount: self
                .amount
                .ok_or_else(|| BuildError::missing_field("amount"))?,
            r#type: self
                .r#type
                .ok_or_else(|| BuildError::missing_field("r#type"))?,
        })
    }
}
