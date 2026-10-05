pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PrepaidTieredPricingModel {
    /// Number of prepaid units
    pub prepaid_units: i64,
    /// Pricing tiers
    pub tiers: Vec<PricingTier>,
}

impl PrepaidTieredPricingModel {
    pub fn builder() -> PrepaidTieredPricingModelBuilder {
        <PrepaidTieredPricingModelBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PrepaidTieredPricingModelBuilder {
    prepaid_units: Option<i64>,
    tiers: Option<Vec<PricingTier>>,
}

impl PrepaidTieredPricingModelBuilder {
    pub fn prepaid_units(mut self, value: i64) -> Self {
        self.prepaid_units = Some(value);
        self
    }

    pub fn tiers(mut self, value: Vec<PricingTier>) -> Self {
        self.tiers = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PrepaidTieredPricingModel`].
    /// This method will fail if any of the following fields are not set:
    /// - [`prepaid_units`](PrepaidTieredPricingModelBuilder::prepaid_units)
    /// - [`tiers`](PrepaidTieredPricingModelBuilder::tiers)
    pub fn build(self) -> Result<PrepaidTieredPricingModel, BuildError> {
        Ok(PrepaidTieredPricingModel {
            prepaid_units: self
                .prepaid_units
                .ok_or_else(|| BuildError::missing_field("prepaid_units"))?,
            tiers: self
                .tiers
                .ok_or_else(|| BuildError::missing_field("tiers"))?,
        })
    }
}
