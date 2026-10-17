pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GraduatedTier {
    /// Fixed fee for this tier
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fixed_fee: Option<String>,
    /// Maximum units for this tier
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_units: Option<i64>,
    /// Minimum units for this tier
    pub min_units: i64,
    /// Percentage for this tier
    #[serde(skip_serializing_if = "Option::is_none")]
    pub percentage: Option<String>,
}

impl GraduatedTier {
    pub fn builder() -> GraduatedTierBuilder {
        <GraduatedTierBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GraduatedTierBuilder {
    fixed_fee: Option<String>,
    max_units: Option<i64>,
    min_units: Option<i64>,
    percentage: Option<String>,
}

impl GraduatedTierBuilder {
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

    pub fn percentage(mut self, value: impl Into<String>) -> Self {
        self.percentage = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`GraduatedTier`].
    /// This method will fail if any of the following fields are not set:
    /// - [`min_units`](GraduatedTierBuilder::min_units)
    pub fn build(self) -> Result<GraduatedTier, BuildError> {
        Ok(GraduatedTier {
            fixed_fee: self.fixed_fee,
            max_units: self.max_units,
            min_units: self
                .min_units
                .ok_or_else(|| BuildError::missing_field("min_units"))?,
            percentage: self.percentage,
        })
    }
}
