pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UnitPricingModelInput {
    /// Price per unit
    pub price_per_unit: String,
}

impl UnitPricingModelInput {
    pub fn builder() -> UnitPricingModelInputBuilder {
        <UnitPricingModelInputBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UnitPricingModelInputBuilder {
    price_per_unit: Option<String>,
}

impl UnitPricingModelInputBuilder {
    pub fn price_per_unit(mut self, value: impl Into<String>) -> Self {
        self.price_per_unit = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`UnitPricingModelInput`].
    /// This method will fail if any of the following fields are not set:
    /// - [`price_per_unit`](UnitPricingModelInputBuilder::price_per_unit)
    pub fn build(self) -> Result<UnitPricingModelInput, BuildError> {
        Ok(UnitPricingModelInput {
            price_per_unit: self
                .price_per_unit
                .ok_or_else(|| BuildError::missing_field("price_per_unit"))?,
        })
    }
}
