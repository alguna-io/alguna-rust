pub use crate::prelude::*;

/// Query parameters for getByLegalEntity
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetByLegalEntityQueryRequest {
    /// Target currency for conversion. Uses the entity's default if not specified.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    /// Time interval for schedule entries
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interval: Option<GetByLegalEntityRevenueSchedulesRequestInterval>,
}

impl GetByLegalEntityQueryRequest {
    pub fn builder() -> GetByLegalEntityQueryRequestBuilder {
        <GetByLegalEntityQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetByLegalEntityQueryRequestBuilder {
    currency: Option<String>,
    interval: Option<GetByLegalEntityRevenueSchedulesRequestInterval>,
}

impl GetByLegalEntityQueryRequestBuilder {
    pub fn currency(mut self, value: impl Into<String>) -> Self {
        self.currency = Some(value.into());
        self
    }

    pub fn interval(mut self, value: GetByLegalEntityRevenueSchedulesRequestInterval) -> Self {
        self.interval = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GetByLegalEntityQueryRequest`].
    pub fn build(self) -> Result<GetByLegalEntityQueryRequest, BuildError> {
        Ok(GetByLegalEntityQueryRequest {
            currency: self.currency,
            interval: self.interval,
        })
    }
}
