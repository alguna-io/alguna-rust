pub use crate::prelude::*;

/// Query parameters for billingsOverTime
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct BillingsOverTimeQueryRequest {
    /// Time period for the query. Determines the primary period and automatically calculates a comparison period.
    pub date_range: BillingsOverTimeInsightsRequestDateRange,
}

impl BillingsOverTimeQueryRequest {
    pub fn builder() -> BillingsOverTimeQueryRequestBuilder {
        <BillingsOverTimeQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BillingsOverTimeQueryRequestBuilder {
    date_range: Option<BillingsOverTimeInsightsRequestDateRange>,
}

impl BillingsOverTimeQueryRequestBuilder {
    pub fn date_range(mut self, value: BillingsOverTimeInsightsRequestDateRange) -> Self {
        self.date_range = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`BillingsOverTimeQueryRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`date_range`](BillingsOverTimeQueryRequestBuilder::date_range)
    pub fn build(self) -> Result<BillingsOverTimeQueryRequest, BuildError> {
        Ok(BillingsOverTimeQueryRequest {
            date_range: self
                .date_range
                .ok_or_else(|| BuildError::missing_field("date_range"))?,
        })
    }
}
