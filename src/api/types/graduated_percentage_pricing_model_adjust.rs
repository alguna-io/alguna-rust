pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GraduatedPercentagePricingModelAdjust {
    /// Percentage tiers (replaces entirely if provided)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tiers: Option<Vec<PercentageTierInput>>,
}

impl GraduatedPercentagePricingModelAdjust {
    pub fn builder() -> GraduatedPercentagePricingModelAdjustBuilder {
        <GraduatedPercentagePricingModelAdjustBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GraduatedPercentagePricingModelAdjustBuilder {
    tiers: Option<Vec<PercentageTierInput>>,
}

impl GraduatedPercentagePricingModelAdjustBuilder {
    pub fn tiers(mut self, value: Vec<PercentageTierInput>) -> Self {
        self.tiers = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GraduatedPercentagePricingModelAdjust`].
    pub fn build(self) -> Result<GraduatedPercentagePricingModelAdjust, BuildError> {
        Ok(GraduatedPercentagePricingModelAdjust { tiers: self.tiers })
    }
}
