pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct MatrixTieredPricingModelResponse {
    /// Pricing tiers (first matching tier applies)
    pub tiers: Vec<MatrixPricingTierResponse>,
    /// Fixed quantity of units (for fixed fee type only)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub units: Option<i64>,
}

impl MatrixTieredPricingModelResponse {
    pub fn builder() -> MatrixTieredPricingModelResponseBuilder {
        <MatrixTieredPricingModelResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct MatrixTieredPricingModelResponseBuilder {
    tiers: Option<Vec<MatrixPricingTierResponse>>,
    units: Option<i64>,
}

impl MatrixTieredPricingModelResponseBuilder {
    pub fn tiers(mut self, value: Vec<MatrixPricingTierResponse>) -> Self {
        self.tiers = Some(value);
        self
    }

    pub fn units(mut self, value: i64) -> Self {
        self.units = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`MatrixTieredPricingModelResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`tiers`](MatrixTieredPricingModelResponseBuilder::tiers)
    pub fn build(self) -> Result<MatrixTieredPricingModelResponse, BuildError> {
        Ok(MatrixTieredPricingModelResponse {
            tiers: self
                .tiers
                .ok_or_else(|| BuildError::missing_field("tiers"))?,
            units: self.units,
        })
    }
}
