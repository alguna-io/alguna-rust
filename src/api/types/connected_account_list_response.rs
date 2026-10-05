pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ConnectedAccountListResponse {
    /// List of connected accounts
    pub data: Vec<ConnectedAccountResponse>,
}

impl ConnectedAccountListResponse {
    pub fn builder() -> ConnectedAccountListResponseBuilder {
        <ConnectedAccountListResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ConnectedAccountListResponseBuilder {
    data: Option<Vec<ConnectedAccountResponse>>,
}

impl ConnectedAccountListResponseBuilder {
    pub fn data(mut self, value: Vec<ConnectedAccountResponse>) -> Self {
        self.data = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ConnectedAccountListResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`data`](ConnectedAccountListResponseBuilder::data)
    pub fn build(self) -> Result<ConnectedAccountListResponse, BuildError> {
        Ok(ConnectedAccountListResponse {
            data: self.data.ok_or_else(|| BuildError::missing_field("data"))?,
        })
    }
}
