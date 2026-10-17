pub use crate::prelude::*;

/// Query parameters for revenueBreakdownOverTime
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct RevenueBreakdownOverTimeQueryRequest {
    /// Time period for the query. Determines the primary period and automatically calculates a comparison period.
    pub date_range: RevenueBreakdownOverTimeInsightsRequestDateRange,
}

impl RevenueBreakdownOverTimeQueryRequest {
    pub fn builder() -> RevenueBreakdownOverTimeQueryRequestBuilder {
        <RevenueBreakdownOverTimeQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RevenueBreakdownOverTimeQueryRequestBuilder {
    date_range: Option<RevenueBreakdownOverTimeInsightsRequestDateRange>,
}

impl RevenueBreakdownOverTimeQueryRequestBuilder {
    pub fn date_range(mut self, value: RevenueBreakdownOverTimeInsightsRequestDateRange) -> Self {
        self.date_range = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`RevenueBreakdownOverTimeQueryRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`date_range`](RevenueBreakdownOverTimeQueryRequestBuilder::date_range)
    pub fn build(self) -> Result<RevenueBreakdownOverTimeQueryRequest, BuildError> {
        Ok(RevenueBreakdownOverTimeQueryRequest {
            date_range: self
                .date_range
                .ok_or_else(|| BuildError::missing_field("date_range"))?,
        })
    }
}
