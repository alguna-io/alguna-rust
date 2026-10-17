pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct FixedPricingModelInput {
    /// Price per unit
    pub price_per_unit: String,
    /// Number of units
    pub units: i64,
}

impl FixedPricingModelInput {
    pub fn builder() -> FixedPricingModelInputBuilder {
        <FixedPricingModelInputBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct FixedPricingModelInputBuilder {
    price_per_unit: Option<String>,
    units: Option<i64>,
}

impl FixedPricingModelInputBuilder {
    pub fn price_per_unit(mut self, value: impl Into<String>) -> Self {
        self.price_per_unit = Some(value.into());
        self
    }

    pub fn units(mut self, value: i64) -> Self {
        self.units = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`FixedPricingModelInput`].
    /// This method will fail if any of the following fields are not set:
    /// - [`price_per_unit`](FixedPricingModelInputBuilder::price_per_unit)
    /// - [`units`](FixedPricingModelInputBuilder::units)
    pub fn build(self) -> Result<FixedPricingModelInput, BuildError> {
        Ok(FixedPricingModelInput {
            price_per_unit: self
                .price_per_unit
                .ok_or_else(|| BuildError::missing_field("price_per_unit"))?,
            units: self
                .units
                .ok_or_else(|| BuildError::missing_field("units"))?,
        })
    }
}
