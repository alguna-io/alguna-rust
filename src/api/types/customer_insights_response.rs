pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CustomerInsightsResponse {
    /// Number of currently active customers
    pub active_customer_count: i64,
    /// Number of active customers in the previous period
    pub active_customer_count_previous: i64,
    /// Customers who churned in the previous period
    pub churned_previous_period: i64,
    /// Customers who churned in the current period
    pub churned_this_period: i64,
    /// New customers in the previous period
    pub new_previous_period: i64,
    /// New customers in the current period
    pub new_this_period: i64,
    /// Reactivated customers in the previous period
    pub reactivated_previous_period: i64,
    /// Reactivated customers in the current period
    pub reactivated_this_period: i64,
}

impl CustomerInsightsResponse {
    pub fn builder() -> CustomerInsightsResponseBuilder {
        <CustomerInsightsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CustomerInsightsResponseBuilder {
    active_customer_count: Option<i64>,
    active_customer_count_previous: Option<i64>,
    churned_previous_period: Option<i64>,
    churned_this_period: Option<i64>,
    new_previous_period: Option<i64>,
    new_this_period: Option<i64>,
    reactivated_previous_period: Option<i64>,
    reactivated_this_period: Option<i64>,
}

impl CustomerInsightsResponseBuilder {
    pub fn active_customer_count(mut self, value: i64) -> Self {
        self.active_customer_count = Some(value);
        self
    }

    pub fn active_customer_count_previous(mut self, value: i64) -> Self {
        self.active_customer_count_previous = Some(value);
        self
    }

    pub fn churned_previous_period(mut self, value: i64) -> Self {
        self.churned_previous_period = Some(value);
        self
    }

    pub fn churned_this_period(mut self, value: i64) -> Self {
        self.churned_this_period = Some(value);
        self
    }

    pub fn new_previous_period(mut self, value: i64) -> Self {
        self.new_previous_period = Some(value);
        self
    }

    pub fn new_this_period(mut self, value: i64) -> Self {
        self.new_this_period = Some(value);
        self
    }

    pub fn reactivated_previous_period(mut self, value: i64) -> Self {
        self.reactivated_previous_period = Some(value);
        self
    }

    pub fn reactivated_this_period(mut self, value: i64) -> Self {
        self.reactivated_this_period = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CustomerInsightsResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`active_customer_count`](CustomerInsightsResponseBuilder::active_customer_count)
    /// - [`active_customer_count_previous`](CustomerInsightsResponseBuilder::active_customer_count_previous)
    /// - [`churned_previous_period`](CustomerInsightsResponseBuilder::churned_previous_period)
    /// - [`churned_this_period`](CustomerInsightsResponseBuilder::churned_this_period)
    /// - [`new_previous_period`](CustomerInsightsResponseBuilder::new_previous_period)
    /// - [`new_this_period`](CustomerInsightsResponseBuilder::new_this_period)
    /// - [`reactivated_previous_period`](CustomerInsightsResponseBuilder::reactivated_previous_period)
    /// - [`reactivated_this_period`](CustomerInsightsResponseBuilder::reactivated_this_period)
    pub fn build(self) -> Result<CustomerInsightsResponse, BuildError> {
        Ok(CustomerInsightsResponse {
            active_customer_count: self
                .active_customer_count
                .ok_or_else(|| BuildError::missing_field("active_customer_count"))?,
            active_customer_count_previous: self
                .active_customer_count_previous
                .ok_or_else(|| BuildError::missing_field("active_customer_count_previous"))?,
            churned_previous_period: self
                .churned_previous_period
                .ok_or_else(|| BuildError::missing_field("churned_previous_period"))?,
            churned_this_period: self
                .churned_this_period
                .ok_or_else(|| BuildError::missing_field("churned_this_period"))?,
            new_previous_period: self
                .new_previous_period
                .ok_or_else(|| BuildError::missing_field("new_previous_period"))?,
            new_this_period: self
                .new_this_period
                .ok_or_else(|| BuildError::missing_field("new_this_period"))?,
            reactivated_previous_period: self
                .reactivated_previous_period
                .ok_or_else(|| BuildError::missing_field("reactivated_previous_period"))?,
            reactivated_this_period: self
                .reactivated_this_period
                .ok_or_else(|| BuildError::missing_field("reactivated_this_period"))?,
        })
    }
}
