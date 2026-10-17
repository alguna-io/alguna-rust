pub use crate::prelude::*;

/// Query parameters for get
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetQueryRequest {
    /// Target currency for conversion. Uses the entity's default if not specified.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    /// Time interval for schedule entries
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interval: Option<GetRevenueSchedulesRequestInterval>,
}

impl GetQueryRequest {
    pub fn builder() -> GetQueryRequestBuilder {
        <GetQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetQueryRequestBuilder {
    currency: Option<String>,
    interval: Option<GetRevenueSchedulesRequestInterval>,
}

impl GetQueryRequestBuilder {
    pub fn currency(mut self, value: impl Into<String>) -> Self {
        self.currency = Some(value.into());
        self
    }

    pub fn interval(mut self, value: GetRevenueSchedulesRequestInterval) -> Self {
        self.interval = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GetQueryRequest`].
    pub fn build(self) -> Result<GetQueryRequest, BuildError> {
        Ok(GetQueryRequest {
            currency: self.currency,
            interval: self.interval,
        })
    }
}
