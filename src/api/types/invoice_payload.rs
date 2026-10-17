pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct InvoicePayload {
    /// Customer account the invoice belongs to
    #[serde(rename = "accountId")]
    pub account_id: String,
    /// End of the billing period covered, RFC 3339
    #[serde(rename = "billingPeriodEnd")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_period_end: Option<DateTime<FixedOffset>>,
    /// Start of the billing period covered, RFC 3339
    #[serde(rename = "billingPeriodStart")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_period_start: Option<DateTime<FixedOffset>>,
    /// When the invoice was created, RFC 3339
    #[serde(rename = "createdAt")]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub created_at: DateTime<FixedOffset>,
    /// ISO 4217 currency code
    pub currency: String,
    /// Invoice description
    pub description: String,
    /// Unique identifier for the invoice
    pub id: String,
    /// Invoice status
    pub status: InvoiceStatus,
    /// Invoice total, formatted to the currency's precision
    pub total: String,
    /// When the invoice was last updated, RFC 3339
    #[serde(rename = "updatedAt")]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub updated_at: DateTime<FixedOffset>,
}

impl InvoicePayload {
    pub fn builder() -> InvoicePayloadBuilder {
        <InvoicePayloadBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct InvoicePayloadBuilder {
    account_id: Option<String>,
    billing_period_end: Option<DateTime<FixedOffset>>,
    billing_period_start: Option<DateTime<FixedOffset>>,
    created_at: Option<DateTime<FixedOffset>>,
    currency: Option<String>,
    description: Option<String>,
    id: Option<String>,
    status: Option<InvoiceStatus>,
    total: Option<String>,
    updated_at: Option<DateTime<FixedOffset>>,
}

impl InvoicePayloadBuilder {
    pub fn account_id(mut self, value: impl Into<String>) -> Self {
        self.account_id = Some(value.into());
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

    pub fn total(mut self, value: impl Into<String>) -> Self {
        self.total = Some(value.into());
        self
    }

    pub fn updated_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.updated_at = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`InvoicePayload`].
    /// This method will fail if any of the following fields are not set:
    /// - [`account_id`](InvoicePayloadBuilder::account_id)
    /// - [`created_at`](InvoicePayloadBuilder::created_at)
    /// - [`currency`](InvoicePayloadBuilder::currency)
    /// - [`description`](InvoicePayloadBuilder::description)
    /// - [`id`](InvoicePayloadBuilder::id)
    /// - [`status`](InvoicePayloadBuilder::status)
    /// - [`total`](InvoicePayloadBuilder::total)
    /// - [`updated_at`](InvoicePayloadBuilder::updated_at)
    pub fn build(self) -> Result<InvoicePayload, BuildError> {
        Ok(InvoicePayload {
            account_id: self
                .account_id
                .ok_or_else(|| BuildError::missing_field("account_id"))?,
            billing_period_end: self.billing_period_end,
            billing_period_start: self.billing_period_start,
            created_at: self
                .created_at
                .ok_or_else(|| BuildError::missing_field("created_at"))?,
            currency: self
                .currency
                .ok_or_else(|| BuildError::missing_field("currency"))?,
            description: self
                .description
                .ok_or_else(|| BuildError::missing_field("description"))?,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            status: self
                .status
                .ok_or_else(|| BuildError::missing_field("status"))?,
            total: self
                .total
                .ok_or_else(|| BuildError::missing_field("total"))?,
            updated_at: self
                .updated_at
                .ok_or_else(|| BuildError::missing_field("updated_at"))?,
        })
    }
}
