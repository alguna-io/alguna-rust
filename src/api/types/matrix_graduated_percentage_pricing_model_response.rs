pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct MatrixGraduatedPercentagePricingModelResponse {
    /// Percentage-based pricing tiers (each tier priced independently)
    pub tiers: Vec<MatrixPercentageTierResponse>,
}

impl MatrixGraduatedPercentagePricingModelResponse {
    pub fn builder() -> MatrixGraduatedPercentagePricingModelResponseBuilder {
        <MatrixGraduatedPercentagePricingModelResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct MatrixGraduatedPercentagePricingModelResponseBuilder {
    tiers: Option<Vec<MatrixPercentageTierResponse>>,
}

impl MatrixGraduatedPercentagePricingModelResponseBuilder {
    pub fn tiers(mut self, value: Vec<MatrixPercentageTierResponse>) -> Self {
        self.tiers = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`MatrixGraduatedPercentagePricingModelResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`tiers`](MatrixGraduatedPercentagePricingModelResponseBuilder::tiers)
    pub fn build(self) -> Result<MatrixGraduatedPercentagePricingModelResponse, BuildError> {
        Ok(MatrixGraduatedPercentagePricingModelResponse {
            tiers: self
                .tiers
                .ok_or_else(|| BuildError::missing_field("tiers"))?,
        })
    }
}
