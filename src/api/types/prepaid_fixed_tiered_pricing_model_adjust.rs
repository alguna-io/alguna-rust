pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PrepaidFixedTieredPricingModelAdjust {
    /// Overage charge interval
    #[serde(skip_serializing_if = "Option::is_none")]
    pub overages_charge_interval: Option<BillingInterval>,
    /// Number of prepaid units
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prepaid_units: Option<i64>,
    /// Overage pricing tiers (replaces entirely if provided)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tiers: Option<Vec<PricingTierInput>>,
}

impl PrepaidFixedTieredPricingModelAdjust {
    pub fn builder() -> PrepaidFixedTieredPricingModelAdjustBuilder {
        <PrepaidFixedTieredPricingModelAdjustBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PrepaidFixedTieredPricingModelAdjustBuilder {
    overages_charge_interval: Option<BillingInterval>,
    prepaid_units: Option<i64>,
    tiers: Option<Vec<PricingTierInput>>,
}

impl PrepaidFixedTieredPricingModelAdjustBuilder {
    pub fn overages_charge_interval(mut self, value: BillingInterval) -> Self {
        self.overages_charge_interval = Some(value);
        self
    }

    pub fn prepaid_units(mut self, value: i64) -> Self {
        self.prepaid_units = Some(value);
        self
    }

    pub fn tiers(mut self, value: Vec<PricingTierInput>) -> Self {
        self.tiers = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PrepaidFixedTieredPricingModelAdjust`].
    pub fn build(self) -> Result<PrepaidFixedTieredPricingModelAdjust, BuildError> {
        Ok(PrepaidFixedTieredPricingModelAdjust {
            overages_charge_interval: self.overages_charge_interval,
            prepaid_units: self.prepaid_units,
            tiers: self.tiers,
        })
    }
}
