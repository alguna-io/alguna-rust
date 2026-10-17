pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct TagListResponse {
    /// List of tags
    pub data: Vec<TagResponse>,
    /// Pagination metadata
    pub pagination: PaginationMeta,
}

impl TagListResponse {
    pub fn builder() -> TagListResponseBuilder {
        <TagListResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TagListResponseBuilder {
    data: Option<Vec<TagResponse>>,
    pagination: Option<PaginationMeta>,
}

impl TagListResponseBuilder {
    pub fn data(mut self, value: Vec<TagResponse>) -> Self {
        self.data = Some(value);
        self
    }

    pub fn pagination(mut self, value: PaginationMeta) -> Self {
        self.pagination = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TagListResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`data`](TagListResponseBuilder::data)
    /// - [`pagination`](TagListResponseBuilder::pagination)
    pub fn build(self) -> Result<TagListResponse, BuildError> {
        Ok(TagListResponse {
            data: self.data.ok_or_else(|| BuildError::missing_field("data"))?,
            pagination: self
                .pagination
                .ok_or_else(|| BuildError::missing_field("pagination"))?,
        })
    }
}
