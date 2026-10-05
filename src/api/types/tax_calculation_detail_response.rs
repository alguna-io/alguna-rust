pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct TaxCalculationDetailResponse {
    /// When an invoice was issued from this calculation; a calculation issues once
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consumed_at: Option<DateTime<FixedOffset>>,
    /// When the calculation was made
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub created_at: DateTime<FixedOffset>,
    /// Currency of the amounts
    pub currency: String,
    /// Your own id for the buyer, as sent; absent when the calculation was for a guest
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_reference: Option<String>,
    /// Whether the calculation can no longer be issued from
    pub expired: bool,
    /// When the calculation stops being issuable
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub expires_at: DateTime<FixedOffset>,
    /// Id of the calculation
    pub id: String,
    /// Invoice this calculation was issued into, when it has been
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoice_id: Option<String>,
    /// Selling legal entity whose tax engine produced the calculation
    pub legal_entity_id: String,
    /// Per-line breakdown behind the total
    pub line_items: Vec<TaxCalculationDetailLineResponse>,
    /// Sub-merchant the calculation was made for; a document issued from it inherits this attribution
    #[serde(skip_serializing_if = "Option::is_none")]
    pub merchant_id: Option<String>,
    /// Date tax was assessed on; an invoice issued from this calculation is taxed as of this date
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub tax_date: DateTime<FixedOffset>,
    /// Sum of per-line tax amounts
    pub total_tax: String,
}

impl TaxCalculationDetailResponse {
    pub fn builder() -> TaxCalculationDetailResponseBuilder {
        <TaxCalculationDetailResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TaxCalculationDetailResponseBuilder {
    consumed_at: Option<DateTime<FixedOffset>>,
    created_at: Option<DateTime<FixedOffset>>,
    currency: Option<String>,
    customer_reference: Option<String>,
    expired: Option<bool>,
    expires_at: Option<DateTime<FixedOffset>>,
    id: Option<String>,
    invoice_id: Option<String>,
    legal_entity_id: Option<String>,
    line_items: Option<Vec<TaxCalculationDetailLineResponse>>,
    merchant_id: Option<String>,
    tax_date: Option<DateTime<FixedOffset>>,
    total_tax: Option<String>,
}

impl TaxCalculationDetailResponseBuilder {
    pub fn consumed_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.consumed_at = Some(value);
        self
    }

    pub fn created_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.created_at = Some(value);
        self
    }

    pub fn currency(mut self, value: impl Into<String>) -> Self {
        self.currency = Some(value.into());
        self
    }

    pub fn customer_reference(mut self, value: impl Into<String>) -> Self {
        self.customer_reference = Some(value.into());
        self
    }

    pub fn expired(mut self, value: bool) -> Self {
        self.expired = Some(value);
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

    pub fn invoice_id(mut self, value: impl Into<String>) -> Self {
        self.invoice_id = Some(value.into());
        self
    }

    pub fn legal_entity_id(mut self, value: impl Into<String>) -> Self {
        self.legal_entity_id = Some(value.into());
        self
    }

    pub fn line_items(mut self, value: Vec<TaxCalculationDetailLineResponse>) -> Self {
        self.line_items = Some(value);
        self
    }

    pub fn merchant_id(mut self, value: impl Into<String>) -> Self {
        self.merchant_id = Some(value.into());
        self
    }

    pub fn tax_date(mut self, value: DateTime<FixedOffset>) -> Self {
        self.tax_date = Some(value);
        self
    }

    pub fn total_tax(mut self, value: impl Into<String>) -> Self {
        self.total_tax = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`TaxCalculationDetailResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`created_at`](TaxCalculationDetailResponseBuilder::created_at)
    /// - [`currency`](TaxCalculationDetailResponseBuilder::currency)
    /// - [`expired`](TaxCalculationDetailResponseBuilder::expired)
    /// - [`expires_at`](TaxCalculationDetailResponseBuilder::expires_at)
    /// - [`id`](TaxCalculationDetailResponseBuilder::id)
    /// - [`legal_entity_id`](TaxCalculationDetailResponseBuilder::legal_entity_id)
    /// - [`line_items`](TaxCalculationDetailResponseBuilder::line_items)
    /// - [`tax_date`](TaxCalculationDetailResponseBuilder::tax_date)
    /// - [`total_tax`](TaxCalculationDetailResponseBuilder::total_tax)
    pub fn build(self) -> Result<TaxCalculationDetailResponse, BuildError> {
        Ok(TaxCalculationDetailResponse {
            consumed_at: self.consumed_at,
            created_at: self
                .created_at
                .ok_or_else(|| BuildError::missing_field("created_at"))?,
            currency: self
                .currency
                .ok_or_else(|| BuildError::missing_field("currency"))?,
            customer_reference: self.customer_reference,
            expired: self
                .expired
                .ok_or_else(|| BuildError::missing_field("expired"))?,
            expires_at: self
                .expires_at
                .ok_or_else(|| BuildError::missing_field("expires_at"))?,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            invoice_id: self.invoice_id,
            legal_entity_id: self
                .legal_entity_id
                .ok_or_else(|| BuildError::missing_field("legal_entity_id"))?,
            line_items: self
                .line_items
                .ok_or_else(|| BuildError::missing_field("line_items"))?,
            merchant_id: self.merchant_id,
            tax_date: self
                .tax_date
                .ok_or_else(|| BuildError::missing_field("tax_date"))?,
            total_tax: self
                .total_tax
                .ok_or_else(|| BuildError::missing_field("total_tax"))?,
        })
    }
}
