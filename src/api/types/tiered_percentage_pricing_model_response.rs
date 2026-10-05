pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct TieredPercentagePricingModelResponse {
    /// Percentage-based pricing tiers (first matching tier applies)
    pub tiers: Vec<PercentageTierResponse>,
}

impl TieredPercentagePricingModelResponse {
    pub fn builder() -> TieredPercentagePricingModelResponseBuilder {
        <TieredPercentagePricingModelResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TieredPercentagePricingModelResponseBuilder {
    tiers: Option<Vec<PercentageTierResponse>>,
}

impl TieredPercentagePricingModelResponseBuilder {
    pub fn tiers(mut self, value: Vec<PercentageTierResponse>) -> Self {
        self.tiers = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TieredPercentagePricingModelResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`tiers`](TieredPercentagePricingModelResponseBuilder::tiers)
    pub fn build(self) -> Result<TieredPercentagePricingModelResponse, BuildError> {
        Ok(TieredPercentagePricingModelResponse {
            tiers: self
                .tiers
                .ok_or_else(|| BuildError::missing_field("tiers"))?,
        })
    }
}
