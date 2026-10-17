pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct SpendingThresholdInput {
    /// Amount
    pub amount: String,
    /// Bill a minimum spend in advance of each period, drawn down by spend, instead of trueing it up in arrears
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_direction: Option<BillingDirection>,
    /// Billing period
    pub period: BillingInterval,
}

impl SpendingThresholdInput {
    pub fn builder() -> SpendingThresholdInputBuilder {
        <SpendingThresholdInputBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SpendingThresholdInputBuilder {
    amount: Option<String>,
    billing_direction: Option<BillingDirection>,
    period: Option<BillingInterval>,
}

impl SpendingThresholdInputBuilder {
    pub fn amount(mut self, value: impl Into<String>) -> Self {
        self.amount = Some(value.into());
        self
    }

    pub fn billing_direction(mut self, value: BillingDirection) -> Self {
        self.billing_direction = Some(value);
        self
    }

    pub fn period(mut self, value: BillingInterval) -> Self {
        self.period = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SpendingThresholdInput`].
    /// This method will fail if any of the following fields are not set:
    /// - [`amount`](SpendingThresholdInputBuilder::amount)
    /// - [`period`](SpendingThresholdInputBuilder::period)
    pub fn build(self) -> Result<SpendingThresholdInput, BuildError> {
        Ok(SpendingThresholdInput {
            amount: self
                .amount
                .ok_or_else(|| BuildError::missing_field("amount"))?,
            billing_direction: self.billing_direction,
            period: self
                .period
                .ok_or_else(|| BuildError::missing_field("period"))?,
        })
    }
}
