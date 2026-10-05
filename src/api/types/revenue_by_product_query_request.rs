pub use crate::prelude::*;

/// Query parameters for revenueByProduct
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct RevenueByProductQueryRequest {
    /// Optional customer ID to filter by
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_id: Option<String>,
    /// Time period for the query.
    pub date_range: RevenueByProductInsightsRequestDateRange,
}

impl RevenueByProductQueryRequest {
    pub fn builder() -> RevenueByProductQueryRequestBuilder {
        <RevenueByProductQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RevenueByProductQueryRequestBuilder {
    customer_id: Option<String>,
    date_range: Option<RevenueByProductInsightsRequestDateRange>,
}

impl RevenueByProductQueryRequestBuilder {
    pub fn customer_id(mut self, value: impl Into<String>) -> Self {
        self.customer_id = Some(value.into());
        self
    }

    pub fn date_range(mut self, value: RevenueByProductInsightsRequestDateRange) -> Self {
        self.date_range = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`RevenueByProductQueryRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`date_range`](RevenueByProductQueryRequestBuilder::date_range)
    pub fn build(self) -> Result<RevenueByProductQueryRequest, BuildError> {
        Ok(RevenueByProductQueryRequest {
            customer_id: self.customer_id,
            date_range: self
                .date_range
                .ok_or_else(|| BuildError::missing_field("date_range"))?,
        })
    }
}
