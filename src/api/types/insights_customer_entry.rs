pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct InsightsCustomerEntry {
    /// Amount for the previous comparison period
    pub comparison_amount: String,
    /// URL of the customer's icon
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_icon_url: Option<String>,
    /// Customer ID
    pub customer_id: String,
    /// Customer name
    pub customer_name: String,
    /// Amount for the selected period
    pub primary_amount: String,
}

impl InsightsCustomerEntry {
    pub fn builder() -> InsightsCustomerEntryBuilder {
        <InsightsCustomerEntryBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct InsightsCustomerEntryBuilder {
    comparison_amount: Option<String>,
    customer_icon_url: Option<String>,
    customer_id: Option<String>,
    customer_name: Option<String>,
    primary_amount: Option<String>,
}

impl InsightsCustomerEntryBuilder {
    pub fn comparison_amount(mut self, value: impl Into<String>) -> Self {
        self.comparison_amount = Some(value.into());
        self
    }

    pub fn customer_icon_url(mut self, value: impl Into<String>) -> Self {
        self.customer_icon_url = Some(value.into());
        self
    }

    pub fn customer_id(mut self, value: impl Into<String>) -> Self {
        self.customer_id = Some(value.into());
        self
    }

    pub fn customer_name(mut self, value: impl Into<String>) -> Self {
        self.customer_name = Some(value.into());
        self
    }

    pub fn primary_amount(mut self, value: impl Into<String>) -> Self {
        self.primary_amount = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`InsightsCustomerEntry`].
    /// This method will fail if any of the following fields are not set:
    /// - [`comparison_amount`](InsightsCustomerEntryBuilder::comparison_amount)
    /// - [`customer_id`](InsightsCustomerEntryBuilder::customer_id)
    /// - [`customer_name`](InsightsCustomerEntryBuilder::customer_name)
    /// - [`primary_amount`](InsightsCustomerEntryBuilder::primary_amount)
    pub fn build(self) -> Result<InsightsCustomerEntry, BuildError> {
        Ok(InsightsCustomerEntry {
            comparison_amount: self
                .comparison_amount
                .ok_or_else(|| BuildError::missing_field("comparison_amount"))?,
            customer_icon_url: self.customer_icon_url,
            customer_id: self
                .customer_id
                .ok_or_else(|| BuildError::missing_field("customer_id"))?,
            customer_name: self
                .customer_name
                .ok_or_else(|| BuildError::missing_field("customer_name"))?,
            primary_amount: self
                .primary_amount
                .ok_or_else(|| BuildError::missing_field("primary_amount"))?,
        })
    }
}
