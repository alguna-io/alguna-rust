pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct QuoteListResponse {
    /// List of quotes
    pub data: Vec<QuoteResponse>,
    /// Pagination metadata
    pub pagination: PaginationMeta,
}

impl QuoteListResponse {
    pub fn builder() -> QuoteListResponseBuilder {
        <QuoteListResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct QuoteListResponseBuilder {
    data: Option<Vec<QuoteResponse>>,
    pagination: Option<PaginationMeta>,
}

impl QuoteListResponseBuilder {
    pub fn data(mut self, value: Vec<QuoteResponse>) -> Self {
        self.data = Some(value);
        self
    }

    pub fn pagination(mut self, value: PaginationMeta) -> Self {
        self.pagination = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`QuoteListResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`data`](QuoteListResponseBuilder::data)
    /// - [`pagination`](QuoteListResponseBuilder::pagination)
    pub fn build(self) -> Result<QuoteListResponse, BuildError> {
        Ok(QuoteListResponse {
            data: self.data.ok_or_else(|| BuildError::missing_field("data"))?,
            pagination: self
                .pagination
                .ok_or_else(|| BuildError::missing_field("pagination"))?,
        })
    }
}
