pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct InsightsProductEntry {
    /// Amount for the previous comparison period
    pub comparison_amount: String,
    /// Amount for the selected period
    pub primary_amount: String,
    /// Product ID
    pub product_id: String,
    /// Product name
    pub product_name: String,
    /// Pricing class
    pub product_type: InsightsProductEntryProductType,
}

impl InsightsProductEntry {
    pub fn builder() -> InsightsProductEntryBuilder {
        <InsightsProductEntryBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct InsightsProductEntryBuilder {
    comparison_amount: Option<String>,
    primary_amount: Option<String>,
    product_id: Option<String>,
    product_name: Option<String>,
    product_type: Option<InsightsProductEntryProductType>,
}

impl InsightsProductEntryBuilder {
    pub fn comparison_amount(mut self, value: impl Into<String>) -> Self {
        self.comparison_amount = Some(value.into());
        self
    }

    pub fn primary_amount(mut self, value: impl Into<String>) -> Self {
        self.primary_amount = Some(value.into());
        self
    }

    pub fn product_id(mut self, value: impl Into<String>) -> Self {
        self.product_id = Some(value.into());
        self
    }

    pub fn product_name(mut self, value: impl Into<String>) -> Self {
        self.product_name = Some(value.into());
        self
    }

    pub fn product_type(mut self, value: InsightsProductEntryProductType) -> Self {
        self.product_type = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`InsightsProductEntry`].
    /// This method will fail if any of the following fields are not set:
    /// - [`comparison_amount`](InsightsProductEntryBuilder::comparison_amount)
    /// - [`primary_amount`](InsightsProductEntryBuilder::primary_amount)
    /// - [`product_id`](InsightsProductEntryBuilder::product_id)
    /// - [`product_name`](InsightsProductEntryBuilder::product_name)
    /// - [`product_type`](InsightsProductEntryBuilder::product_type)
    pub fn build(self) -> Result<InsightsProductEntry, BuildError> {
        Ok(InsightsProductEntry {
            comparison_amount: self
                .comparison_amount
                .ok_or_else(|| BuildError::missing_field("comparison_amount"))?,
            primary_amount: self
                .primary_amount
                .ok_or_else(|| BuildError::missing_field("primary_amount"))?,
            product_id: self
                .product_id
                .ok_or_else(|| BuildError::missing_field("product_id"))?,
            product_name: self
                .product_name
                .ok_or_else(|| BuildError::missing_field("product_name"))?,
            product_type: self
                .product_type
                .ok_or_else(|| BuildError::missing_field("product_type"))?,
        })
    }
}
