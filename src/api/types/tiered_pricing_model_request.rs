pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct TieredPricingModelRequest {
    /// Pricing tiers
    pub tiers: Vec<PricingTier>,
    /// Upfront units for fixed products
    #[serde(skip_serializing_if = "Option::is_none")]
    pub units: Option<i64>,
}

impl TieredPricingModelRequest {
    pub fn builder() -> TieredPricingModelRequestBuilder {
        <TieredPricingModelRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TieredPricingModelRequestBuilder {
    tiers: Option<Vec<PricingTier>>,
    units: Option<i64>,
}

impl TieredPricingModelRequestBuilder {
    pub fn tiers(mut self, value: Vec<PricingTier>) -> Self {
        self.tiers = Some(value);
        self
    }

    pub fn units(mut self, value: i64) -> Self {
        self.units = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TieredPricingModelRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`tiers`](TieredPricingModelRequestBuilder::tiers)
    pub fn build(self) -> Result<TieredPricingModelRequest, BuildError> {
        Ok(TieredPricingModelRequest {
            tiers: self
                .tiers
                .ok_or_else(|| BuildError::missing_field("tiers"))?,
            units: self.units,
        })
    }
}
