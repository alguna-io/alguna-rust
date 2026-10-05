pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct VolumePercentagePricingModelResponse {
    /// Optional fixed fee
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fixed_fee: Option<String>,
    /// Percentage rate
    pub percentage: String,
    /// Base price per unit
    pub price_per_unit: String,
}

impl VolumePercentagePricingModelResponse {
    pub fn builder() -> VolumePercentagePricingModelResponseBuilder {
        <VolumePercentagePricingModelResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct VolumePercentagePricingModelResponseBuilder {
    fixed_fee: Option<String>,
    percentage: Option<String>,
    price_per_unit: Option<String>,
}

impl VolumePercentagePricingModelResponseBuilder {
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

    /// Consumes the builder and constructs a [`VolumePercentagePricingModelResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`percentage`](VolumePercentagePricingModelResponseBuilder::percentage)
    /// - [`price_per_unit`](VolumePercentagePricingModelResponseBuilder::price_per_unit)
    pub fn build(self) -> Result<VolumePercentagePricingModelResponse, BuildError> {
        Ok(VolumePercentagePricingModelResponse {
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
