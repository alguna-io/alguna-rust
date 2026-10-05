pub use crate::prelude::*;

/// Query parameters for getByCustomer
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetByCustomerQueryRequest {
    /// Target currency for conversion. Uses the entity's default if not specified.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    /// Time interval for schedule entries
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interval: Option<GetByCustomerRevenueSchedulesRequestInterval>,
}

impl GetByCustomerQueryRequest {
    pub fn builder() -> GetByCustomerQueryRequestBuilder {
        <GetByCustomerQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetByCustomerQueryRequestBuilder {
    currency: Option<String>,
    interval: Option<GetByCustomerRevenueSchedulesRequestInterval>,
}

impl GetByCustomerQueryRequestBuilder {
    pub fn currency(mut self, value: impl Into<String>) -> Self {
        self.currency = Some(value.into());
        self
    }

    pub fn interval(mut self, value: GetByCustomerRevenueSchedulesRequestInterval) -> Self {
        self.interval = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GetByCustomerQueryRequest`].
    pub fn build(self) -> Result<GetByCustomerQueryRequest, BuildError> {
        Ok(GetByCustomerQueryRequest {
            currency: self.currency,
            interval: self.interval,
        })
    }
}
