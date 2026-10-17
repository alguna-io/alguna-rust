pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GraduatedTieredPricingModelInput {
    /// Pricing tiers
    pub tiers: Vec<PricingTierInput>,
    /// Fixed quantity of units (for fixed fee type only)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub units: Option<i64>,
}

impl GraduatedTieredPricingModelInput {
    pub fn builder() -> GraduatedTieredPricingModelInputBuilder {
        <GraduatedTieredPricingModelInputBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GraduatedTieredPricingModelInputBuilder {
    tiers: Option<Vec<PricingTierInput>>,
    units: Option<i64>,
}

impl GraduatedTieredPricingModelInputBuilder {
    pub fn tiers(mut self, value: Vec<PricingTierInput>) -> Self {
        self.tiers = Some(value);
        self
    }

    pub fn units(mut self, value: i64) -> Self {
        self.units = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GraduatedTieredPricingModelInput`].
    /// This method will fail if any of the following fields are not set:
    /// - [`tiers`](GraduatedTieredPricingModelInputBuilder::tiers)
    pub fn build(self) -> Result<GraduatedTieredPricingModelInput, BuildError> {
        Ok(GraduatedTieredPricingModelInput {
            tiers: self
                .tiers
                .ok_or_else(|| BuildError::missing_field("tiers"))?,
            units: self.units,
        })
    }
}
