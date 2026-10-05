pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GraduatedPercentagePricingModelRequest {
    /// Pricing tiers
    pub tiers: Vec<GraduatedTier>,
}

impl GraduatedPercentagePricingModelRequest {
    pub fn builder() -> GraduatedPercentagePricingModelRequestBuilder {
        <GraduatedPercentagePricingModelRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GraduatedPercentagePricingModelRequestBuilder {
    tiers: Option<Vec<GraduatedTier>>,
}

impl GraduatedPercentagePricingModelRequestBuilder {
    pub fn tiers(mut self, value: Vec<GraduatedTier>) -> Self {
        self.tiers = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GraduatedPercentagePricingModelRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`tiers`](GraduatedPercentagePricingModelRequestBuilder::tiers)
    pub fn build(self) -> Result<GraduatedPercentagePricingModelRequest, BuildError> {
        Ok(GraduatedPercentagePricingModelRequest {
            tiers: self
                .tiers
                .ok_or_else(|| BuildError::missing_field("tiers"))?,
        })
    }
}
