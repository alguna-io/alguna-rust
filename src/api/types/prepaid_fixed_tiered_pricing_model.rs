pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PrepaidFixedTieredPricingModel {
    /// Interval for charging overages
    pub overages_charge_interval: String,
    /// Number of prepaid units
    pub prepaid_units: i64,
    /// Pricing tiers
    pub tiers: Vec<PricingTier>,
}

impl PrepaidFixedTieredPricingModel {
    pub fn builder() -> PrepaidFixedTieredPricingModelBuilder {
        <PrepaidFixedTieredPricingModelBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PrepaidFixedTieredPricingModelBuilder {
    overages_charge_interval: Option<String>,
    prepaid_units: Option<i64>,
    tiers: Option<Vec<PricingTier>>,
}

impl PrepaidFixedTieredPricingModelBuilder {
    pub fn overages_charge_interval(mut self, value: impl Into<String>) -> Self {
        self.overages_charge_interval = Some(value.into());
        self
    }

    pub fn prepaid_units(mut self, value: i64) -> Self {
        self.prepaid_units = Some(value);
        self
    }

    pub fn tiers(mut self, value: Vec<PricingTier>) -> Self {
        self.tiers = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PrepaidFixedTieredPricingModel`].
    /// This method will fail if any of the following fields are not set:
    /// - [`overages_charge_interval`](PrepaidFixedTieredPricingModelBuilder::overages_charge_interval)
    /// - [`prepaid_units`](PrepaidFixedTieredPricingModelBuilder::prepaid_units)
    /// - [`tiers`](PrepaidFixedTieredPricingModelBuilder::tiers)
    pub fn build(self) -> Result<PrepaidFixedTieredPricingModel, BuildError> {
        Ok(PrepaidFixedTieredPricingModel {
            overages_charge_interval: self
                .overages_charge_interval
                .ok_or_else(|| BuildError::missing_field("overages_charge_interval"))?,
            prepaid_units: self
                .prepaid_units
                .ok_or_else(|| BuildError::missing_field("prepaid_units"))?,
            tiers: self
                .tiers
                .ok_or_else(|| BuildError::missing_field("tiers"))?,
        })
    }
}
