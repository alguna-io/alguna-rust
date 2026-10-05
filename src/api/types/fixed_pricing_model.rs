pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct FixedPricingModel {
    /// Price per unit
    pub price_per_unit: String,
    /// Total price
    pub total: String,
    /// Number of units
    pub units: i64,
}

impl FixedPricingModel {
    pub fn builder() -> FixedPricingModelBuilder {
        <FixedPricingModelBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct FixedPricingModelBuilder {
    price_per_unit: Option<String>,
    total: Option<String>,
    units: Option<i64>,
}

impl FixedPricingModelBuilder {
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

    /// Consumes the builder and constructs a [`FixedPricingModel`].
    /// This method will fail if any of the following fields are not set:
    /// - [`price_per_unit`](FixedPricingModelBuilder::price_per_unit)
    /// - [`total`](FixedPricingModelBuilder::total)
    /// - [`units`](FixedPricingModelBuilder::units)
    pub fn build(self) -> Result<FixedPricingModel, BuildError> {
        Ok(FixedPricingModel {
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
