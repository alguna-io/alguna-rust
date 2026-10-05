pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct TaxCalculationListItemResponse {
    /// When an invoice was issued from this calculation
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
    /// Number of lines on the calculation
    pub line_count: i64,
    /// Date tax was assessed on
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub tax_date: DateTime<FixedOffset>,
    /// Sum of per-line tax amounts
    pub total_tax: String,
}

impl TaxCalculationListItemResponse {
    pub fn builder() -> TaxCalculationListItemResponseBuilder {
        <TaxCalculationListItemResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TaxCalculationListItemResponseBuilder {
    consumed_at: Option<DateTime<FixedOffset>>,
    created_at: Option<DateTime<FixedOffset>>,
    currency: Option<String>,
    customer_reference: Option<String>,
    expired: Option<bool>,
    expires_at: Option<DateTime<FixedOffset>>,
    id: Option<String>,
    invoice_id: Option<String>,
    line_count: Option<i64>,
    tax_date: Option<DateTime<FixedOffset>>,
    total_tax: Option<String>,
}

impl TaxCalculationListItemResponseBuilder {
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

    pub fn line_count(mut self, value: i64) -> Self {
        self.line_count = Some(value);
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

    /// Consumes the builder and constructs a [`TaxCalculationListItemResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`created_at`](TaxCalculationListItemResponseBuilder::created_at)
    /// - [`currency`](TaxCalculationListItemResponseBuilder::currency)
    /// - [`expired`](TaxCalculationListItemResponseBuilder::expired)
    /// - [`expires_at`](TaxCalculationListItemResponseBuilder::expires_at)
    /// - [`id`](TaxCalculationListItemResponseBuilder::id)
    /// - [`line_count`](TaxCalculationListItemResponseBuilder::line_count)
    /// - [`tax_date`](TaxCalculationListItemResponseBuilder::tax_date)
    /// - [`total_tax`](TaxCalculationListItemResponseBuilder::total_tax)
    pub fn build(self) -> Result<TaxCalculationListItemResponse, BuildError> {
        Ok(TaxCalculationListItemResponse {
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
            line_count: self
                .line_count
                .ok_or_else(|| BuildError::missing_field("line_count"))?,
            tax_date: self
                .tax_date
                .ok_or_else(|| BuildError::missing_field("tax_date"))?,
            total_tax: self
                .total_tax
                .ok_or_else(|| BuildError::missing_field("total_tax"))?,
        })
    }
}
