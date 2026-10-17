pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PaymentMethodDetails {
    /// Set when type is bank_transfer
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bank_account: Option<PaymentMethodBankAccount>,
    /// Set when type is payment_card
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card: Option<PaymentMethodCard>,
}

impl PaymentMethodDetails {
    pub fn builder() -> PaymentMethodDetailsBuilder {
        <PaymentMethodDetailsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PaymentMethodDetailsBuilder {
    bank_account: Option<PaymentMethodBankAccount>,
    card: Option<PaymentMethodCard>,
}

impl PaymentMethodDetailsBuilder {
    pub fn bank_account(mut self, value: PaymentMethodBankAccount) -> Self {
        self.bank_account = Some(value);
        self
    }

    pub fn card(mut self, value: PaymentMethodCard) -> Self {
        self.card = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PaymentMethodDetails`].
    pub fn build(self) -> Result<PaymentMethodDetails, BuildError> {
        Ok(PaymentMethodDetails {
            bank_account: self.bank_account,
            card: self.card,
        })
    }
}
