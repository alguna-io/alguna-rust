pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PaymentListResponse {
    /// List of payments
    pub data: Vec<PaymentResponse>,
    /// Pagination metadata
    pub pagination: PaginationMeta,
}

impl PaymentListResponse {
    pub fn builder() -> PaymentListResponseBuilder {
        <PaymentListResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PaymentListResponseBuilder {
    data: Option<Vec<PaymentResponse>>,
    pagination: Option<PaginationMeta>,
}

impl PaymentListResponseBuilder {
    pub fn data(mut self, value: Vec<PaymentResponse>) -> Self {
        self.data = Some(value);
        self
    }

    pub fn pagination(mut self, value: PaginationMeta) -> Self {
        self.pagination = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PaymentListResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`data`](PaymentListResponseBuilder::data)
    /// - [`pagination`](PaymentListResponseBuilder::pagination)
    pub fn build(self) -> Result<PaymentListResponse, BuildError> {
        Ok(PaymentListResponse {
            data: self.data.ok_or_else(|| BuildError::missing_field("data"))?,
            pagination: self
                .pagination
                .ok_or_else(|| BuildError::missing_field("pagination"))?,
        })
    }
}
