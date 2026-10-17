pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CustomerListResponse {
    /// List of customers
    pub data: Vec<CustomerResponse>,
    /// Pagination metadata
    pub pagination: PaginationMeta,
}

impl CustomerListResponse {
    pub fn builder() -> CustomerListResponseBuilder {
        <CustomerListResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CustomerListResponseBuilder {
    data: Option<Vec<CustomerResponse>>,
    pagination: Option<PaginationMeta>,
}

impl CustomerListResponseBuilder {
    pub fn data(mut self, value: Vec<CustomerResponse>) -> Self {
        self.data = Some(value);
        self
    }

    pub fn pagination(mut self, value: PaginationMeta) -> Self {
        self.pagination = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CustomerListResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`data`](CustomerListResponseBuilder::data)
    /// - [`pagination`](CustomerListResponseBuilder::pagination)
    pub fn build(self) -> Result<CustomerListResponse, BuildError> {
        Ok(CustomerListResponse {
            data: self.data.ok_or_else(|| BuildError::missing_field("data"))?,
            pagination: self
                .pagination
                .ok_or_else(|| BuildError::missing_field("pagination"))?,
        })
    }
}
