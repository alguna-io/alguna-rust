pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct MatrixPricingTierResponse {
    /// Fixed fee for this tier
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fixed_fee: Option<String>,
    /// Upper bound of the tier; omitted when unlimited
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_units: Option<i64>,
    /// Lower bound of the tier (inclusive)
    pub min_units: i64,
    /// Price per unit in this tier
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price_per_unit: Option<String>,
    /// Committed per-unit rate when present
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_price: Option<String>,
}

impl MatrixPricingTierResponse {
    pub fn builder() -> MatrixPricingTierResponseBuilder {
        <MatrixPricingTierResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct MatrixPricingTierResponseBuilder {
    fixed_fee: Option<String>,
    max_units: Option<i64>,
    min_units: Option<i64>,
    price_per_unit: Option<String>,
    unit_price: Option<String>,
}

impl MatrixPricingTierResponseBuilder {
    pub fn fixed_fee(mut self, value: impl Into<String>) -> Self {
        self.fixed_fee = Some(value.into());
        self
    }

    pub fn max_units(mut self, value: i64) -> Self {
        self.max_units = Some(value);
        self
    }

    pub fn min_units(mut self, value: i64) -> Self {
        self.min_units = Some(value);
        self
    }

    pub fn price_per_unit(mut self, value: impl Into<String>) -> Self {
        self.price_per_unit = Some(value.into());
        self
    }

    pub fn unit_price(mut self, value: impl Into<String>) -> Self {
        self.unit_price = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`MatrixPricingTierResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`min_units`](MatrixPricingTierResponseBuilder::min_units)
    pub fn build(self) -> Result<MatrixPricingTierResponse, BuildError> {
        Ok(MatrixPricingTierResponse {
            fixed_fee: self.fixed_fee,
            max_units: self.max_units,
            min_units: self
                .min_units
                .ok_or_else(|| BuildError::missing_field("min_units"))?,
            price_per_unit: self.price_per_unit,
            unit_price: self.unit_price,
        })
    }
}
