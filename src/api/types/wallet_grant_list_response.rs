pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct WalletGrantListResponse {
    /// List of wallet grants
    pub data: Vec<WalletGrantResponse>,
    /// Pagination metadata
    pub pagination: PaginationMeta,
}

impl WalletGrantListResponse {
    pub fn builder() -> WalletGrantListResponseBuilder {
        <WalletGrantListResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct WalletGrantListResponseBuilder {
    data: Option<Vec<WalletGrantResponse>>,
    pagination: Option<PaginationMeta>,
}

impl WalletGrantListResponseBuilder {
    pub fn data(mut self, value: Vec<WalletGrantResponse>) -> Self {
        self.data = Some(value);
        self
    }

    pub fn pagination(mut self, value: PaginationMeta) -> Self {
        self.pagination = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`WalletGrantListResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`data`](WalletGrantListResponseBuilder::data)
    /// - [`pagination`](WalletGrantListResponseBuilder::pagination)
    pub fn build(self) -> Result<WalletGrantListResponse, BuildError> {
        Ok(WalletGrantListResponse {
            data: self.data.ok_or_else(|| BuildError::missing_field("data"))?,
            pagination: self
                .pagination
                .ok_or_else(|| BuildError::missing_field("pagination"))?,
        })
    }
}
