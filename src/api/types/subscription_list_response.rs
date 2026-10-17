pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct SubscriptionListResponse {
    /// List of subscriptions
    pub data: Vec<SubscriptionResponse>,
    /// Pagination metadata
    pub pagination: PaginationMeta,
}

impl SubscriptionListResponse {
    pub fn builder() -> SubscriptionListResponseBuilder {
        <SubscriptionListResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SubscriptionListResponseBuilder {
    data: Option<Vec<SubscriptionResponse>>,
    pagination: Option<PaginationMeta>,
}

impl SubscriptionListResponseBuilder {
    pub fn data(mut self, value: Vec<SubscriptionResponse>) -> Self {
        self.data = Some(value);
        self
    }

    pub fn pagination(mut self, value: PaginationMeta) -> Self {
        self.pagination = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SubscriptionListResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`data`](SubscriptionListResponseBuilder::data)
    /// - [`pagination`](SubscriptionListResponseBuilder::pagination)
    pub fn build(self) -> Result<SubscriptionListResponse, BuildError> {
        Ok(SubscriptionListResponse {
            data: self.data.ok_or_else(|| BuildError::missing_field("data"))?,
            pagination: self
                .pagination
                .ok_or_else(|| BuildError::missing_field("pagination"))?,
        })
    }
}
