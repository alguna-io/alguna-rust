pub use crate::prelude::*;

/// Query parameters for list
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct SubscriptionsListQueryRequest {
    /// Filter by customer ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_id: Option<String>,
    /// Number of items per page
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// Number of items to skip
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<i64>,
    /// Sort field and direction in format field:order. Sortable fields: name, status, created_at, activated_at, ended_at, updated_at, contract_start_date, contract_end_date
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort: Option<String>,
    /// Filter by subscription status. Pass a comma-separated list to match any.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<Vec<ListSubscriptionsRequestStatusItem>>,
    /// Filter by tag IDs (comma-separated to match any)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_ids: Option<Vec<String>>,
}

impl SubscriptionsListQueryRequest {
    pub fn builder() -> SubscriptionsListQueryRequestBuilder {
        <SubscriptionsListQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SubscriptionsListQueryRequestBuilder {
    customer_id: Option<String>,
    limit: Option<i64>,
    offset: Option<i64>,
    sort: Option<String>,
    status: Option<Vec<ListSubscriptionsRequestStatusItem>>,
    tag_ids: Option<Vec<String>>,
}

impl SubscriptionsListQueryRequestBuilder {
    pub fn customer_id(mut self, value: impl Into<String>) -> Self {
        self.customer_id = Some(value.into());
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

    pub fn status(mut self, value: Vec<ListSubscriptionsRequestStatusItem>) -> Self {
        self.status = Some(value);
        self
    }

    pub fn tag_ids(mut self, value: Vec<String>) -> Self {
        self.tag_ids = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SubscriptionsListQueryRequest`].
    pub fn build(self) -> Result<SubscriptionsListQueryRequest, BuildError> {
        Ok(SubscriptionsListQueryRequest {
            customer_id: self.customer_id,
            limit: self.limit,
            offset: self.offset,
            sort: self.sort,
            status: self.status,
            tag_ids: self.tag_ids,
        })
    }
}
