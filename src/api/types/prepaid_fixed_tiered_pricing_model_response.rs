pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct PrepaidFixedTieredPricingModelResponse {
    /// Billing interval for overages
    pub overages_charge_interval: BillingInterval,
    /// Number of prepaid units included
    pub prepaid_units: i64,
    /// Overage pricing tiers
    pub tiers: Vec<PricingTierResponse>,
}

impl PrepaidFixedTieredPricingModelResponse {
    pub fn builder() -> PrepaidFixedTieredPricingModelResponseBuilder {
        <PrepaidFixedTieredPricingModelResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PrepaidFixedTieredPricingModelResponseBuilder {
    overages_charge_interval: Option<BillingInterval>,
    prepaid_units: Option<i64>,
    tiers: Option<Vec<PricingTierResponse>>,
}

impl PrepaidFixedTieredPricingModelResponseBuilder {
    pub fn overages_charge_interval(mut self, value: BillingInterval) -> Self {
        self.overages_charge_interval = Some(value);
        self
    }

    pub fn prepaid_units(mut self, value: i64) -> Self {
        self.prepaid_units = Some(value);
        self
    }

    pub fn tiers(mut self, value: Vec<PricingTierResponse>) -> Self {
        self.tiers = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PrepaidFixedTieredPricingModelResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`overages_charge_interval`](PrepaidFixedTieredPricingModelResponseBuilder::overages_charge_interval)
    /// - [`prepaid_units`](PrepaidFixedTieredPricingModelResponseBuilder::prepaid_units)
    /// - [`tiers`](PrepaidFixedTieredPricingModelResponseBuilder::tiers)
    pub fn build(self) -> Result<PrepaidFixedTieredPricingModelResponse, BuildError> {
        Ok(PrepaidFixedTieredPricingModelResponse {
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
