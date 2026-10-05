pub use crate::prelude::*;

/// Query parameters for list
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct WalletsListQueryRequest {
    /// Filter by active status
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,
    /// Filter by customer ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_id: Option<String>,
    /// Number of items to return per page
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// Number of items to skip
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<i64>,
}

impl WalletsListQueryRequest {
    pub fn builder() -> WalletsListQueryRequestBuilder {
        <WalletsListQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct WalletsListQueryRequestBuilder {
    active: Option<bool>,
    customer_id: Option<String>,
    limit: Option<i64>,
    offset: Option<i64>,
}

impl WalletsListQueryRequestBuilder {
    pub fn active(mut self, value: bool) -> Self {
        self.active = Some(value);
        self
    }

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

    /// Consumes the builder and constructs a [`WalletsListQueryRequest`].
    pub fn build(self) -> Result<WalletsListQueryRequest, BuildError> {
        Ok(WalletsListQueryRequest {
            active: self.active,
            customer_id: self.customer_id,
            limit: self.limit,
            offset: self.offset,
        })
    }
}
