pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct TaxCalculationLineItemRequest {
    /// Per-unit amount; line total is amount * quantity
    pub amount: String,
    /// End of the service period this line covers, exclusive: for a period through 31 January send 2026-02-01T00:00:00Z, which the invoice prints as 31 January. Carried onto the invoice line minted from this calculation. Send with billing_period_start.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_period_end: Option<DateTime<FixedOffset>>,
    /// Start of the service period this line covers, inclusive. Carried onto the invoice line minted from this calculation. Send with billing_period_end.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_period_start: Option<DateTime<FixedOffset>>,
    /// Human-readable line description
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Quantity
    pub quantity: String,
    /// Your reference for the line; echoed back on the matching result line. Generated when omitted.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference: Option<String>,
    /// Canonical tax code; defaults to general when omitted
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_code: Option<String>,
}

impl TaxCalculationLineItemRequest {
    pub fn builder() -> TaxCalculationLineItemRequestBuilder {
        <TaxCalculationLineItemRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TaxCalculationLineItemRequestBuilder {
    amount: Option<String>,
    billing_period_end: Option<DateTime<FixedOffset>>,
    billing_period_start: Option<DateTime<FixedOffset>>,
    description: Option<String>,
    quantity: Option<String>,
    reference: Option<String>,
    tax_code: Option<String>,
}

impl TaxCalculationLineItemRequestBuilder {
    pub fn amount(mut self, value: impl Into<String>) -> Self {
        self.amount = Some(value.into());
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

    pub fn quantity(mut self, value: impl Into<String>) -> Self {
        self.quantity = Some(value.into());
        self
    }

    pub fn reference(mut self, value: impl Into<String>) -> Self {
        self.reference = Some(value.into());
        self
    }

    pub fn tax_code(mut self, value: impl Into<String>) -> Self {
        self.tax_code = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`TaxCalculationLineItemRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`amount`](TaxCalculationLineItemRequestBuilder::amount)
    /// - [`quantity`](TaxCalculationLineItemRequestBuilder::quantity)
    pub fn build(self) -> Result<TaxCalculationLineItemRequest, BuildError> {
        Ok(TaxCalculationLineItemRequest {
            amount: self
                .amount
                .ok_or_else(|| BuildError::missing_field("amount"))?,
            billing_period_end: self.billing_period_end,
            billing_period_start: self.billing_period_start,
            description: self.description,
            quantity: self
                .quantity
                .ok_or_else(|| BuildError::missing_field("quantity"))?,
            reference: self.reference,
            tax_code: self.tax_code,
        })
    }
}
