pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ConnectedAccountCreateRequest {
    /// Identifier of the account in the external system
    pub external_id: String,
    /// Identifier of the integration to link the account through
    pub integration_id: String,
}

impl ConnectedAccountCreateRequest {
    pub fn builder() -> ConnectedAccountCreateRequestBuilder {
        <ConnectedAccountCreateRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ConnectedAccountCreateRequestBuilder {
    external_id: Option<String>,
    integration_id: Option<String>,
}

impl ConnectedAccountCreateRequestBuilder {
    pub fn external_id(mut self, value: impl Into<String>) -> Self {
        self.external_id = Some(value.into());
        self
    }

    pub fn integration_id(mut self, value: impl Into<String>) -> Self {
        self.integration_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ConnectedAccountCreateRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`external_id`](ConnectedAccountCreateRequestBuilder::external_id)
    /// - [`integration_id`](ConnectedAccountCreateRequestBuilder::integration_id)
    pub fn build(self) -> Result<ConnectedAccountCreateRequest, BuildError> {
        Ok(ConnectedAccountCreateRequest {
            external_id: self
                .external_id
                .ok_or_else(|| BuildError::missing_field("external_id"))?,
            integration_id: self
                .integration_id
                .ok_or_else(|| BuildError::missing_field("integration_id"))?,
        })
    }
}
