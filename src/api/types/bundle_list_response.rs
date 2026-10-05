pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct BundleListResponse {
    /// List of bundles
    pub data: Vec<BundleResponse>,
    /// Pagination metadata
    pub pagination: PaginationMeta,
}

impl BundleListResponse {
    pub fn builder() -> BundleListResponseBuilder {
        <BundleListResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BundleListResponseBuilder {
    data: Option<Vec<BundleResponse>>,
    pagination: Option<PaginationMeta>,
}

impl BundleListResponseBuilder {
    pub fn data(mut self, value: Vec<BundleResponse>) -> Self {
        self.data = Some(value);
        self
    }

    pub fn pagination(mut self, value: PaginationMeta) -> Self {
        self.pagination = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`BundleListResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`data`](BundleListResponseBuilder::data)
    /// - [`pagination`](BundleListResponseBuilder::pagination)
    pub fn build(self) -> Result<BundleListResponse, BuildError> {
        Ok(BundleListResponse {
            data: self.data.ok_or_else(|| BuildError::missing_field("data"))?,
            pagination: self
                .pagination
                .ok_or_else(|| BuildError::missing_field("pagination"))?,
        })
    }
}
