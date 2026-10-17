pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PrepaidTieredPricingModelInput {
    /// Number of prepaid units
    pub prepaid_units: i64,
    /// Overage pricing tiers
    pub tiers: Vec<PricingTierInput>,
}

impl PrepaidTieredPricingModelInput {
    pub fn builder() -> PrepaidTieredPricingModelInputBuilder {
        <PrepaidTieredPricingModelInputBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PrepaidTieredPricingModelInputBuilder {
    prepaid_units: Option<i64>,
    tiers: Option<Vec<PricingTierInput>>,
}

impl PrepaidTieredPricingModelInputBuilder {
    pub fn prepaid_units(mut self, value: i64) -> Self {
        self.prepaid_units = Some(value);
        self
    }

    pub fn tiers(mut self, value: Vec<PricingTierInput>) -> Self {
        self.tiers = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PrepaidTieredPricingModelInput`].
    /// This method will fail if any of the following fields are not set:
    /// - [`prepaid_units`](PrepaidTieredPricingModelInputBuilder::prepaid_units)
    /// - [`tiers`](PrepaidTieredPricingModelInputBuilder::tiers)
    pub fn build(self) -> Result<PrepaidTieredPricingModelInput, BuildError> {
        Ok(PrepaidTieredPricingModelInput {
            prepaid_units: self
                .prepaid_units
                .ok_or_else(|| BuildError::missing_field("prepaid_units"))?,
            tiers: self
                .tiers
                .ok_or_else(|| BuildError::missing_field("tiers"))?,
        })
    }
}
