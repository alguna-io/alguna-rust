pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct TaxCalculationDetailLineResponse {
    /// Per-unit amount sent for this line
    pub amount: String,
    /// End of the service period sent for this line, exclusive
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_period_end: Option<DateTime<FixedOffset>>,
    /// Start of the service period sent for this line, inclusive
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_period_start: Option<DateTime<FixedOffset>>,
    /// Description sent for this line
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Quantity sent for this line
    pub quantity: String,
    /// Reference sent for this line (or the generated one)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference: Option<String>,
    /// Tax amount for the line
    pub tax_amount: String,
    /// Canonical tax code the line was taxed under
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_code: Option<String>,
    /// Combined tax rate applied across jurisdictions, as a percentage (8.25 = 8.25%)
    pub tax_rate: String,
    /// Reason determining how tax was applied
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_reason: Option<TaxabilityReason>,
    /// Type of tax applied
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_type: Option<TaxType>,
    /// Portion of the line the tax rate was applied to
    pub taxable_amount: String,
}

impl TaxCalculationDetailLineResponse {
    pub fn builder() -> TaxCalculationDetailLineResponseBuilder {
        <TaxCalculationDetailLineResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TaxCalculationDetailLineResponseBuilder {
    amount: Option<String>,
    billing_period_end: Option<DateTime<FixedOffset>>,
    billing_period_start: Option<DateTime<FixedOffset>>,
    description: Option<String>,
    quantity: Option<String>,
    reference: Option<String>,
    tax_amount: Option<String>,
    tax_code: Option<String>,
    tax_rate: Option<String>,
    tax_reason: Option<TaxabilityReason>,
    tax_type: Option<TaxType>,
    taxable_amount: Option<String>,
}

impl TaxCalculationDetailLineResponseBuilder {
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

    pub fn tax_amount(mut self, value: impl Into<String>) -> Self {
        self.tax_amount = Some(value.into());
        self
    }

    pub fn tax_code(mut self, value: impl Into<String>) -> Self {
        self.tax_code = Some(value.into());
        self
    }

    pub fn tax_rate(mut self, value: impl Into<String>) -> Self {
        self.tax_rate = Some(value.into());
        self
    }

    pub fn tax_reason(mut self, value: TaxabilityReason) -> Self {
        self.tax_reason = Some(value);
        self
    }

    pub fn tax_type(mut self, value: TaxType) -> Self {
        self.tax_type = Some(value);
        self
    }

    pub fn taxable_amount(mut self, value: impl Into<String>) -> Self {
        self.taxable_amount = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`TaxCalculationDetailLineResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`amount`](TaxCalculationDetailLineResponseBuilder::amount)
    /// - [`quantity`](TaxCalculationDetailLineResponseBuilder::quantity)
    /// - [`tax_amount`](TaxCalculationDetailLineResponseBuilder::tax_amount)
    /// - [`tax_rate`](TaxCalculationDetailLineResponseBuilder::tax_rate)
    /// - [`taxable_amount`](TaxCalculationDetailLineResponseBuilder::taxable_amount)
    pub fn build(self) -> Result<TaxCalculationDetailLineResponse, BuildError> {
        Ok(TaxCalculationDetailLineResponse {
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
            tax_amount: self
                .tax_amount
                .ok_or_else(|| BuildError::missing_field("tax_amount"))?,
            tax_code: self.tax_code,
            tax_rate: self
                .tax_rate
                .ok_or_else(|| BuildError::missing_field("tax_rate"))?,
            tax_reason: self.tax_reason,
            tax_type: self.tax_type,
            taxable_amount: self
                .taxable_amount
                .ok_or_else(|| BuildError::missing_field("taxable_amount"))?,
        })
    }
}
