pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct EntitlementListResponse {
    /// List of entitlements
    pub data: Vec<EntitlementResponse>,
}

impl EntitlementListResponse {
    pub fn builder() -> EntitlementListResponseBuilder {
        <EntitlementListResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct EntitlementListResponseBuilder {
    data: Option<Vec<EntitlementResponse>>,
}

impl EntitlementListResponseBuilder {
    pub fn data(mut self, value: Vec<EntitlementResponse>) -> Self {
        self.data = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`EntitlementListResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`data`](EntitlementListResponseBuilder::data)
    pub fn build(self) -> Result<EntitlementListResponse, BuildError> {
        Ok(EntitlementListResponse {
            data: self.data.ok_or_else(|| BuildError::missing_field("data"))?,
        })
    }
}
