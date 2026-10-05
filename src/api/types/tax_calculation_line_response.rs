pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct TaxCalculationLineResponse {
    /// End of the service period sent for this line, exclusive. The invoice minted from this calculation prints the preceding day.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_period_end: Option<DateTime<FixedOffset>>,
    /// Start of the service period sent for this line, inclusive
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_period_start: Option<DateTime<FixedOffset>>,
    /// Echoes the reference from the matching request line (or the generated one)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference: Option<String>,
    /// Tax amount for the line
    pub tax_amount: String,
    /// Combined tax rate applied across jurisdictions, as a percentage (8.25 = 8.25%). Where a jurisdiction taxes only part of the line, tax_amount reflects that taxable portion, so tax_rate multiplied by the line amount may exceed tax_amount.
    pub tax_rate: String,
    /// Reason determining how tax was applied
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_reason: Option<TaxabilityReason>,
    /// Type of tax applied
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_type: Option<TaxType>,
    /// Portion of the line the tax rate was applied to. Equals the line total unless a jurisdiction taxes only part of it (Texas taxes 80% of SaaS); zero when no tax was applied.
    pub taxable_amount: String,
}

impl TaxCalculationLineResponse {
    pub fn builder() -> TaxCalculationLineResponseBuilder {
        <TaxCalculationLineResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TaxCalculationLineResponseBuilder {
    billing_period_end: Option<DateTime<FixedOffset>>,
    billing_period_start: Option<DateTime<FixedOffset>>,
    reference: Option<String>,
    tax_amount: Option<String>,
    tax_rate: Option<String>,
    tax_reason: Option<TaxabilityReason>,
    tax_type: Option<TaxType>,
    taxable_amount: Option<String>,
}

impl TaxCalculationLineResponseBuilder {
    pub fn billing_period_end(mut self, value: DateTime<FixedOffset>) -> Self {
        self.billing_period_end = Some(value);
        self
    }

    pub fn billing_period_start(mut self, value: DateTime<FixedOffset>) -> Self {
        self.billing_period_start = Some(value);
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

    /// Consumes the builder and constructs a [`TaxCalculationLineResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`tax_amount`](TaxCalculationLineResponseBuilder::tax_amount)
    /// - [`tax_rate`](TaxCalculationLineResponseBuilder::tax_rate)
    /// - [`taxable_amount`](TaxCalculationLineResponseBuilder::taxable_amount)
    pub fn build(self) -> Result<TaxCalculationLineResponse, BuildError> {
        Ok(TaxCalculationLineResponse {
            billing_period_end: self.billing_period_end,
            billing_period_start: self.billing_period_start,
            reference: self.reference,
            tax_amount: self
                .tax_amount
                .ok_or_else(|| BuildError::missing_field("tax_amount"))?,
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
