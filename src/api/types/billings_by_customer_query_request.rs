pub use crate::prelude::*;

/// Query parameters for billingsByCustomer
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct BillingsByCustomerQueryRequest {
    /// Time period for the query. Determines the primary period and automatically calculates a comparison period.
    pub date_range: BillingsByCustomerInsightsRequestDateRange,
}

impl BillingsByCustomerQueryRequest {
    pub fn builder() -> BillingsByCustomerQueryRequestBuilder {
        <BillingsByCustomerQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BillingsByCustomerQueryRequestBuilder {
    date_range: Option<BillingsByCustomerInsightsRequestDateRange>,
}

impl BillingsByCustomerQueryRequestBuilder {
    pub fn date_range(mut self, value: BillingsByCustomerInsightsRequestDateRange) -> Self {
        self.date_range = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`BillingsByCustomerQueryRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`date_range`](BillingsByCustomerQueryRequestBuilder::date_range)
    pub fn build(self) -> Result<BillingsByCustomerQueryRequest, BuildError> {
        Ok(BillingsByCustomerQueryRequest {
            date_range: self
                .date_range
                .ok_or_else(|| BuildError::missing_field("date_range"))?,
        })
    }
}
