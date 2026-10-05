pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct VolumePercentagePricingModelAdjust {
    /// Optional fixed fee
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fixed_fee: Option<String>,
    /// Percentage rate
    #[serde(skip_serializing_if = "Option::is_none")]
    pub percentage: Option<String>,
    /// Base price per unit
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price_per_unit: Option<String>,
}

impl VolumePercentagePricingModelAdjust {
    pub fn builder() -> VolumePercentagePricingModelAdjustBuilder {
        <VolumePercentagePricingModelAdjustBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct VolumePercentagePricingModelAdjustBuilder {
    fixed_fee: Option<String>,
    percentage: Option<String>,
    price_per_unit: Option<String>,
}

impl VolumePercentagePricingModelAdjustBuilder {
    pub fn fixed_fee(mut self, value: impl Into<String>) -> Self {
        self.fixed_fee = Some(value.into());
        self
    }

    pub fn percentage(mut self, value: impl Into<String>) -> Self {
        self.percentage = Some(value.into());
        self
    }

    pub fn price_per_unit(mut self, value: impl Into<String>) -> Self {
        self.price_per_unit = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`VolumePercentagePricingModelAdjust`].
    pub fn build(self) -> Result<VolumePercentagePricingModelAdjust, BuildError> {
        Ok(VolumePercentagePricingModelAdjust {
            fixed_fee: self.fixed_fee,
            percentage: self.percentage,
            price_per_unit: self.price_per_unit,
        })
    }
}
