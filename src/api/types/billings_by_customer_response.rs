pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct BillingsByCustomerResponse {
    /// ISO 4217 currency code
    pub currency: String,
    /// Aggregated billings from remaining customers for the comparison period
    pub other_customers_net_billings_previous_period: String,
    /// Aggregated billings from remaining customers for the selected period
    pub other_customers_net_billings_this_period: String,
    /// Top customers by billings
    pub top_customers: Vec<InsightsCustomerEntry>,
}

impl BillingsByCustomerResponse {
    pub fn builder() -> BillingsByCustomerResponseBuilder {
        <BillingsByCustomerResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BillingsByCustomerResponseBuilder {
    currency: Option<String>,
    other_customers_net_billings_previous_period: Option<String>,
    other_customers_net_billings_this_period: Option<String>,
    top_customers: Option<Vec<InsightsCustomerEntry>>,
}

impl BillingsByCustomerResponseBuilder {
    pub fn currency(mut self, value: impl Into<String>) -> Self {
        self.currency = Some(value.into());
        self
    }

    pub fn other_customers_net_billings_previous_period(
        mut self,
        value: impl Into<String>,
    ) -> Self {
        self.other_customers_net_billings_previous_period = Some(value.into());
        self
    }

    pub fn other_customers_net_billings_this_period(mut self, value: impl Into<String>) -> Self {
        self.other_customers_net_billings_this_period = Some(value.into());
        self
    }

    pub fn top_customers(mut self, value: Vec<InsightsCustomerEntry>) -> Self {
        self.top_customers = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`BillingsByCustomerResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`currency`](BillingsByCustomerResponseBuilder::currency)
    /// - [`other_customers_net_billings_previous_period`](BillingsByCustomerResponseBuilder::other_customers_net_billings_previous_period)
    /// - [`other_customers_net_billings_this_period`](BillingsByCustomerResponseBuilder::other_customers_net_billings_this_period)
    /// - [`top_customers`](BillingsByCustomerResponseBuilder::top_customers)
    pub fn build(self) -> Result<BillingsByCustomerResponse, BuildError> {
        Ok(BillingsByCustomerResponse {
            currency: self
                .currency
                .ok_or_else(|| BuildError::missing_field("currency"))?,
            other_customers_net_billings_previous_period: self
                .other_customers_net_billings_previous_period
                .ok_or_else(|| {
                    BuildError::missing_field("other_customers_net_billings_previous_period")
                })?,
            other_customers_net_billings_this_period: self
                .other_customers_net_billings_this_period
                .ok_or_else(|| {
                    BuildError::missing_field("other_customers_net_billings_this_period")
                })?,
            top_customers: self
                .top_customers
                .ok_or_else(|| BuildError::missing_field("top_customers"))?,
        })
    }
}
