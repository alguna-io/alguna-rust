pub use crate::prelude::*;

/// Query parameters for listCalculations
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListCalculationsQueryRequest {
    /// Filter by whether an invoice has been issued from the calculation
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consumed: Option<bool>,
    /// Filter by currency. Pass a comma-separated list to match any.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<Vec<String>>,
    /// Filter by your own buyer id. Pass a comma-separated list to match any.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_reference: Option<Vec<String>>,
    /// Filter by whether the calculation can still be issued from
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expired: Option<bool>,
    /// Number of items to return per page
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// Number of items to skip
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<i64>,
    /// Sort field and direction in format field:order. Sortable fields: created_at, tax_date, expires_at, total_tax, currency.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort: Option<String>,
    /// Tax date lower bound (inclusive)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub tax_date_gte: Option<DateTime<FixedOffset>>,
    /// Tax date upper bound (inclusive)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub tax_date_lte: Option<DateTime<FixedOffset>>,
}

impl ListCalculationsQueryRequest {
    pub fn builder() -> ListCalculationsQueryRequestBuilder {
        <ListCalculationsQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListCalculationsQueryRequestBuilder {
    consumed: Option<bool>,
    currency: Option<Vec<String>>,
    customer_reference: Option<Vec<String>>,
    expired: Option<bool>,
    limit: Option<i64>,
    offset: Option<i64>,
    sort: Option<String>,
    tax_date_gte: Option<DateTime<FixedOffset>>,
    tax_date_lte: Option<DateTime<FixedOffset>>,
}

impl ListCalculationsQueryRequestBuilder {
    pub fn consumed(mut self, value: bool) -> Self {
        self.consumed = Some(value);
        self
    }

    pub fn currency(mut self, value: Vec<String>) -> Self {
        self.currency = Some(value);
        self
    }

    pub fn customer_reference(mut self, value: Vec<String>) -> Self {
        self.customer_reference = Some(value);
        self
    }

    pub fn expired(mut self, value: bool) -> Self {
        self.expired = Some(value);
        self
    }

    pub fn limit(mut self, value: i64) -> Self {
        self.limit = Some(value);
        self
    }

    pub fn offset(mut self, value: i64) -> Self {
        self.offset = Some(value);
        self
    }

    pub fn sort(mut self, value: impl Into<String>) -> Self {
        self.sort = Some(value.into());
        self
    }

    pub fn tax_date_gte(mut self, value: DateTime<FixedOffset>) -> Self {
        self.tax_date_gte = Some(value);
        self
    }

    pub fn tax_date_lte(mut self, value: DateTime<FixedOffset>) -> Self {
        self.tax_date_lte = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListCalculationsQueryRequest`].
    pub fn build(self) -> Result<ListCalculationsQueryRequest, BuildError> {
        Ok(ListCalculationsQueryRequest {
            consumed: self.consumed,
            currency: self.currency,
            customer_reference: self.customer_reference,
            expired: self.expired,
            limit: self.limit,
            offset: self.offset,
            sort: self.sort,
            tax_date_gte: self.tax_date_gte,
            tax_date_lte: self.tax_date_lte,
        })
    }
}
