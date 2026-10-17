pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GraduatedPercentagePricingModelResponse {
    /// Percentage-based pricing tiers (each tier priced independently)
    pub tiers: Vec<PercentageTierResponse>,
}

impl GraduatedPercentagePricingModelResponse {
    pub fn builder() -> GraduatedPercentagePricingModelResponseBuilder {
        <GraduatedPercentagePricingModelResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GraduatedPercentagePricingModelResponseBuilder {
    tiers: Option<Vec<PercentageTierResponse>>,
}

impl GraduatedPercentagePricingModelResponseBuilder {
    pub fn tiers(mut self, value: Vec<PercentageTierResponse>) -> Self {
        self.tiers = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GraduatedPercentagePricingModelResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`tiers`](GraduatedPercentagePricingModelResponseBuilder::tiers)
    pub fn build(self) -> Result<GraduatedPercentagePricingModelResponse, BuildError> {
        Ok(GraduatedPercentagePricingModelResponse {
            tiers: self
                .tiers
                .ok_or_else(|| BuildError::missing_field("tiers"))?,
        })
    }
}
