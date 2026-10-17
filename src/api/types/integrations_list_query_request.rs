pub use crate::prelude::*;

/// Query parameters for list
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct IntegrationsListQueryRequest {
    /// Filter by integration provider
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider: Option<String>,
}

impl IntegrationsListQueryRequest {
    pub fn builder() -> IntegrationsListQueryRequestBuilder {
        <IntegrationsListQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct IntegrationsListQueryRequestBuilder {
    provider: Option<String>,
}

impl IntegrationsListQueryRequestBuilder {
    pub fn provider(mut self, value: impl Into<String>) -> Self {
        self.provider = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`IntegrationsListQueryRequest`].
    pub fn build(self) -> Result<IntegrationsListQueryRequest, BuildError> {
        Ok(IntegrationsListQueryRequest {
            provider: self.provider,
        })
    }
}
