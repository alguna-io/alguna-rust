pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GraduatedPercentagePricingModelInput {
    /// Percentage-based pricing tiers
    pub tiers: Vec<PercentageTierInput>,
}

impl GraduatedPercentagePricingModelInput {
    pub fn builder() -> GraduatedPercentagePricingModelInputBuilder {
        <GraduatedPercentagePricingModelInputBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GraduatedPercentagePricingModelInputBuilder {
    tiers: Option<Vec<PercentageTierInput>>,
}

impl GraduatedPercentagePricingModelInputBuilder {
    pub fn tiers(mut self, value: Vec<PercentageTierInput>) -> Self {
        self.tiers = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GraduatedPercentagePricingModelInput`].
    /// This method will fail if any of the following fields are not set:
    /// - [`tiers`](GraduatedPercentagePricingModelInputBuilder::tiers)
    pub fn build(self) -> Result<GraduatedPercentagePricingModelInput, BuildError> {
        Ok(GraduatedPercentagePricingModelInput {
            tiers: self
                .tiers
                .ok_or_else(|| BuildError::missing_field("tiers"))?,
        })
    }
}
