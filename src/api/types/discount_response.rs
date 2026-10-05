pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct DiscountResponse {
    /// Discount amount
    pub amount: String,
    /// Duration type
    pub duration_type: DurationType,
    /// Duration unit
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration_unit: Option<String>,
    /// Duration value
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration_value: Option<i64>,
    /// Discount type
    pub r#type: DiscountType,
}

impl DiscountResponse {
    pub fn builder() -> DiscountResponseBuilder {
        <DiscountResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DiscountResponseBuilder {
    amount: Option<String>,
    duration_type: Option<DurationType>,
    duration_unit: Option<String>,
    duration_value: Option<i64>,
    r#type: Option<DiscountType>,
}

impl DiscountResponseBuilder {
    pub fn amount(mut self, value: impl Into<String>) -> Self {
        self.amount = Some(value.into());
        self
    }

    pub fn duration_type(mut self, value: DurationType) -> Self {
        self.duration_type = Some(value);
        self
    }

    pub fn duration_unit(mut self, value: impl Into<String>) -> Self {
        self.duration_unit = Some(value.into());
        self
    }

    pub fn duration_value(mut self, value: i64) -> Self {
        self.duration_value = Some(value);
        self
    }

    pub fn r#type(mut self, value: DiscountType) -> Self {
        self.r#type = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`DiscountResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`amount`](DiscountResponseBuilder::amount)
    /// - [`duration_type`](DiscountResponseBuilder::duration_type)
    /// - [`r#type`](DiscountResponseBuilder::r#type)
    pub fn build(self) -> Result<DiscountResponse, BuildError> {
        Ok(DiscountResponse {
            amount: self
                .amount
                .ok_or_else(|| BuildError::missing_field("amount"))?,
            duration_type: self
                .duration_type
                .ok_or_else(|| BuildError::missing_field("duration_type"))?,
            duration_unit: self.duration_unit,
            duration_value: self.duration_value,
            r#type: self
                .r#type
                .ok_or_else(|| BuildError::missing_field("r#type"))?,
        })
    }
}
