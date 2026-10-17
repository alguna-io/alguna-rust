pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct TieredPricingModelInput {
    /// Pricing tiers
    pub tiers: Vec<PricingTierInput>,
    /// Fixed quantity of units (for fixed fee type only)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub units: Option<i64>,
}

impl TieredPricingModelInput {
    pub fn builder() -> TieredPricingModelInputBuilder {
        <TieredPricingModelInputBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TieredPricingModelInputBuilder {
    tiers: Option<Vec<PricingTierInput>>,
    units: Option<i64>,
}

impl TieredPricingModelInputBuilder {
    pub fn tiers(mut self, value: Vec<PricingTierInput>) -> Self {
        self.tiers = Some(value);
        self
    }

    pub fn units(mut self, value: i64) -> Self {
        self.units = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TieredPricingModelInput`].
    /// This method will fail if any of the following fields are not set:
    /// - [`tiers`](TieredPricingModelInputBuilder::tiers)
    pub fn build(self) -> Result<TieredPricingModelInput, BuildError> {
        Ok(TieredPricingModelInput {
            tiers: self
                .tiers
                .ok_or_else(|| BuildError::missing_field("tiers"))?,
            units: self.units,
        })
    }
}
