pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UnitPricingModelAdjust {
    /// Price per unit
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price_per_unit: Option<String>,
}

impl UnitPricingModelAdjust {
    pub fn builder() -> UnitPricingModelAdjustBuilder {
        <UnitPricingModelAdjustBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UnitPricingModelAdjustBuilder {
    price_per_unit: Option<String>,
}

impl UnitPricingModelAdjustBuilder {
    pub fn price_per_unit(mut self, value: impl Into<String>) -> Self {
        self.price_per_unit = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`UnitPricingModelAdjust`].
    pub fn build(self) -> Result<UnitPricingModelAdjust, BuildError> {
        Ok(UnitPricingModelAdjust {
            price_per_unit: self.price_per_unit,
        })
    }
}
