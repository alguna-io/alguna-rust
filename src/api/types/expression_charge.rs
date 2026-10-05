pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct ExpressionCharge {
    /// Billing direction
    pub billing_direction: BillingDirection,
    /// Charge description
    pub description: String,
    /// Expression for calculating quantity
    pub quantity_expression: String,
    /// Expression for calculating unit price
    pub unit_price_expression: String,
}

impl ExpressionCharge {
    pub fn builder() -> ExpressionChargeBuilder {
        <ExpressionChargeBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ExpressionChargeBuilder {
    billing_direction: Option<BillingDirection>,
    description: Option<String>,
    quantity_expression: Option<String>,
    unit_price_expression: Option<String>,
}

impl ExpressionChargeBuilder {
    pub fn billing_direction(mut self, value: BillingDirection) -> Self {
        self.billing_direction = Some(value);
        self
    }

    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
        self
    }

    pub fn quantity_expression(mut self, value: impl Into<String>) -> Self {
        self.quantity_expression = Some(value.into());
        self
    }

    pub fn unit_price_expression(mut self, value: impl Into<String>) -> Self {
        self.unit_price_expression = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ExpressionCharge`].
    /// This method will fail if any of the following fields are not set:
    /// - [`billing_direction`](ExpressionChargeBuilder::billing_direction)
    /// - [`description`](ExpressionChargeBuilder::description)
    /// - [`quantity_expression`](ExpressionChargeBuilder::quantity_expression)
    /// - [`unit_price_expression`](ExpressionChargeBuilder::unit_price_expression)
    pub fn build(self) -> Result<ExpressionCharge, BuildError> {
        Ok(ExpressionCharge {
            billing_direction: self
                .billing_direction
                .ok_or_else(|| BuildError::missing_field("billing_direction"))?,
            description: self
                .description
                .ok_or_else(|| BuildError::missing_field("description"))?,
            quantity_expression: self
                .quantity_expression
                .ok_or_else(|| BuildError::missing_field("quantity_expression"))?,
            unit_price_expression: self
                .unit_price_expression
                .ok_or_else(|| BuildError::missing_field("unit_price_expression"))?,
        })
    }
}
