pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct EntitlementAllocationInput {
    /// Allocation amount
    pub amount: String,
    /// Credit type
    pub credit_type: SubscriptionCreditType,
    /// Whether rollover is enabled
    pub rollover_enabled: bool,
    /// Maximum rollover periods
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rollover_max_periods: Option<i64>,
}

impl EntitlementAllocationInput {
    pub fn builder() -> EntitlementAllocationInputBuilder {
        <EntitlementAllocationInputBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct EntitlementAllocationInputBuilder {
    amount: Option<String>,
    credit_type: Option<SubscriptionCreditType>,
    rollover_enabled: Option<bool>,
    rollover_max_periods: Option<i64>,
}

impl EntitlementAllocationInputBuilder {
    pub fn amount(mut self, value: impl Into<String>) -> Self {
        self.amount = Some(value.into());
        self
    }

    pub fn credit_type(mut self, value: SubscriptionCreditType) -> Self {
        self.credit_type = Some(value);
        self
    }

    pub fn rollover_enabled(mut self, value: bool) -> Self {
        self.rollover_enabled = Some(value);
        self
    }

    pub fn rollover_max_periods(mut self, value: i64) -> Self {
        self.rollover_max_periods = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`EntitlementAllocationInput`].
    /// This method will fail if any of the following fields are not set:
    /// - [`amount`](EntitlementAllocationInputBuilder::amount)
    /// - [`credit_type`](EntitlementAllocationInputBuilder::credit_type)
    /// - [`rollover_enabled`](EntitlementAllocationInputBuilder::rollover_enabled)
    pub fn build(self) -> Result<EntitlementAllocationInput, BuildError> {
        Ok(EntitlementAllocationInput {
            amount: self
                .amount
                .ok_or_else(|| BuildError::missing_field("amount"))?,
            credit_type: self
                .credit_type
                .ok_or_else(|| BuildError::missing_field("credit_type"))?,
            rollover_enabled: self
                .rollover_enabled
                .ok_or_else(|| BuildError::missing_field("rollover_enabled"))?,
            rollover_max_periods: self.rollover_max_periods,
        })
    }
}
