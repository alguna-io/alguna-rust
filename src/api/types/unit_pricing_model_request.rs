pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UnitPricingModelRequest {
    /// Price per unit
    pub price_per_unit: String,
}

impl UnitPricingModelRequest {
    pub fn builder() -> UnitPricingModelRequestBuilder {
        <UnitPricingModelRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UnitPricingModelRequestBuilder {
    price_per_unit: Option<String>,
}

impl UnitPricingModelRequestBuilder {
    pub fn price_per_unit(mut self, value: impl Into<String>) -> Self {
        self.price_per_unit = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`UnitPricingModelRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`price_per_unit`](UnitPricingModelRequestBuilder::price_per_unit)
    pub fn build(self) -> Result<UnitPricingModelRequest, BuildError> {
        Ok(UnitPricingModelRequest {
            price_per_unit: self
                .price_per_unit
                .ok_or_else(|| BuildError::missing_field("price_per_unit"))?,
        })
    }
}
