pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GraduatedTieredPricingModelAdjust {
    /// Pricing tiers (replaces entirely if provided)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tiers: Option<Vec<PricingTierInput>>,
    /// Fixed quantity of units
    #[serde(skip_serializing_if = "Option::is_none")]
    pub units: Option<i64>,
}

impl GraduatedTieredPricingModelAdjust {
    pub fn builder() -> GraduatedTieredPricingModelAdjustBuilder {
        <GraduatedTieredPricingModelAdjustBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GraduatedTieredPricingModelAdjustBuilder {
    tiers: Option<Vec<PricingTierInput>>,
    units: Option<i64>,
}

impl GraduatedTieredPricingModelAdjustBuilder {
    pub fn tiers(mut self, value: Vec<PricingTierInput>) -> Self {
        self.tiers = Some(value);
        self
    }

    pub fn units(mut self, value: i64) -> Self {
        self.units = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GraduatedTieredPricingModelAdjust`].
    pub fn build(self) -> Result<GraduatedTieredPricingModelAdjust, BuildError> {
        Ok(GraduatedTieredPricingModelAdjust {
            tiers: self.tiers,
            units: self.units,
        })
    }
}
