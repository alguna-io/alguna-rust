pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct LineItemResponse {
    /// Amount for this line before tax (quantity x unit price)
    pub amount_excluding_tax: String,
    /// Amount for this line including its tax. This is the figure a full credit of the line returns.
    pub amount_including_tax: String,
    /// End of the service period this line covers, exclusive
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_period_end: Option<DateTime<FixedOffset>>,
    /// Start of the service period this line covers, inclusive
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_period_start: Option<DateTime<FixedOffset>>,
    /// Description of the line item
    pub description: String,
    /// Unique identifier for the line item
    pub id: String,
    /// Whether this line item can be manually edited
    pub is_editable: bool,
    /// Associated product identifier
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_id: Option<String>,
    /// Quantity of the line item
    pub quantity: String,
    /// Tax charged on this line
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_amount: Option<String>,
    /// Percentage rate the tax was charged at
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_rate: Option<String>,
    /// Deprecated: use amount_excluding_tax, which carries the same value. Despite the name this excludes tax.
    pub total_price: String,
    /// Unit price of the line item, excluding tax
    pub unit_price: String,
}

impl LineItemResponse {
    pub fn builder() -> LineItemResponseBuilder {
        <LineItemResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct LineItemResponseBuilder {
    amount_excluding_tax: Option<String>,
    amount_including_tax: Option<String>,
    billing_period_end: Option<DateTime<FixedOffset>>,
    billing_period_start: Option<DateTime<FixedOffset>>,
    description: Option<String>,
    id: Option<String>,
    is_editable: Option<bool>,
    product_id: Option<String>,
    quantity: Option<String>,
    tax_amount: Option<String>,
    tax_rate: Option<String>,
    total_price: Option<String>,
    unit_price: Option<String>,
}

impl LineItemResponseBuilder {
    pub fn amount_excluding_tax(mut self, value: impl Into<String>) -> Self {
        self.amount_excluding_tax = Some(value.into());
        self
    }

    pub fn amount_including_tax(mut self, value: impl Into<String>) -> Self {
        self.amount_including_tax = Some(value.into());
        self
    }

    pub fn billing_period_end(mut self, value: DateTime<FixedOffset>) -> Self {
        self.billing_period_end = Some(value);
        self
    }

    pub fn billing_period_start(mut self, value: DateTime<FixedOffset>) -> Self {
        self.billing_period_start = Some(value);
        self
    }

    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn is_editable(mut self, value: bool) -> Self {
        self.is_editable = Some(value);
        self
    }

    pub fn product_id(mut self, value: impl Into<String>) -> Self {
        self.product_id = Some(value.into());
        self
    }

    pub fn quantity(mut self, value: impl Into<String>) -> Self {
        self.quantity = Some(value.into());
        self
    }

    pub fn tax_amount(mut self, value: impl Into<String>) -> Self {
        self.tax_amount = Some(value.into());
        self
    }

    pub fn tax_rate(mut self, value: impl Into<String>) -> Self {
        self.tax_rate = Some(value.into());
        self
    }

    pub fn total_price(mut self, value: impl Into<String>) -> Self {
        self.total_price = Some(value.into());
        self
    }

    pub fn unit_price(mut self, value: impl Into<String>) -> Self {
        self.unit_price = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`LineItemResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`amount_excluding_tax`](LineItemResponseBuilder::amount_excluding_tax)
    /// - [`amount_including_tax`](LineItemResponseBuilder::amount_including_tax)
    /// - [`description`](LineItemResponseBuilder::description)
    /// - [`id`](LineItemResponseBuilder::id)
    /// - [`is_editable`](LineItemResponseBuilder::is_editable)
    /// - [`quantity`](LineItemResponseBuilder::quantity)
    /// - [`total_price`](LineItemResponseBuilder::total_price)
    /// - [`unit_price`](LineItemResponseBuilder::unit_price)
    pub fn build(self) -> Result<LineItemResponse, BuildError> {
        Ok(LineItemResponse {
            amount_excluding_tax: self
                .amount_excluding_tax
                .ok_or_else(|| BuildError::missing_field("amount_excluding_tax"))?,
            amount_including_tax: self
                .amount_including_tax
                .ok_or_else(|| BuildError::missing_field("amount_including_tax"))?,
            billing_period_end: self.billing_period_end,
            billing_period_start: self.billing_period_start,
            description: self
                .description
                .ok_or_else(|| BuildError::missing_field("description"))?,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            is_editable: self
                .is_editable
                .ok_or_else(|| BuildError::missing_field("is_editable"))?,
            product_id: self.product_id,
            quantity: self
                .quantity
                .ok_or_else(|| BuildError::missing_field("quantity"))?,
            tax_amount: self.tax_amount,
            tax_rate: self.tax_rate,
            total_price: self
                .total_price
                .ok_or_else(|| BuildError::missing_field("total_price"))?,
            unit_price: self
                .unit_price
                .ok_or_else(|| BuildError::missing_field("unit_price"))?,
        })
    }
}
