pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreateInvoiceLineItemRequest {
    /// End of the service period this line covers, exclusive: for a period through 31 January send 2026-02-01T00:00:00Z. Send with billing_period_start.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_period_end: Option<DateTime<FixedOffset>>,
    /// Start of the service period this line covers, inclusive. Send with billing_period_end.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_period_start: Option<DateTime<FixedOffset>>,
    /// Description of the line item
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Associated product identifier
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_id: Option<String>,
    /// Quantity of the line item
    pub quantity: String,
    /// Unit price of the line item, excluding tax. Tax is calculated on top of it.
    pub unit_price: String,
}

impl CreateInvoiceLineItemRequest {
    pub fn builder() -> CreateInvoiceLineItemRequestBuilder {
        <CreateInvoiceLineItemRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateInvoiceLineItemRequestBuilder {
    billing_period_end: Option<DateTime<FixedOffset>>,
    billing_period_start: Option<DateTime<FixedOffset>>,
    description: Option<String>,
    product_id: Option<String>,
    quantity: Option<String>,
    unit_price: Option<String>,
}

impl CreateInvoiceLineItemRequestBuilder {
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

    pub fn product_id(mut self, value: impl Into<String>) -> Self {
        self.product_id = Some(value.into());
        self
    }

    pub fn quantity(mut self, value: impl Into<String>) -> Self {
        self.quantity = Some(value.into());
        self
    }

    pub fn unit_price(mut self, value: impl Into<String>) -> Self {
        self.unit_price = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CreateInvoiceLineItemRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`quantity`](CreateInvoiceLineItemRequestBuilder::quantity)
    /// - [`unit_price`](CreateInvoiceLineItemRequestBuilder::unit_price)
    pub fn build(self) -> Result<CreateInvoiceLineItemRequest, BuildError> {
        Ok(CreateInvoiceLineItemRequest {
            billing_period_end: self.billing_period_end,
            billing_period_start: self.billing_period_start,
            description: self.description,
            product_id: self.product_id,
            quantity: self
                .quantity
                .ok_or_else(|| BuildError::missing_field("quantity"))?,
            unit_price: self
                .unit_price
                .ok_or_else(|| BuildError::missing_field("unit_price"))?,
        })
    }
}
