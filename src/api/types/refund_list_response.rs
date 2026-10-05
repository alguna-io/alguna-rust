pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct RefundListResponse {
    /// List of refunds
    pub data: Vec<RefundResponse>,
    /// Pagination metadata
    pub pagination: PaginationMeta,
}

impl RefundListResponse {
    pub fn builder() -> RefundListResponseBuilder {
        <RefundListResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RefundListResponseBuilder {
    data: Option<Vec<RefundResponse>>,
    pagination: Option<PaginationMeta>,
}

impl RefundListResponseBuilder {
    pub fn data(mut self, value: Vec<RefundResponse>) -> Self {
        self.data = Some(value);
        self
    }

    pub fn pagination(mut self, value: PaginationMeta) -> Self {
        self.pagination = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`RefundListResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`data`](RefundListResponseBuilder::data)
    /// - [`pagination`](RefundListResponseBuilder::pagination)
    pub fn build(self) -> Result<RefundListResponse, BuildError> {
        Ok(RefundListResponse {
            data: self.data.ok_or_else(|| BuildError::missing_field("data"))?,
            pagination: self
                .pagination
                .ok_or_else(|| BuildError::missing_field("pagination"))?,
        })
    }
}
