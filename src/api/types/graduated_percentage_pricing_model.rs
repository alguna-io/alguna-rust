pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GraduatedPercentagePricingModel {
    /// Pricing tiers
    pub tiers: Vec<GraduatedTier>,
}

impl GraduatedPercentagePricingModel {
    pub fn builder() -> GraduatedPercentagePricingModelBuilder {
        <GraduatedPercentagePricingModelBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GraduatedPercentagePricingModelBuilder {
    tiers: Option<Vec<GraduatedTier>>,
}

impl GraduatedPercentagePricingModelBuilder {
    pub fn tiers(mut self, value: Vec<GraduatedTier>) -> Self {
        self.tiers = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GraduatedPercentagePricingModel`].
    /// This method will fail if any of the following fields are not set:
    /// - [`tiers`](GraduatedPercentagePricingModelBuilder::tiers)
    pub fn build(self) -> Result<GraduatedPercentagePricingModel, BuildError> {
        Ok(GraduatedPercentagePricingModel {
            tiers: self
                .tiers
                .ok_or_else(|| BuildError::missing_field("tiers"))?,
        })
    }
}
