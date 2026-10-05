pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct FixedPricingModelAdjust {
    /// Price per unit
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price_per_unit: Option<String>,
    /// Number of units
    #[serde(skip_serializing_if = "Option::is_none")]
    pub units: Option<i64>,
}

impl FixedPricingModelAdjust {
    pub fn builder() -> FixedPricingModelAdjustBuilder {
        <FixedPricingModelAdjustBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct FixedPricingModelAdjustBuilder {
    price_per_unit: Option<String>,
    units: Option<i64>,
}

impl FixedPricingModelAdjustBuilder {
    pub fn price_per_unit(mut self, value: impl Into<String>) -> Self {
        self.price_per_unit = Some(value.into());
        self
    }

    pub fn units(mut self, value: i64) -> Self {
        self.units = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`FixedPricingModelAdjust`].
    pub fn build(self) -> Result<FixedPricingModelAdjust, BuildError> {
        Ok(FixedPricingModelAdjust {
            price_per_unit: self.price_per_unit,
            units: self.units,
        })
    }
}
