pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct TieredPercentagePricingModelAdjust {
    /// Percentage tiers (replaces entirely if provided)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tiers: Option<Vec<PercentageTierInput>>,
}

impl TieredPercentagePricingModelAdjust {
    pub fn builder() -> TieredPercentagePricingModelAdjustBuilder {
        <TieredPercentagePricingModelAdjustBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TieredPercentagePricingModelAdjustBuilder {
    tiers: Option<Vec<PercentageTierInput>>,
}

impl TieredPercentagePricingModelAdjustBuilder {
    pub fn tiers(mut self, value: Vec<PercentageTierInput>) -> Self {
        self.tiers = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TieredPercentagePricingModelAdjust`].
    pub fn build(self) -> Result<TieredPercentagePricingModelAdjust, BuildError> {
        Ok(TieredPercentagePricingModelAdjust { tiers: self.tiers })
    }
}
