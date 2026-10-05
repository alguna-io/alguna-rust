pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct TaxCalculationResponse {
    /// Currency of the amounts
    pub currency: String,
    /// When the calculation stops being issuable
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub expires_at: DateTime<FixedOffset>,
    /// Id of the persisted calculation; pass it when issuing a receipt to pin these results
    pub id: String,
    /// Per-line tax breakdown
    pub line_items: Vec<TaxCalculationLineResponse>,
    /// Sub-merchant the calculation was made for; a document issued from it inherits this attribution
    #[serde(skip_serializing_if = "Option::is_none")]
    pub merchant_id: Option<String>,
    /// Sum of per-line tax amounts
    pub total_tax: String,
}

impl TaxCalculationResponse {
    pub fn builder() -> TaxCalculationResponseBuilder {
        <TaxCalculationResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TaxCalculationResponseBuilder {
    currency: Option<String>,
    expires_at: Option<DateTime<FixedOffset>>,
    id: Option<String>,
    line_items: Option<Vec<TaxCalculationLineResponse>>,
    merchant_id: Option<String>,
    total_tax: Option<String>,
}

impl TaxCalculationResponseBuilder {
    pub fn currency(mut self, value: impl Into<String>) -> Self {
        self.currency = Some(value.into());
        self
    }

    pub fn expires_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.expires_at = Some(value);
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn line_items(mut self, value: Vec<TaxCalculationLineResponse>) -> Self {
        self.line_items = Some(value);
        self
    }

    pub fn merchant_id(mut self, value: impl Into<String>) -> Self {
        self.merchant_id = Some(value.into());
        self
    }

    pub fn total_tax(mut self, value: impl Into<String>) -> Self {
        self.total_tax = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`TaxCalculationResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`currency`](TaxCalculationResponseBuilder::currency)
    /// - [`expires_at`](TaxCalculationResponseBuilder::expires_at)
    /// - [`id`](TaxCalculationResponseBuilder::id)
    /// - [`line_items`](TaxCalculationResponseBuilder::line_items)
    /// - [`total_tax`](TaxCalculationResponseBuilder::total_tax)
    pub fn build(self) -> Result<TaxCalculationResponse, BuildError> {
        Ok(TaxCalculationResponse {
            currency: self
                .currency
                .ok_or_else(|| BuildError::missing_field("currency"))?,
            expires_at: self
                .expires_at
                .ok_or_else(|| BuildError::missing_field("expires_at"))?,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            line_items: self
                .line_items
                .ok_or_else(|| BuildError::missing_field("line_items"))?,
            merchant_id: self.merchant_id,
            total_tax: self
                .total_tax
                .ok_or_else(|| BuildError::missing_field("total_tax"))?,
        })
    }
}
