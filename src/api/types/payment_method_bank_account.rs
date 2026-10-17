pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PaymentMethodBankAccount {
    /// Name of the account holder
    pub account_holder_name: String,
    /// ISO 3166-1 alpha-2 country of the bank account
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    /// Last four digits of the account number
    pub last4: String,
}

impl PaymentMethodBankAccount {
    pub fn builder() -> PaymentMethodBankAccountBuilder {
        <PaymentMethodBankAccountBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PaymentMethodBankAccountBuilder {
    account_holder_name: Option<String>,
    country: Option<String>,
    last4: Option<String>,
}

impl PaymentMethodBankAccountBuilder {
    pub fn account_holder_name(mut self, value: impl Into<String>) -> Self {
        self.account_holder_name = Some(value.into());
        self
    }

    pub fn country(mut self, value: impl Into<String>) -> Self {
        self.country = Some(value.into());
        self
    }

    pub fn last4(mut self, value: impl Into<String>) -> Self {
        self.last4 = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PaymentMethodBankAccount`].
    /// This method will fail if any of the following fields are not set:
    /// - [`account_holder_name`](PaymentMethodBankAccountBuilder::account_holder_name)
    /// - [`last4`](PaymentMethodBankAccountBuilder::last4)
    pub fn build(self) -> Result<PaymentMethodBankAccount, BuildError> {
        Ok(PaymentMethodBankAccount {
            account_holder_name: self
                .account_holder_name
                .ok_or_else(|| BuildError::missing_field("account_holder_name"))?,
            country: self.country,
            last4: self
                .last4
                .ok_or_else(|| BuildError::missing_field("last4"))?,
        })
    }
}
