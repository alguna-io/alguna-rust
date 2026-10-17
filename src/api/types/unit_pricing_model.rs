pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UnitPricingModel {
    /// Price per unit
    pub price_per_unit: String,
}

impl UnitPricingModel {
    pub fn builder() -> UnitPricingModelBuilder {
        <UnitPricingModelBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UnitPricingModelBuilder {
    price_per_unit: Option<String>,
}

impl UnitPricingModelBuilder {
    pub fn price_per_unit(mut self, value: impl Into<String>) -> Self {
        self.price_per_unit = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`UnitPricingModel`].
    /// This method will fail if any of the following fields are not set:
    /// - [`price_per_unit`](UnitPricingModelBuilder::price_per_unit)
    pub fn build(self) -> Result<UnitPricingModel, BuildError> {
        Ok(UnitPricingModel {
            price_per_unit: self
                .price_per_unit
                .ok_or_else(|| BuildError::missing_field("price_per_unit"))?,
        })
    }
}
