pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PaymentMethodCard {
    /// Card network
    pub brand: String,
    /// Expiry month, 1 to 12
    pub exp_month: i64,
    /// Four-digit expiry year
    pub exp_year: i64,
    /// Last four digits of the card number
    pub last4: String,
}

impl PaymentMethodCard {
    pub fn builder() -> PaymentMethodCardBuilder {
        <PaymentMethodCardBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PaymentMethodCardBuilder {
    brand: Option<String>,
    exp_month: Option<i64>,
    exp_year: Option<i64>,
    last4: Option<String>,
}

impl PaymentMethodCardBuilder {
    pub fn brand(mut self, value: impl Into<String>) -> Self {
        self.brand = Some(value.into());
        self
    }

    pub fn exp_month(mut self, value: i64) -> Self {
        self.exp_month = Some(value);
        self
    }

    pub fn exp_year(mut self, value: i64) -> Self {
        self.exp_year = Some(value);
        self
    }

    pub fn last4(mut self, value: impl Into<String>) -> Self {
        self.last4 = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PaymentMethodCard`].
    /// This method will fail if any of the following fields are not set:
    /// - [`brand`](PaymentMethodCardBuilder::brand)
    /// - [`exp_month`](PaymentMethodCardBuilder::exp_month)
    /// - [`exp_year`](PaymentMethodCardBuilder::exp_year)
    /// - [`last4`](PaymentMethodCardBuilder::last4)
    pub fn build(self) -> Result<PaymentMethodCard, BuildError> {
        Ok(PaymentMethodCard {
            brand: self
                .brand
                .ok_or_else(|| BuildError::missing_field("brand"))?,
            exp_month: self
                .exp_month
                .ok_or_else(|| BuildError::missing_field("exp_month"))?,
            exp_year: self
                .exp_year
                .ok_or_else(|| BuildError::missing_field("exp_year"))?,
            last4: self
                .last4
                .ok_or_else(|| BuildError::missing_field("last4"))?,
        })
    }
}
