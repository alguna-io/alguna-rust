pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct VolumePercentagePricingModelRequest {
    /// Optional fixed fee
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fixed_fee: Option<String>,
    /// Percentage rate
    pub percentage: String,
    /// Price per unit
    pub price_per_unit: String,
}

impl VolumePercentagePricingModelRequest {
    pub fn builder() -> VolumePercentagePricingModelRequestBuilder {
        <VolumePercentagePricingModelRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct VolumePercentagePricingModelRequestBuilder {
    fixed_fee: Option<String>,
    percentage: Option<String>,
    price_per_unit: Option<String>,
}

impl VolumePercentagePricingModelRequestBuilder {
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

    /// Consumes the builder and constructs a [`VolumePercentagePricingModelRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`percentage`](VolumePercentagePricingModelRequestBuilder::percentage)
    /// - [`price_per_unit`](VolumePercentagePricingModelRequestBuilder::price_per_unit)
    pub fn build(self) -> Result<VolumePercentagePricingModelRequest, BuildError> {
        Ok(VolumePercentagePricingModelRequest {
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
