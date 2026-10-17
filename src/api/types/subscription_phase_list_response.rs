pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct SubscriptionPhaseListResponse {
    /// List of subscription versions
    pub data: Vec<SubscriptionPhaseSummaryResponse>,
}

impl SubscriptionPhaseListResponse {
    pub fn builder() -> SubscriptionPhaseListResponseBuilder {
        <SubscriptionPhaseListResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SubscriptionPhaseListResponseBuilder {
    data: Option<Vec<SubscriptionPhaseSummaryResponse>>,
}

impl SubscriptionPhaseListResponseBuilder {
    pub fn data(mut self, value: Vec<SubscriptionPhaseSummaryResponse>) -> Self {
        self.data = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SubscriptionPhaseListResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`data`](SubscriptionPhaseListResponseBuilder::data)
    pub fn build(self) -> Result<SubscriptionPhaseListResponse, BuildError> {
        Ok(SubscriptionPhaseListResponse {
            data: self.data.ok_or_else(|| BuildError::missing_field("data"))?,
        })
    }
}
