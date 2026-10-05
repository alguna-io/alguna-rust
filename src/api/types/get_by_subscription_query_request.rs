pub use crate::prelude::*;

/// Query parameters for getBySubscription
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetBySubscriptionQueryRequest {
    /// Target currency for conversion. Uses the entity's default if not specified.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    /// Time interval for schedule entries
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interval: Option<GetBySubscriptionRevenueSchedulesRequestInterval>,
}

impl GetBySubscriptionQueryRequest {
    pub fn builder() -> GetBySubscriptionQueryRequestBuilder {
        <GetBySubscriptionQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetBySubscriptionQueryRequestBuilder {
    currency: Option<String>,
    interval: Option<GetBySubscriptionRevenueSchedulesRequestInterval>,
}

impl GetBySubscriptionQueryRequestBuilder {
    pub fn currency(mut self, value: impl Into<String>) -> Self {
        self.currency = Some(value.into());
        self
    }

    pub fn interval(mut self, value: GetBySubscriptionRevenueSchedulesRequestInterval) -> Self {
        self.interval = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GetBySubscriptionQueryRequest`].
    pub fn build(self) -> Result<GetBySubscriptionQueryRequest, BuildError> {
        Ok(GetBySubscriptionQueryRequest {
            currency: self.currency,
            interval: self.interval,
        })
    }
}
