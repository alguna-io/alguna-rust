pub use crate::prelude::*;

/// Query parameters for list
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct WalletGrantsListQueryRequest {
    /// Number of items to return per page
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// Number of items to skip
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<i64>,
    /// Filter by grant status
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<ListWalletGrantsRequestStatus>,
    /// Filter by wallet ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wallet_id: Option<String>,
}

impl WalletGrantsListQueryRequest {
    pub fn builder() -> WalletGrantsListQueryRequestBuilder {
        <WalletGrantsListQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct WalletGrantsListQueryRequestBuilder {
    limit: Option<i64>,
    offset: Option<i64>,
    status: Option<ListWalletGrantsRequestStatus>,
    wallet_id: Option<String>,
}

impl WalletGrantsListQueryRequestBuilder {
    pub fn limit(mut self, value: i64) -> Self {
        self.limit = Some(value);
        self
    }

    pub fn offset(mut self, value: i64) -> Self {
        self.offset = Some(value);
        self
    }

    pub fn status(mut self, value: ListWalletGrantsRequestStatus) -> Self {
        self.status = Some(value);
        self
    }

    pub fn wallet_id(mut self, value: impl Into<String>) -> Self {
        self.wallet_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`WalletGrantsListQueryRequest`].
    pub fn build(self) -> Result<WalletGrantsListQueryRequest, BuildError> {
        Ok(WalletGrantsListQueryRequest {
            limit: self.limit,
            offset: self.offset,
            status: self.status,
            wallet_id: self.wallet_id,
        })
    }
}
