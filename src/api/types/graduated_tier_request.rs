pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GraduatedTierRequest {
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

impl GraduatedTierRequest {
    pub fn builder() -> GraduatedTierRequestBuilder {
        <GraduatedTierRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GraduatedTierRequestBuilder {
    fixed_fee: Option<String>,
    max_units: Option<i64>,
    min_units: Option<i64>,
    percentage: Option<String>,
}

impl GraduatedTierRequestBuilder {
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

    /// Consumes the builder and constructs a [`GraduatedTierRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`min_units`](GraduatedTierRequestBuilder::min_units)
    pub fn build(self) -> Result<GraduatedTierRequest, BuildError> {
        Ok(GraduatedTierRequest {
            fixed_fee: self.fixed_fee,
            max_units: self.max_units,
            min_units: self
                .min_units
                .ok_or_else(|| BuildError::missing_field("min_units"))?,
            percentage: self.percentage,
        })
    }
}
