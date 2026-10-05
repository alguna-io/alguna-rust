pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct VolumePercentagePricingModelInput {
    /// Optional fixed fee
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fixed_fee: Option<String>,
    /// Percentage rate
    pub percentage: String,
    /// Base price per unit
    pub price_per_unit: String,
}

impl VolumePercentagePricingModelInput {
    pub fn builder() -> VolumePercentagePricingModelInputBuilder {
        <VolumePercentagePricingModelInputBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct VolumePercentagePricingModelInputBuilder {
    fixed_fee: Option<String>,
    percentage: Option<String>,
    price_per_unit: Option<String>,
}

impl VolumePercentagePricingModelInputBuilder {
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

    /// Consumes the builder and constructs a [`VolumePercentagePricingModelInput`].
    /// This method will fail if any of the following fields are not set:
    /// - [`percentage`](VolumePercentagePricingModelInputBuilder::percentage)
    /// - [`price_per_unit`](VolumePercentagePricingModelInputBuilder::price_per_unit)
    pub fn build(self) -> Result<VolumePercentagePricingModelInput, BuildError> {
        Ok(VolumePercentagePricingModelInput {
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
