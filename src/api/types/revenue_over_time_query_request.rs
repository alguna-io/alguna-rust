pub use crate::prelude::*;

/// Query parameters for revenueOverTime
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct RevenueOverTimeQueryRequest {
    /// Time period for the query. Determines the primary period and automatically calculates a comparison period.
    pub date_range: RevenueOverTimeInsightsRequestDateRange,
}

impl RevenueOverTimeQueryRequest {
    pub fn builder() -> RevenueOverTimeQueryRequestBuilder {
        <RevenueOverTimeQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RevenueOverTimeQueryRequestBuilder {
    date_range: Option<RevenueOverTimeInsightsRequestDateRange>,
}

impl RevenueOverTimeQueryRequestBuilder {
    pub fn date_range(mut self, value: RevenueOverTimeInsightsRequestDateRange) -> Self {
        self.date_range = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`RevenueOverTimeQueryRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`date_range`](RevenueOverTimeQueryRequestBuilder::date_range)
    pub fn build(self) -> Result<RevenueOverTimeQueryRequest, BuildError> {
        Ok(RevenueOverTimeQueryRequest {
            date_range: self
                .date_range
                .ok_or_else(|| BuildError::missing_field("date_range"))?,
        })
    }
}
