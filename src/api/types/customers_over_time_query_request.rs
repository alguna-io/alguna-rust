pub use crate::prelude::*;

/// Query parameters for customersOverTime
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct CustomersOverTimeQueryRequest {
    /// Time period for the query. Determines the primary period and automatically calculates a comparison period.
    pub date_range: CustomersOverTimeInsightsRequestDateRange,
}

impl CustomersOverTimeQueryRequest {
    pub fn builder() -> CustomersOverTimeQueryRequestBuilder {
        <CustomersOverTimeQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CustomersOverTimeQueryRequestBuilder {
    date_range: Option<CustomersOverTimeInsightsRequestDateRange>,
}

impl CustomersOverTimeQueryRequestBuilder {
    pub fn date_range(mut self, value: CustomersOverTimeInsightsRequestDateRange) -> Self {
        self.date_range = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CustomersOverTimeQueryRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`date_range`](CustomersOverTimeQueryRequestBuilder::date_range)
    pub fn build(self) -> Result<CustomersOverTimeQueryRequest, BuildError> {
        Ok(CustomersOverTimeQueryRequest {
            date_range: self
                .date_range
                .ok_or_else(|| BuildError::missing_field("date_range"))?,
        })
    }
}
