pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ProductListResponse {
    /// List of products
    pub data: Vec<ProductResponse>,
    /// Pagination metadata
    pub pagination: PaginationMeta,
}

impl ProductListResponse {
    pub fn builder() -> ProductListResponseBuilder {
        <ProductListResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ProductListResponseBuilder {
    data: Option<Vec<ProductResponse>>,
    pagination: Option<PaginationMeta>,
}

impl ProductListResponseBuilder {
    pub fn data(mut self, value: Vec<ProductResponse>) -> Self {
        self.data = Some(value);
        self
    }

    pub fn pagination(mut self, value: PaginationMeta) -> Self {
        self.pagination = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ProductListResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`data`](ProductListResponseBuilder::data)
    /// - [`pagination`](ProductListResponseBuilder::pagination)
    pub fn build(self) -> Result<ProductListResponse, BuildError> {
        Ok(ProductListResponse {
            data: self.data.ok_or_else(|| BuildError::missing_field("data"))?,
            pagination: self
                .pagination
                .ok_or_else(|| BuildError::missing_field("pagination"))?,
        })
    }
}
