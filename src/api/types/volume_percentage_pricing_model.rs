pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct VolumePercentagePricingModel {
    /// Optional fixed fee
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fixed_fee: Option<String>,
    /// Percentage rate
    pub percentage: String,
    /// Price per unit
    pub price_per_unit: String,
}

impl VolumePercentagePricingModel {
    pub fn builder() -> VolumePercentagePricingModelBuilder {
        <VolumePercentagePricingModelBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct VolumePercentagePricingModelBuilder {
    fixed_fee: Option<String>,
    percentage: Option<String>,
    price_per_unit: Option<String>,
}

impl VolumePercentagePricingModelBuilder {
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

    /// Consumes the builder and constructs a [`VolumePercentagePricingModel`].
    /// This method will fail if any of the following fields are not set:
    /// - [`percentage`](VolumePercentagePricingModelBuilder::percentage)
    /// - [`price_per_unit`](VolumePercentagePricingModelBuilder::price_per_unit)
    pub fn build(self) -> Result<VolumePercentagePricingModel, BuildError> {
        Ok(VolumePercentagePricingModel {
            fixed_fee: self.fixed_fee,
            percentage: self
                .percentage
                .ok_or_else(|| BuildError::missing_field("percentage"))?,
            price_per_unit: self
                .price_per_unit
                .ok_or_else(|| BuildError::missing_field("price_per_unit"))?,
        })
    }
}
