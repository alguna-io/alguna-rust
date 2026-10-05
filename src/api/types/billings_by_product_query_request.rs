pub use crate::prelude::*;

/// Query parameters for billingsByProduct
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct BillingsByProductQueryRequest {
    /// Optional customer ID to filter by
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_id: Option<String>,
    /// Time period for the query.
    pub date_range: BillingsByProductInsightsRequestDateRange,
}

impl BillingsByProductQueryRequest {
    pub fn builder() -> BillingsByProductQueryRequestBuilder {
        <BillingsByProductQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BillingsByProductQueryRequestBuilder {
    customer_id: Option<String>,
    date_range: Option<BillingsByProductInsightsRequestDateRange>,
}

impl BillingsByProductQueryRequestBuilder {
    pub fn customer_id(mut self, value: impl Into<String>) -> Self {
        self.customer_id = Some(value.into());
        self
    }

    pub fn date_range(mut self, value: BillingsByProductInsightsRequestDateRange) -> Self {
        self.date_range = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`BillingsByProductQueryRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`date_range`](BillingsByProductQueryRequestBuilder::date_range)
    pub fn build(self) -> Result<BillingsByProductQueryRequest, BuildError> {
        Ok(BillingsByProductQueryRequest {
            customer_id: self.customer_id,
            date_range: self
                .date_range
                .ok_or_else(|| BuildError::missing_field("date_range"))?,
        })
    }
}
