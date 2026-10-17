pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct DiscountRequest {
    /// Discount amount
    pub amount: String,
    /// Type of discount
    pub discount_type: DiscountType,
    /// Duration type
    pub duration_type: DurationType,
    /// Duration unit (required if duration_type is fixed)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration_unit: Option<String>,
    /// Duration value (required if duration_type is fixed)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration_value: Option<i64>,
}

impl DiscountRequest {
    pub fn builder() -> DiscountRequestBuilder {
        <DiscountRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DiscountRequestBuilder {
    amount: Option<String>,
    discount_type: Option<DiscountType>,
    duration_type: Option<DurationType>,
    duration_unit: Option<String>,
    duration_value: Option<i64>,
}

impl DiscountRequestBuilder {
    pub fn amount(mut self, value: impl Into<String>) -> Self {
        self.amount = Some(value.into());
        self
    }

    pub fn discount_type(mut self, value: DiscountType) -> Self {
        self.discount_type = Some(value);
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

    /// Consumes the builder and constructs a [`DiscountRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`amount`](DiscountRequestBuilder::amount)
    /// - [`discount_type`](DiscountRequestBuilder::discount_type)
    /// - [`duration_type`](DiscountRequestBuilder::duration_type)
    pub fn build(self) -> Result<DiscountRequest, BuildError> {
        Ok(DiscountRequest {
            amount: self
                .amount
                .ok_or_else(|| BuildError::missing_field("amount"))?,
            discount_type: self
                .discount_type
                .ok_or_else(|| BuildError::missing_field("discount_type"))?,
            duration_type: self
                .duration_type
                .ok_or_else(|| BuildError::missing_field("duration_type"))?,
            duration_unit: self.duration_unit,
            duration_value: self.duration_value,
        })
    }
}
