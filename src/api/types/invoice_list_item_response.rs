pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct InvoiceListItemResponse {
    /// Formatted billing period string
    pub billing_period: String,
    /// End of the billing period
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_period_end: Option<DateTime<FixedOffset>>,
    /// Start of the billing period
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_period_start: Option<DateTime<FixedOffset>>,
    /// Timestamp when the invoice was created
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub created_at: DateTime<FixedOffset>,
    /// ISO 4217 currency code
    pub currency: String,
    /// Customer this invoice belongs to
    pub customer_id: String,
    /// Invoice description or memo
    pub description: String,
    /// Unique identifier for the invoice
    pub id: String,
    /// Current status of the invoice
    pub status: InvoiceStatus,
    /// Id of the tax calculation this invoice was created from, if any
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_calculation_id: Option<String>,
    /// Total amount including tax
    pub total: String,
    /// Timestamp when the invoice was last updated
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub updated_at: DateTime<FixedOffset>,
}

impl InvoiceListItemResponse {
    pub fn builder() -> InvoiceListItemResponseBuilder {
        <InvoiceListItemResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct InvoiceListItemResponseBuilder {
    billing_period: Option<String>,
    billing_period_end: Option<DateTime<FixedOffset>>,
    billing_period_start: Option<DateTime<FixedOffset>>,
    created_at: Option<DateTime<FixedOffset>>,
    currency: Option<String>,
    customer_id: Option<String>,
    description: Option<String>,
    id: Option<String>,
    status: Option<InvoiceStatus>,
    tax_calculation_id: Option<String>,
    total: Option<String>,
    updated_at: Option<DateTime<FixedOffset>>,
}

impl InvoiceListItemResponseBuilder {
    pub fn billing_period(mut self, value: impl Into<String>) -> Self {
        self.billing_period = Some(value.into());
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

    pub fn created_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.created_at = Some(value);
        self
    }

    pub fn currency(mut self, value: impl Into<String>) -> Self {
        self.currency = Some(value.into());
        self
    }

    pub fn customer_id(mut self, value: impl Into<String>) -> Self {
        self.customer_id = Some(value.into());
        self
    }

    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn status(mut self, value: InvoiceStatus) -> Self {
        self.status = Some(value);
        self
    }

    pub fn tax_calculation_id(mut self, value: impl Into<String>) -> Self {
        self.tax_calculation_id = Some(value.into());
        self
    }

    pub fn total(mut self, value: impl Into<String>) -> Self {
        self.total = Some(value.into());
        self
    }

    pub fn updated_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.updated_at = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`InvoiceListItemResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`billing_period`](InvoiceListItemResponseBuilder::billing_period)
    /// - [`created_at`](InvoiceListItemResponseBuilder::created_at)
    /// - [`currency`](InvoiceListItemResponseBuilder::currency)
    /// - [`customer_id`](InvoiceListItemResponseBuilder::customer_id)
    /// - [`description`](InvoiceListItemResponseBuilder::description)
    /// - [`id`](InvoiceListItemResponseBuilder::id)
    /// - [`status`](InvoiceListItemResponseBuilder::status)
    /// - [`total`](InvoiceListItemResponseBuilder::total)
    /// - [`updated_at`](InvoiceListItemResponseBuilder::updated_at)
    pub fn build(self) -> Result<InvoiceListItemResponse, BuildError> {
        Ok(InvoiceListItemResponse {
            billing_period: self
                .billing_period
                .ok_or_else(|| BuildError::missing_field("billing_period"))?,
            billing_period_end: self.billing_period_end,
            billing_period_start: self.billing_period_start,
            created_at: self
                .created_at
                .ok_or_else(|| BuildError::missing_field("created_at"))?,
            currency: self
                .currency
                .ok_or_else(|| BuildError::missing_field("currency"))?,
            customer_id: self
                .customer_id
                .ok_or_else(|| BuildError::missing_field("customer_id"))?,
            description: self
                .description
                .ok_or_else(|| BuildError::missing_field("description"))?,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            status: self
                .status
                .ok_or_else(|| BuildError::missing_field("status"))?,
            tax_calculation_id: self.tax_calculation_id,
            total: self
                .total
                .ok_or_else(|| BuildError::missing_field("total"))?,
            updated_at: self
                .updated_at
                .ok_or_else(|| BuildError::missing_field("updated_at"))?,
        })
    }
}
