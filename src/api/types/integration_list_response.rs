pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct IntegrationListResponse {
    /// List of integrations
    pub data: Vec<IntegrationResponse>,
}

impl IntegrationListResponse {
    pub fn builder() -> IntegrationListResponseBuilder {
        <IntegrationListResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct IntegrationListResponseBuilder {
    data: Option<Vec<IntegrationResponse>>,
}

impl IntegrationListResponseBuilder {
    pub fn data(mut self, value: Vec<IntegrationResponse>) -> Self {
        self.data = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`IntegrationListResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`data`](IntegrationListResponseBuilder::data)
    pub fn build(self) -> Result<IntegrationListResponse, BuildError> {
        Ok(IntegrationListResponse {
            data: self.data.ok_or_else(|| BuildError::missing_field("data"))?,
        })
    }
}
