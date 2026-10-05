pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct SubscriptionRevenueResponse {
    /// Annual contract value
    pub acv: String,
    /// Annual recurring revenue
    pub arr: String,
    /// ISO 4217 currency code
    pub currency: String,
    /// Monthly recurring revenue
    pub mrr: String,
    /// Whether MRR and ARR include all matrix-priced recurring revenue
    pub recurring_revenue_status: RecurringRevenueStatus,
    /// Subscription ID
    pub subscription_id: String,
}

impl SubscriptionRevenueResponse {
    pub fn builder() -> SubscriptionRevenueResponseBuilder {
        <SubscriptionRevenueResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SubscriptionRevenueResponseBuilder {
    acv: Option<String>,
    arr: Option<String>,
    currency: Option<String>,
    mrr: Option<String>,
    recurring_revenue_status: Option<RecurringRevenueStatus>,
    subscription_id: Option<String>,
}

impl SubscriptionRevenueResponseBuilder {
    pub fn acv(mut self, value: impl Into<String>) -> Self {
        self.acv = Some(value.into());
        self
    }

    pub fn arr(mut self, value: impl Into<String>) -> Self {
        self.arr = Some(value.into());
        self
    }

    pub fn currency(mut self, value: impl Into<String>) -> Self {
        self.currency = Some(value.into());
        self
    }

    pub fn mrr(mut self, value: impl Into<String>) -> Self {
        self.mrr = Some(value.into());
        self
    }

    pub fn recurring_revenue_status(mut self, value: RecurringRevenueStatus) -> Self {
        self.recurring_revenue_status = Some(value);
        self
    }

    pub fn subscription_id(mut self, value: impl Into<String>) -> Self {
        self.subscription_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`SubscriptionRevenueResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`acv`](SubscriptionRevenueResponseBuilder::acv)
    /// - [`arr`](SubscriptionRevenueResponseBuilder::arr)
    /// - [`currency`](SubscriptionRevenueResponseBuilder::currency)
    /// - [`mrr`](SubscriptionRevenueResponseBuilder::mrr)
    /// - [`recurring_revenue_status`](SubscriptionRevenueResponseBuilder::recurring_revenue_status)
    /// - [`subscription_id`](SubscriptionRevenueResponseBuilder::subscription_id)
    pub fn build(self) -> Result<SubscriptionRevenueResponse, BuildError> {
        Ok(SubscriptionRevenueResponse {
            acv: self.acv.ok_or_else(|| BuildError::missing_field("acv"))?,
            arr: self.arr.ok_or_else(|| BuildError::missing_field("arr"))?,
            currency: self
                .currency
                .ok_or_else(|| BuildError::missing_field("currency"))?,
            mrr: self.mrr.ok_or_else(|| BuildError::missing_field("mrr"))?,
            recurring_revenue_status: self
                .recurring_revenue_status
                .ok_or_else(|| BuildError::missing_field("recurring_revenue_status"))?,
            subscription_id: self
                .subscription_id
                .ok_or_else(|| BuildError::missing_field("subscription_id"))?,
        })
    }
}
