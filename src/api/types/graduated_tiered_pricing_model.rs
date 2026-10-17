pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GraduatedTieredPricingModel {
    /// Pricing tiers
    pub tiers: Vec<PricingTier>,
    /// Upfront units for fixed products
    #[serde(skip_serializing_if = "Option::is_none")]
    pub units: Option<i64>,
}

impl GraduatedTieredPricingModel {
    pub fn builder() -> GraduatedTieredPricingModelBuilder {
        <GraduatedTieredPricingModelBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GraduatedTieredPricingModelBuilder {
    tiers: Option<Vec<PricingTier>>,
    units: Option<i64>,
}

impl GraduatedTieredPricingModelBuilder {
    pub fn tiers(mut self, value: Vec<PricingTier>) -> Self {
        self.tiers = Some(value);
        self
    }

    pub fn units(mut self, value: i64) -> Self {
        self.units = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GraduatedTieredPricingModel`].
    /// This method will fail if any of the following fields are not set:
    /// - [`tiers`](GraduatedTieredPricingModelBuilder::tiers)
    pub fn build(self) -> Result<GraduatedTieredPricingModel, BuildError> {
        Ok(GraduatedTieredPricingModel {
            tiers: self
                .tiers
                .ok_or_else(|| BuildError::missing_field("tiers"))?,
            units: self.units,
        })
    }
}
