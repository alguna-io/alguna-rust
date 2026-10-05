pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct SpendingThreshold {
    /// Threshold amount
    pub amount: String,
    /// Bill a minimum spend in advance of each period, drawn down by spend, instead of trueing it up in arrears
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_direction: Option<BillingDirection>,
    /// Period for the threshold
    pub period: String,
}

impl SpendingThreshold {
    pub fn builder() -> SpendingThresholdBuilder {
        <SpendingThresholdBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SpendingThresholdBuilder {
    amount: Option<String>,
    billing_direction: Option<BillingDirection>,
    period: Option<String>,
}

impl SpendingThresholdBuilder {
    pub fn amount(mut self, value: impl Into<String>) -> Self {
        self.amount = Some(value.into());
        self
    }

    pub fn billing_direction(mut self, value: BillingDirection) -> Self {
        self.billing_direction = Some(value);
        self
    }

    pub fn period(mut self, value: impl Into<String>) -> Self {
        self.period = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`SpendingThreshold`].
    /// This method will fail if any of the following fields are not set:
    /// - [`amount`](SpendingThresholdBuilder::amount)
    /// - [`period`](SpendingThresholdBuilder::period)
    pub fn build(self) -> Result<SpendingThreshold, BuildError> {
        Ok(SpendingThreshold {
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
