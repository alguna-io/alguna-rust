pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct MatrixGraduatedTieredPricingModelResponse {
    /// Pricing tiers (each tier priced independently)
    pub tiers: Vec<MatrixPricingTierResponse>,
    /// Fixed quantity of units (for fixed fee type only)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub units: Option<i64>,
}

impl MatrixGraduatedTieredPricingModelResponse {
    pub fn builder() -> MatrixGraduatedTieredPricingModelResponseBuilder {
        <MatrixGraduatedTieredPricingModelResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct MatrixGraduatedTieredPricingModelResponseBuilder {
    tiers: Option<Vec<MatrixPricingTierResponse>>,
    units: Option<i64>,
}

impl MatrixGraduatedTieredPricingModelResponseBuilder {
    pub fn tiers(mut self, value: Vec<MatrixPricingTierResponse>) -> Self {
        self.tiers = Some(value);
        self
    }

    pub fn units(mut self, value: i64) -> Self {
        self.units = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`MatrixGraduatedTieredPricingModelResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`tiers`](MatrixGraduatedTieredPricingModelResponseBuilder::tiers)
    pub fn build(self) -> Result<MatrixGraduatedTieredPricingModelResponse, BuildError> {
        Ok(MatrixGraduatedTieredPricingModelResponse {
            tiers: self
                .tiers
                .ok_or_else(|| BuildError::missing_field("tiers"))?,
            units: self.units,
        })
    }
}
