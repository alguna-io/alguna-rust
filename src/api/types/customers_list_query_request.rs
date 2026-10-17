pub use crate::prelude::*;

/// Query parameters for list
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CustomersListQueryRequest {
    /// Filter by customer alias (external ID)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alias: Option<String>,
    /// Filter by customer success manager user ID (comma-separated to match any)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub csm_id: Option<Vec<String>>,
    /// Filter by contact email address
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    /// Filter by whether the customer has active subscriptions
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_active_subscriptions: Option<bool>,
    /// Filter by external integration IDs (comma-separated to match any)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub integration_ids: Option<Vec<String>>,
    /// Number of items to return per page
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// Number of items to skip
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<i64>,
    /// Filter by sales owner user ID (comma-separated to match any)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sales_owner_id: Option<Vec<String>>,
    /// Sort field and direction in format field:order. Sortable fields: name, created_at
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort: Option<String>,
    /// Filter by tag IDs (comma-separated to match any)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_ids: Option<Vec<String>>,
}

impl CustomersListQueryRequest {
    pub fn builder() -> CustomersListQueryRequestBuilder {
        <CustomersListQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CustomersListQueryRequestBuilder {
    alias: Option<String>,
    csm_id: Option<Vec<String>>,
    email: Option<String>,
    has_active_subscriptions: Option<bool>,
    integration_ids: Option<Vec<String>>,
    limit: Option<i64>,
    offset: Option<i64>,
    sales_owner_id: Option<Vec<String>>,
    sort: Option<String>,
    tag_ids: Option<Vec<String>>,
}

impl CustomersListQueryRequestBuilder {
    pub fn alias(mut self, value: impl Into<String>) -> Self {
        self.alias = Some(value.into());
        self
    }

    pub fn csm_id(mut self, value: Vec<String>) -> Self {
        self.csm_id = Some(value);
        self
    }

    pub fn email(mut self, value: impl Into<String>) -> Self {
        self.email = Some(value.into());
        self
    }

    pub fn has_active_subscriptions(mut self, value: bool) -> Self {
        self.has_active_subscriptions = Some(value);
        self
    }

    pub fn integration_ids(mut self, value: Vec<String>) -> Self {
        self.integration_ids = Some(value);
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

    pub fn sales_owner_id(mut self, value: Vec<String>) -> Self {
        self.sales_owner_id = Some(value);
        self
    }

    pub fn sort(mut self, value: impl Into<String>) -> Self {
        self.sort = Some(value.into());
        self
    }

    pub fn tag_ids(mut self, value: Vec<String>) -> Self {
        self.tag_ids = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CustomersListQueryRequest`].
    pub fn build(self) -> Result<CustomersListQueryRequest, BuildError> {
        Ok(CustomersListQueryRequest {
            alias: self.alias,
            csm_id: self.csm_id,
            email: self.email,
            has_active_subscriptions: self.has_active_subscriptions,
            integration_ids: self.integration_ids,
            limit: self.limit,
            offset: self.offset,
            sales_owner_id: self.sales_owner_id,
            sort: self.sort,
            tag_ids: self.tag_ids,
        })
    }
}
