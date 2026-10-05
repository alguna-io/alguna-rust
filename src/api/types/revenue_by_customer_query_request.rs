pub use crate::prelude::*;

/// Query parameters for revenueByCustomer
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct RevenueByCustomerQueryRequest {
    /// Time period for the query. Determines the primary period and automatically calculates a comparison period.
    pub date_range: RevenueByCustomerInsightsRequestDateRange,
}

impl RevenueByCustomerQueryRequest {
    pub fn builder() -> RevenueByCustomerQueryRequestBuilder {
        <RevenueByCustomerQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RevenueByCustomerQueryRequestBuilder {
    date_range: Option<RevenueByCustomerInsightsRequestDateRange>,
}

impl RevenueByCustomerQueryRequestBuilder {
    pub fn date_range(mut self, value: RevenueByCustomerInsightsRequestDateRange) -> Self {
        self.date_range = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`RevenueByCustomerQueryRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`date_range`](RevenueByCustomerQueryRequestBuilder::date_range)
    pub fn build(self) -> Result<RevenueByCustomerQueryRequest, BuildError> {
        Ok(RevenueByCustomerQueryRequest {
            date_range: self
                .date_range
                .ok_or_else(|| BuildError::missing_field("date_range"))?,
        })
    }
}
