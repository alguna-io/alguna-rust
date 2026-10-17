pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreateLineItemRequest {
    /// End of the service period this line covers, exclusive: for a period through 31 January send 2026-02-01T00:00:00Z. Send with billing_period_start.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_period_end: Option<DateTime<FixedOffset>>,
    /// Start of the service period this line covers, inclusive. Send with billing_period_end.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_period_start: Option<DateTime<FixedOffset>>,
    /// Description of the line item
    pub description: String,
    /// Quantity
    pub quantity: String,
    /// Unit price, excluding tax. Tax is calculated on top of it.
    pub unit_price: String,
}

impl CreateLineItemRequest {
    pub fn builder() -> CreateLineItemRequestBuilder {
        <CreateLineItemRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateLineItemRequestBuilder {
    billing_period_end: Option<DateTime<FixedOffset>>,
    billing_period_start: Option<DateTime<FixedOffset>>,
    description: Option<String>,
    quantity: Option<String>,
    unit_price: Option<String>,
}

impl CreateLineItemRequestBuilder {
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

    pub fn quantity(mut self, value: impl Into<String>) -> Self {
        self.quantity = Some(value.into());
        self
    }

    pub fn unit_price(mut self, value: impl Into<String>) -> Self {
        self.unit_price = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CreateLineItemRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`description`](CreateLineItemRequestBuilder::description)
    /// - [`quantity`](CreateLineItemRequestBuilder::quantity)
    /// - [`unit_price`](CreateLineItemRequestBuilder::unit_price)
    pub fn build(self) -> Result<CreateLineItemRequest, BuildError> {
        Ok(CreateLineItemRequest {
            billing_period_end: self.billing_period_end,
            billing_period_start: self.billing_period_start,
            description: self
                .description
                .ok_or_else(|| BuildError::missing_field("description"))?,
            quantity: self
                .quantity
                .ok_or_else(|| BuildError::missing_field("quantity"))?,
            unit_price: self
                .unit_price
                .ok_or_else(|| BuildError::missing_field("unit_price"))?,
        })
    }
}
