pub use crate::prelude::*;

/// Query parameters for list
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct InvoicesListQueryRequest {
    /// Filter by autopay flag
    #[serde(skip_serializing_if = "Option::is_none")]
    pub autopay: Option<bool>,
    /// Filter by customer ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_id: Option<String>,
    /// Due date lower bound (inclusive)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub due_date_gte: Option<DateTime<FixedOffset>>,
    /// Due date upper bound (inclusive)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub due_date_lte: Option<DateTime<FixedOffset>>,
    /// Filter by external reference
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_reference: Option<String>,
    /// Invoicing date lower bound (inclusive)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub invoicing_date_gte: Option<DateTime<FixedOffset>>,
    /// Invoicing date upper bound (inclusive)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub invoicing_date_lte: Option<DateTime<FixedOffset>>,
    /// Issue date lower bound (inclusive)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub issue_date_gte: Option<DateTime<FixedOffset>>,
    /// Issue date upper bound (inclusive)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub issue_date_lte: Option<DateTime<FixedOffset>>,
    /// Number of items to return per page
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// Number of items to skip
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<i64>,
    /// Paid date lower bound (inclusive)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub paid_date_gte: Option<DateTime<FixedOffset>>,
    /// Paid date upper bound (inclusive)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub paid_date_lte: Option<DateTime<FixedOffset>>,
    /// Filter by invoice reference number
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference: Option<String>,
    /// Sort field and direction in format field:order. Sortable fields: created_at, invoicing_date, issue_date, due_date, paid_date, account_name, status
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort: Option<String>,
    /// Filter by invoice status. Pass a comma-separated list to match any.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<Vec<ListInvoicesRequestStatusItem>>,
    /// Filter by subscription ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription_id: Option<String>,
    /// Filter by tag IDs (comma-separated to match any)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_ids: Option<Vec<String>>,
    /// Filter by tax calculation ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_calculation_id: Option<String>,
}

impl InvoicesListQueryRequest {
    pub fn builder() -> InvoicesListQueryRequestBuilder {
        <InvoicesListQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct InvoicesListQueryRequestBuilder {
    autopay: Option<bool>,
    customer_id: Option<String>,
    due_date_gte: Option<DateTime<FixedOffset>>,
    due_date_lte: Option<DateTime<FixedOffset>>,
    external_reference: Option<String>,
    invoicing_date_gte: Option<DateTime<FixedOffset>>,
    invoicing_date_lte: Option<DateTime<FixedOffset>>,
    issue_date_gte: Option<DateTime<FixedOffset>>,
    issue_date_lte: Option<DateTime<FixedOffset>>,
    limit: Option<i64>,
    offset: Option<i64>,
    paid_date_gte: Option<DateTime<FixedOffset>>,
    paid_date_lte: Option<DateTime<FixedOffset>>,
    reference: Option<String>,
    sort: Option<String>,
    status: Option<Vec<ListInvoicesRequestStatusItem>>,
    subscription_id: Option<String>,
    tag_ids: Option<Vec<String>>,
    tax_calculation_id: Option<String>,
}

impl InvoicesListQueryRequestBuilder {
    pub fn autopay(mut self, value: bool) -> Self {
        self.autopay = Some(value);
        self
    }

    pub fn customer_id(mut self, value: impl Into<String>) -> Self {
        self.customer_id = Some(value.into());
        self
    }

    pub fn due_date_gte(mut self, value: DateTime<FixedOffset>) -> Self {
        self.due_date_gte = Some(value);
        self
    }

    pub fn due_date_lte(mut self, value: DateTime<FixedOffset>) -> Self {
        self.due_date_lte = Some(value);
        self
    }

    pub fn external_reference(mut self, value: impl Into<String>) -> Self {
        self.external_reference = Some(value.into());
        self
    }

    pub fn invoicing_date_gte(mut self, value: DateTime<FixedOffset>) -> Self {
        self.invoicing_date_gte = Some(value);
        self
    }

    pub fn invoicing_date_lte(mut self, value: DateTime<FixedOffset>) -> Self {
        self.invoicing_date_lte = Some(value);
        self
    }

    pub fn issue_date_gte(mut self, value: DateTime<FixedOffset>) -> Self {
        self.issue_date_gte = Some(value);
        self
    }

    pub fn issue_date_lte(mut self, value: DateTime<FixedOffset>) -> Self {
        self.issue_date_lte = Some(value);
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

    pub fn paid_date_gte(mut self, value: DateTime<FixedOffset>) -> Self {
        self.paid_date_gte = Some(value);
        self
    }

    pub fn paid_date_lte(mut self, value: DateTime<FixedOffset>) -> Self {
        self.paid_date_lte = Some(value);
        self
    }

    pub fn reference(mut self, value: impl Into<String>) -> Self {
        self.reference = Some(value.into());
        self
    }

    pub fn sort(mut self, value: impl Into<String>) -> Self {
        self.sort = Some(value.into());
        self
    }

    pub fn status(mut self, value: Vec<ListInvoicesRequestStatusItem>) -> Self {
        self.status = Some(value);
        self
    }

    pub fn subscription_id(mut self, value: impl Into<String>) -> Self {
        self.subscription_id = Some(value.into());
        self
    }

    pub fn tag_ids(mut self, value: Vec<String>) -> Self {
        self.tag_ids = Some(value);
        self
    }

    pub fn tax_calculation_id(mut self, value: impl Into<String>) -> Self {
        self.tax_calculation_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`InvoicesListQueryRequest`].
    pub fn build(self) -> Result<InvoicesListQueryRequest, BuildError> {
        Ok(InvoicesListQueryRequest {
            autopay: self.autopay,
            customer_id: self.customer_id,
            due_date_gte: self.due_date_gte,
            due_date_lte: self.due_date_lte,
            external_reference: self.external_reference,
            invoicing_date_gte: self.invoicing_date_gte,
            invoicing_date_lte: self.invoicing_date_lte,
            issue_date_gte: self.issue_date_gte,
            issue_date_lte: self.issue_date_lte,
            limit: self.limit,
            offset: self.offset,
            paid_date_gte: self.paid_date_gte,
            paid_date_lte: self.paid_date_lte,
            reference: self.reference,
            sort: self.sort,
            status: self.status,
            subscription_id: self.subscription_id,
            tag_ids: self.tag_ids,
            tax_calculation_id: self.tax_calculation_id,
        })
    }
}
