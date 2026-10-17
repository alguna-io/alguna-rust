pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreditNoteListResponse {
    /// List of credit notes
    pub data: Vec<CreditNoteResponse>,
    /// Pagination metadata
    pub pagination: PaginationMeta,
}

impl CreditNoteListResponse {
    pub fn builder() -> CreditNoteListResponseBuilder {
        <CreditNoteListResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreditNoteListResponseBuilder {
    data: Option<Vec<CreditNoteResponse>>,
    pagination: Option<PaginationMeta>,
}

impl CreditNoteListResponseBuilder {
    pub fn data(mut self, value: Vec<CreditNoteResponse>) -> Self {
        self.data = Some(value);
        self
    }

    pub fn pagination(mut self, value: PaginationMeta) -> Self {
        self.pagination = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CreditNoteListResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`data`](CreditNoteListResponseBuilder::data)
    /// - [`pagination`](CreditNoteListResponseBuilder::pagination)
    pub fn build(self) -> Result<CreditNoteListResponse, BuildError> {
        Ok(CreditNoteListResponse {
            data: self.data.ok_or_else(|| BuildError::missing_field("data"))?,
            pagination: self
                .pagination
                .ok_or_else(|| BuildError::missing_field("pagination"))?,
        })
    }
}
