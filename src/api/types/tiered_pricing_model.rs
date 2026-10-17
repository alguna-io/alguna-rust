pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct TieredPricingModel {
    /// Pricing tiers
    pub tiers: Vec<PricingTier>,
    /// Upfront units for fixed products
    #[serde(skip_serializing_if = "Option::is_none")]
    pub units: Option<i64>,
}

impl TieredPricingModel {
    pub fn builder() -> TieredPricingModelBuilder {
        <TieredPricingModelBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TieredPricingModelBuilder {
    tiers: Option<Vec<PricingTier>>,
    units: Option<i64>,
}

impl TieredPricingModelBuilder {
    pub fn tiers(mut self, value: Vec<PricingTier>) -> Self {
        self.tiers = Some(value);
        self
    }

    pub fn units(mut self, value: i64) -> Self {
        self.units = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TieredPricingModel`].
    /// This method will fail if any of the following fields are not set:
    /// - [`tiers`](TieredPricingModelBuilder::tiers)
    pub fn build(self) -> Result<TieredPricingModel, BuildError> {
        Ok(TieredPricingModel {
            tiers: self
                .tiers
                .ok_or_else(|| BuildError::missing_field("tiers"))?,
            units: self.units,
        })
    }
}
