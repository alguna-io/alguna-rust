pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreditNoteLineItemResponse {
    /// Additional description for the line item
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_description: Option<String>,
    /// End of the billing period
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub billing_period_end: DateTime<FixedOffset>,
    /// Start of the billing period
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub billing_period_start: DateTime<FixedOffset>,
    /// Description of the line item
    pub description: String,
    /// Unique identifier for the line item
    pub id: String,
    /// Associated invoice line item ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoice_line_item_id: Option<String>,
    /// Whether the line item can be edited
    pub is_editable: bool,
    /// Associated product ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_id: Option<String>,
    /// Quantity of items
    pub quantity: String,
    /// Tax amount for the line item
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_amount: Option<String>,
    /// Tax rate applied to the line item
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_rate: Option<String>,
    /// Total price of the line item
    pub total_price: String,
    /// Unit price of the line item, excluding tax
    pub unit_price: String,
}

impl CreditNoteLineItemResponse {
    pub fn builder() -> CreditNoteLineItemResponseBuilder {
        <CreditNoteLineItemResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreditNoteLineItemResponseBuilder {
    additional_description: Option<String>,
    billing_period_end: Option<DateTime<FixedOffset>>,
    billing_period_start: Option<DateTime<FixedOffset>>,
    description: Option<String>,
    id: Option<String>,
    invoice_line_item_id: Option<String>,
    is_editable: Option<bool>,
    product_id: Option<String>,
    quantity: Option<String>,
    tax_amount: Option<String>,
    tax_rate: Option<String>,
    total_price: Option<String>,
    unit_price: Option<String>,
}

impl CreditNoteLineItemResponseBuilder {
    pub fn additional_description(mut self, value: impl Into<String>) -> Self {
        self.additional_description = Some(value.into());
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

    pub fn invoice_line_item_id(mut self, value: impl Into<String>) -> Self {
        self.invoice_line_item_id = Some(value.into());
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

    /// Consumes the builder and constructs a [`CreditNoteLineItemResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`billing_period_end`](CreditNoteLineItemResponseBuilder::billing_period_end)
    /// - [`billing_period_start`](CreditNoteLineItemResponseBuilder::billing_period_start)
    /// - [`description`](CreditNoteLineItemResponseBuilder::description)
    /// - [`id`](CreditNoteLineItemResponseBuilder::id)
    /// - [`is_editable`](CreditNoteLineItemResponseBuilder::is_editable)
    /// - [`quantity`](CreditNoteLineItemResponseBuilder::quantity)
    /// - [`total_price`](CreditNoteLineItemResponseBuilder::total_price)
    /// - [`unit_price`](CreditNoteLineItemResponseBuilder::unit_price)
    pub fn build(self) -> Result<CreditNoteLineItemResponse, BuildError> {
        Ok(CreditNoteLineItemResponse {
            additional_description: self.additional_description,
            billing_period_end: self
                .billing_period_end
                .ok_or_else(|| BuildError::missing_field("billing_period_end"))?,
            billing_period_start: self
                .billing_period_start
                .ok_or_else(|| BuildError::missing_field("billing_period_start"))?,
            description: self
                .description
                .ok_or_else(|| BuildError::missing_field("description"))?,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            invoice_line_item_id: self.invoice_line_item_id,
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
