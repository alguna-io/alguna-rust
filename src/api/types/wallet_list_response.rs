pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct WalletListResponse {
    /// List of wallets
    pub data: Vec<WalletResponse>,
    /// Pagination metadata
    pub pagination: PaginationMeta,
}

impl WalletListResponse {
    pub fn builder() -> WalletListResponseBuilder {
        <WalletListResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct WalletListResponseBuilder {
    data: Option<Vec<WalletResponse>>,
    pagination: Option<PaginationMeta>,
}

impl WalletListResponseBuilder {
    pub fn data(mut self, value: Vec<WalletResponse>) -> Self {
        self.data = Some(value);
        self
    }

    pub fn pagination(mut self, value: PaginationMeta) -> Self {
        self.pagination = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`WalletListResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`data`](WalletListResponseBuilder::data)
    /// - [`pagination`](WalletListResponseBuilder::pagination)
    pub fn build(self) -> Result<WalletListResponse, BuildError> {
        Ok(WalletListResponse {
            data: self.data.ok_or_else(|| BuildError::missing_field("data"))?,
            pagination: self
                .pagination
                .ok_or_else(|| BuildError::missing_field("pagination"))?,
        })
    }
}
