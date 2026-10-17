pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct SubscriptionCreditBalanceResponse {
    /// The point in time the balance was evaluated at
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub as_of: DateTime<FixedOffset>,
    /// Current balance at the evaluated point in time
    pub balance: String,
    /// Credit type
    pub credit_type: SubscriptionCreditBalanceResponseCreditType,
    /// ISO 4217 currency code
    pub currency: String,
    /// Subscription ID
    pub subscription_id: String,
}

impl SubscriptionCreditBalanceResponse {
    pub fn builder() -> SubscriptionCreditBalanceResponseBuilder {
        <SubscriptionCreditBalanceResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SubscriptionCreditBalanceResponseBuilder {
    as_of: Option<DateTime<FixedOffset>>,
    balance: Option<String>,
    credit_type: Option<SubscriptionCreditBalanceResponseCreditType>,
    currency: Option<String>,
    subscription_id: Option<String>,
}

impl SubscriptionCreditBalanceResponseBuilder {
    pub fn as_of(mut self, value: DateTime<FixedOffset>) -> Self {
        self.as_of = Some(value);
        self
    }

    pub fn balance(mut self, value: impl Into<String>) -> Self {
        self.balance = Some(value.into());
        self
    }

    pub fn credit_type(mut self, value: SubscriptionCreditBalanceResponseCreditType) -> Self {
        self.credit_type = Some(value);
        self
    }

    pub fn currency(mut self, value: impl Into<String>) -> Self {
        self.currency = Some(value.into());
        self
    }

    pub fn subscription_id(mut self, value: impl Into<String>) -> Self {
        self.subscription_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`SubscriptionCreditBalanceResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`as_of`](SubscriptionCreditBalanceResponseBuilder::as_of)
    /// - [`balance`](SubscriptionCreditBalanceResponseBuilder::balance)
    /// - [`credit_type`](SubscriptionCreditBalanceResponseBuilder::credit_type)
    /// - [`currency`](SubscriptionCreditBalanceResponseBuilder::currency)
    /// - [`subscription_id`](SubscriptionCreditBalanceResponseBuilder::subscription_id)
    pub fn build(self) -> Result<SubscriptionCreditBalanceResponse, BuildError> {
        Ok(SubscriptionCreditBalanceResponse {
            as_of: self
                .as_of
                .ok_or_else(|| BuildError::missing_field("as_of"))?,
            balance: self
                .balance
                .ok_or_else(|| BuildError::missing_field("balance"))?,
            credit_type: self
                .credit_type
                .ok_or_else(|| BuildError::missing_field("credit_type"))?,
            currency: self
                .currency
                .ok_or_else(|| BuildError::missing_field("currency"))?,
            subscription_id: self
                .subscription_id
                .ok_or_else(|| BuildError::missing_field("subscription_id"))?,
        })
    }
}
