pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct SpendingThresholdResponse {
    /// Spending threshold amount
    pub amount: String,
    /// Whether the minimum spend is billed in advance of each period and drawn down by spend, or trued up in arrears
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_direction: Option<BillingDirection>,
    /// Billing period for the threshold
    pub period: BillingInterval,
}

impl SpendingThresholdResponse {
    pub fn builder() -> SpendingThresholdResponseBuilder {
        <SpendingThresholdResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SpendingThresholdResponseBuilder {
    amount: Option<String>,
    billing_direction: Option<BillingDirection>,
    period: Option<BillingInterval>,
}

impl SpendingThresholdResponseBuilder {
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

    /// Consumes the builder and constructs a [`SpendingThresholdResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`amount`](SpendingThresholdResponseBuilder::amount)
    /// - [`period`](SpendingThresholdResponseBuilder::period)
    pub fn build(self) -> Result<SpendingThresholdResponse, BuildError> {
        Ok(SpendingThresholdResponse {
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
