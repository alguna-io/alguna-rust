pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct TieredPricingModelResponse {
    /// Pricing tiers (first matching tier applies)
    pub tiers: Vec<PricingTierResponse>,
    /// Fixed quantity of units (for fixed fee type only)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub units: Option<i64>,
}

impl TieredPricingModelResponse {
    pub fn builder() -> TieredPricingModelResponseBuilder {
        <TieredPricingModelResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TieredPricingModelResponseBuilder {
    tiers: Option<Vec<PricingTierResponse>>,
    units: Option<i64>,
}

impl TieredPricingModelResponseBuilder {
    pub fn tiers(mut self, value: Vec<PricingTierResponse>) -> Self {
        self.tiers = Some(value);
        self
    }

    pub fn units(mut self, value: i64) -> Self {
        self.units = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TieredPricingModelResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`tiers`](TieredPricingModelResponseBuilder::tiers)
    pub fn build(self) -> Result<TieredPricingModelResponse, BuildError> {
        Ok(TieredPricingModelResponse {
            tiers: self
                .tiers
                .ok_or_else(|| BuildError::missing_field("tiers"))?,
            units: self.units,
        })
    }
}
