pub use crate::prelude::*;

/// Query parameters for list
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct BundlesListQueryRequest {
    /// Number of items to return per page
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// Number of items to skip
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<i64>,
}

impl BundlesListQueryRequest {
    pub fn builder() -> BundlesListQueryRequestBuilder {
        <BundlesListQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BundlesListQueryRequestBuilder {
    limit: Option<i64>,
    offset: Option<i64>,
}

impl BundlesListQueryRequestBuilder {
    pub fn limit(mut self, value: i64) -> Self {
        self.limit = Some(value);
        self
    }

    pub fn offset(mut self, value: i64) -> Self {
        self.offset = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`BundlesListQueryRequest`].
    pub fn build(self) -> Result<BundlesListQueryRequest, BuildError> {
        Ok(BundlesListQueryRequest {
            limit: self.limit,
            offset: self.offset,
        })
    }
}
