pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct TieredPercentagePricingModelRequest {
    /// Pricing tiers
    pub tiers: Vec<GraduatedTier>,
}

impl TieredPercentagePricingModelRequest {
    pub fn builder() -> TieredPercentagePricingModelRequestBuilder {
        <TieredPercentagePricingModelRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TieredPercentagePricingModelRequestBuilder {
    tiers: Option<Vec<GraduatedTier>>,
}

impl TieredPercentagePricingModelRequestBuilder {
    pub fn tiers(mut self, value: Vec<GraduatedTier>) -> Self {
        self.tiers = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TieredPercentagePricingModelRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`tiers`](TieredPercentagePricingModelRequestBuilder::tiers)
    pub fn build(self) -> Result<TieredPercentagePricingModelRequest, BuildError> {
        Ok(TieredPercentagePricingModelRequest {
            tiers: self
                .tiers
                .ok_or_else(|| BuildError::missing_field("tiers"))?,
        })
    }
}
