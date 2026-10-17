pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PrepaidTieredPricingModelAdjust {
    /// Number of prepaid units
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prepaid_units: Option<i64>,
    /// Overage pricing tiers (replaces entirely if provided)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tiers: Option<Vec<PricingTierInput>>,
}

impl PrepaidTieredPricingModelAdjust {
    pub fn builder() -> PrepaidTieredPricingModelAdjustBuilder {
        <PrepaidTieredPricingModelAdjustBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PrepaidTieredPricingModelAdjustBuilder {
    prepaid_units: Option<i64>,
    tiers: Option<Vec<PricingTierInput>>,
}

impl PrepaidTieredPricingModelAdjustBuilder {
    pub fn prepaid_units(mut self, value: i64) -> Self {
        self.prepaid_units = Some(value);
        self
    }

    pub fn tiers(mut self, value: Vec<PricingTierInput>) -> Self {
        self.tiers = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PrepaidTieredPricingModelAdjust`].
    pub fn build(self) -> Result<PrepaidTieredPricingModelAdjust, BuildError> {
        Ok(PrepaidTieredPricingModelAdjust {
            prepaid_units: self.prepaid_units,
            tiers: self.tiers,
        })
    }
}
