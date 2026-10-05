pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PrepaidTieredPricingModelResponse {
    /// Number of prepaid units included
    pub prepaid_units: i64,
    /// Overage pricing tiers
    pub tiers: Vec<PricingTierResponse>,
}

impl PrepaidTieredPricingModelResponse {
    pub fn builder() -> PrepaidTieredPricingModelResponseBuilder {
        <PrepaidTieredPricingModelResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PrepaidTieredPricingModelResponseBuilder {
    prepaid_units: Option<i64>,
    tiers: Option<Vec<PricingTierResponse>>,
}

impl PrepaidTieredPricingModelResponseBuilder {
    pub fn prepaid_units(mut self, value: i64) -> Self {
        self.prepaid_units = Some(value);
        self
    }

    pub fn tiers(mut self, value: Vec<PricingTierResponse>) -> Self {
        self.tiers = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PrepaidTieredPricingModelResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`prepaid_units`](PrepaidTieredPricingModelResponseBuilder::prepaid_units)
    /// - [`tiers`](PrepaidTieredPricingModelResponseBuilder::tiers)
    pub fn build(self) -> Result<PrepaidTieredPricingModelResponse, BuildError> {
        Ok(PrepaidTieredPricingModelResponse {
            prepaid_units: self
                .prepaid_units
                .ok_or_else(|| BuildError::missing_field("prepaid_units"))?,
            tiers: self
                .tiers
                .ok_or_else(|| BuildError::missing_field("tiers"))?,
        })
    }
}
