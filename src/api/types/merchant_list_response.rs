pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct MerchantListResponse {
    /// List of merchants
    pub data: Vec<MerchantResponse>,
    /// Pagination metadata
    pub pagination: PaginationMeta,
}

impl MerchantListResponse {
    pub fn builder() -> MerchantListResponseBuilder {
        <MerchantListResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct MerchantListResponseBuilder {
    data: Option<Vec<MerchantResponse>>,
    pagination: Option<PaginationMeta>,
}

impl MerchantListResponseBuilder {
    pub fn data(mut self, value: Vec<MerchantResponse>) -> Self {
        self.data = Some(value);
        self
    }

    pub fn pagination(mut self, value: PaginationMeta) -> Self {
        self.pagination = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`MerchantListResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`data`](MerchantListResponseBuilder::data)
    /// - [`pagination`](MerchantListResponseBuilder::pagination)
    pub fn build(self) -> Result<MerchantListResponse, BuildError> {
        Ok(MerchantListResponse {
            data: self.data.ok_or_else(|| BuildError::missing_field("data"))?,
            pagination: self
                .pagination
                .ok_or_else(|| BuildError::missing_field("pagination"))?,
        })
    }
}
