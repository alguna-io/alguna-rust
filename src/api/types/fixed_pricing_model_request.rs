pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct FixedPricingModelRequest {
    /// Price per unit
    pub price_per_unit: String,
    /// Total price
    pub total: String,
    /// Number of units
    pub units: i64,
}

impl FixedPricingModelRequest {
    pub fn builder() -> FixedPricingModelRequestBuilder {
        <FixedPricingModelRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct FixedPricingModelRequestBuilder {
    price_per_unit: Option<String>,
    total: Option<String>,
    units: Option<i64>,
}

impl FixedPricingModelRequestBuilder {
    pub fn price_per_unit(mut self, value: impl Into<String>) -> Self {
        self.price_per_unit = Some(value.into());
        self
    }

    pub fn total(mut self, value: impl Into<String>) -> Self {
        self.total = Some(value.into());
        self
    }

    pub fn units(mut self, value: i64) -> Self {
        self.units = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`FixedPricingModelRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`price_per_unit`](FixedPricingModelRequestBuilder::price_per_unit)
    /// - [`total`](FixedPricingModelRequestBuilder::total)
    /// - [`units`](FixedPricingModelRequestBuilder::units)
    pub fn build(self) -> Result<FixedPricingModelRequest, BuildError> {
        Ok(FixedPricingModelRequest {
            price_per_unit: self
                .price_per_unit
                .ok_or_else(|| BuildError::missing_field("price_per_unit"))?,
            total: self
                .total
                .ok_or_else(|| BuildError::missing_field("total"))?,
            units: self
                .units
                .ok_or_else(|| BuildError::missing_field("units"))?,
        })
    }
}
