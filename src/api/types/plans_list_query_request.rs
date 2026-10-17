pub use crate::prelude::*;

/// Query parameters for list
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PlansListQueryRequest {
    /// Number of items to return per page
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// Number of items to skip
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<i64>,
    /// Sort field and direction in format field:order
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort: Option<String>,
    /// Filter by tag IDs (comma-separated to match any)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_ids: Option<Vec<String>>,
}

impl PlansListQueryRequest {
    pub fn builder() -> PlansListQueryRequestBuilder {
        <PlansListQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PlansListQueryRequestBuilder {
    limit: Option<i64>,
    offset: Option<i64>,
    sort: Option<String>,
    tag_ids: Option<Vec<String>>,
}

impl PlansListQueryRequestBuilder {
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

    pub fn tag_ids(mut self, value: Vec<String>) -> Self {
        self.tag_ids = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PlansListQueryRequest`].
    pub fn build(self) -> Result<PlansListQueryRequest, BuildError> {
        Ok(PlansListQueryRequest {
            limit: self.limit,
            offset: self.offset,
            sort: self.sort,
            tag_ids: self.tag_ids,
        })
    }
}
