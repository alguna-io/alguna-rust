pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ConnectedAccountResponse {
    /// Alguna customer ID the connected account belongs to
    pub customer_id: String,
    /// Identifier of the account in the external system
    pub external_id: String,
    /// Unique identifier for the connected account link
    pub id: String,
    /// Identifier of the integration the account is connected through
    pub integration_id: String,
}

impl ConnectedAccountResponse {
    pub fn builder() -> ConnectedAccountResponseBuilder {
        <ConnectedAccountResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ConnectedAccountResponseBuilder {
    customer_id: Option<String>,
    external_id: Option<String>,
    id: Option<String>,
    integration_id: Option<String>,
}

impl ConnectedAccountResponseBuilder {
    pub fn customer_id(mut self, value: impl Into<String>) -> Self {
        self.customer_id = Some(value.into());
        self
    }

    pub fn external_id(mut self, value: impl Into<String>) -> Self {
        self.external_id = Some(value.into());
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn integration_id(mut self, value: impl Into<String>) -> Self {
        self.integration_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ConnectedAccountResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`customer_id`](ConnectedAccountResponseBuilder::customer_id)
    /// - [`external_id`](ConnectedAccountResponseBuilder::external_id)
    /// - [`id`](ConnectedAccountResponseBuilder::id)
    /// - [`integration_id`](ConnectedAccountResponseBuilder::integration_id)
    pub fn build(self) -> Result<ConnectedAccountResponse, BuildError> {
        Ok(ConnectedAccountResponse {
            customer_id: self
                .customer_id
                .ok_or_else(|| BuildError::missing_field("customer_id"))?,
            external_id: self
                .external_id
                .ok_or_else(|| BuildError::missing_field("external_id"))?,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            integration_id: self
                .integration_id
                .ok_or_else(|| BuildError::missing_field("integration_id"))?,
        })
    }
}
