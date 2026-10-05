pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct MatrixTieredPercentagePricingModelResponse {
    /// Percentage-based pricing tiers (first matching tier applies)
    pub tiers: Vec<MatrixPercentageTierResponse>,
}

impl MatrixTieredPercentagePricingModelResponse {
    pub fn builder() -> MatrixTieredPercentagePricingModelResponseBuilder {
        <MatrixTieredPercentagePricingModelResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct MatrixTieredPercentagePricingModelResponseBuilder {
    tiers: Option<Vec<MatrixPercentageTierResponse>>,
}

impl MatrixTieredPercentagePricingModelResponseBuilder {
    pub fn tiers(mut self, value: Vec<MatrixPercentageTierResponse>) -> Self {
        self.tiers = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`MatrixTieredPercentagePricingModelResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`tiers`](MatrixTieredPercentagePricingModelResponseBuilder::tiers)
    pub fn build(self) -> Result<MatrixTieredPercentagePricingModelResponse, BuildError> {
        Ok(MatrixTieredPercentagePricingModelResponse {
            tiers: self
                .tiers
                .ok_or_else(|| BuildError::missing_field("tiers"))?,
        })
    }
}
