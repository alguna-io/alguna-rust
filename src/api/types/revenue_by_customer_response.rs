pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct RevenueByCustomerResponse {
    /// ISO 4217 currency code
    pub currency: String,
    /// Aggregated revenue from remaining customers for the comparison period
    pub other_customers_net_revenue_previous_period: String,
    /// Aggregated revenue from remaining customers for the selected period
    pub other_customers_net_revenue_this_period: String,
    /// Top customers by revenue
    pub top_customers: Vec<InsightsCustomerEntry>,
}

impl RevenueByCustomerResponse {
    pub fn builder() -> RevenueByCustomerResponseBuilder {
        <RevenueByCustomerResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RevenueByCustomerResponseBuilder {
    currency: Option<String>,
    other_customers_net_revenue_previous_period: Option<String>,
    other_customers_net_revenue_this_period: Option<String>,
    top_customers: Option<Vec<InsightsCustomerEntry>>,
}

impl RevenueByCustomerResponseBuilder {
    pub fn currency(mut self, value: impl Into<String>) -> Self {
        self.currency = Some(value.into());
        self
    }

    pub fn other_customers_net_revenue_previous_period(mut self, value: impl Into<String>) -> Self {
        self.other_customers_net_revenue_previous_period = Some(value.into());
        self
    }

    pub fn other_customers_net_revenue_this_period(mut self, value: impl Into<String>) -> Self {
        self.other_customers_net_revenue_this_period = Some(value.into());
        self
    }

    pub fn top_customers(mut self, value: Vec<InsightsCustomerEntry>) -> Self {
        self.top_customers = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`RevenueByCustomerResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`currency`](RevenueByCustomerResponseBuilder::currency)
    /// - [`other_customers_net_revenue_previous_period`](RevenueByCustomerResponseBuilder::other_customers_net_revenue_previous_period)
    /// - [`other_customers_net_revenue_this_period`](RevenueByCustomerResponseBuilder::other_customers_net_revenue_this_period)
    /// - [`top_customers`](RevenueByCustomerResponseBuilder::top_customers)
    pub fn build(self) -> Result<RevenueByCustomerResponse, BuildError> {
        Ok(RevenueByCustomerResponse {
            currency: self
                .currency
                .ok_or_else(|| BuildError::missing_field("currency"))?,
            other_customers_net_revenue_previous_period: self
                .other_customers_net_revenue_previous_period
                .ok_or_else(|| {
                    BuildError::missing_field("other_customers_net_revenue_previous_period")
                })?,
            other_customers_net_revenue_this_period: self
                .other_customers_net_revenue_this_period
                .ok_or_else(|| {
                BuildError::missing_field("other_customers_net_revenue_this_period")
            })?,
            top_customers: self
                .top_customers
                .ok_or_else(|| BuildError::missing_field("top_customers"))?,
        })
    }
}
