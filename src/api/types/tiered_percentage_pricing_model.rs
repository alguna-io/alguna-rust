pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct TieredPercentagePricingModel {
    /// Pricing tiers
    pub tiers: Vec<GraduatedTier>,
}

impl TieredPercentagePricingModel {
    pub fn builder() -> TieredPercentagePricingModelBuilder {
        <TieredPercentagePricingModelBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TieredPercentagePricingModelBuilder {
    tiers: Option<Vec<GraduatedTier>>,
}

impl TieredPercentagePricingModelBuilder {
    pub fn tiers(mut self, value: Vec<GraduatedTier>) -> Self {
        self.tiers = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TieredPercentagePricingModel`].
    /// This method will fail if any of the following fields are not set:
    /// - [`tiers`](TieredPercentagePricingModelBuilder::tiers)
    pub fn build(self) -> Result<TieredPercentagePricingModel, BuildError> {
        Ok(TieredPercentagePricingModel {
            tiers: self
                .tiers
                .ok_or_else(|| BuildError::missing_field("tiers"))?,
        })
    }
}
