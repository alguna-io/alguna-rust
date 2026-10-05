pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CheckoutLineItemRequest {
    /// Line item description shown to the customer
    pub description: String,
    /// Product name shown alongside the description
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_name: Option<String>,
    /// Quantity
    pub quantity: String,
    /// Price per unit
    pub unit_price: String,
}

impl CheckoutLineItemRequest {
    pub fn builder() -> CheckoutLineItemRequestBuilder {
        <CheckoutLineItemRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CheckoutLineItemRequestBuilder {
    description: Option<String>,
    product_name: Option<String>,
    quantity: Option<String>,
    unit_price: Option<String>,
}

impl CheckoutLineItemRequestBuilder {
    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
        self
    }

    pub fn product_name(mut self, value: impl Into<String>) -> Self {
        self.product_name = Some(value.into());
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

    /// Consumes the builder and constructs a [`CheckoutLineItemRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`description`](CheckoutLineItemRequestBuilder::description)
    /// - [`quantity`](CheckoutLineItemRequestBuilder::quantity)
    /// - [`unit_price`](CheckoutLineItemRequestBuilder::unit_price)
    pub fn build(self) -> Result<CheckoutLineItemRequest, BuildError> {
        Ok(CheckoutLineItemRequest {
            description: self
                .description
                .ok_or_else(|| BuildError::missing_field("description"))?,
            product_name: self.product_name,
            quantity: self
                .quantity
                .ok_or_else(|| BuildError::missing_field("quantity"))?,
            unit_price: self
                .unit_price
                .ok_or_else(|| BuildError::missing_field("unit_price"))?,
        })
    }
}
