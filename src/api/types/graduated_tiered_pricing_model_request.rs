pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GraduatedTieredPricingModelRequest {
    /// Pricing tiers
    pub tiers: Vec<PricingTier>,
    /// Upfront units for fixed products
    #[serde(skip_serializing_if = "Option::is_none")]
    pub units: Option<i64>,
}

impl GraduatedTieredPricingModelRequest {
    pub fn builder() -> GraduatedTieredPricingModelRequestBuilder {
        <GraduatedTieredPricingModelRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GraduatedTieredPricingModelRequestBuilder {
    tiers: Option<Vec<PricingTier>>,
    units: Option<i64>,
}

impl GraduatedTieredPricingModelRequestBuilder {
    pub fn tiers(mut self, value: Vec<PricingTier>) -> Self {
        self.tiers = Some(value);
        self
    }

    pub fn units(mut self, value: i64) -> Self {
        self.units = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GraduatedTieredPricingModelRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`tiers`](GraduatedTieredPricingModelRequestBuilder::tiers)
    pub fn build(self) -> Result<GraduatedTieredPricingModelRequest, BuildError> {
        Ok(GraduatedTieredPricingModelRequest {
            tiers: self
                .tiers
                .ok_or_else(|| BuildError::missing_field("tiers"))?,
            units: self.units,
        })
    }
}
