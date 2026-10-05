pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct RevenueBySubscriptionResponse {
    /// Annual contract value (ACV)
    pub annual_contract_value: String,
    /// Annual recurring revenue (ARR)
    pub annual_recurring_revenue: String,
    /// ISO 4217 currency code
    pub currency: String,
    /// Monthly recurring revenue (MRR)
    pub monthly_recurring_revenue: String,
}

impl RevenueBySubscriptionResponse {
    pub fn builder() -> RevenueBySubscriptionResponseBuilder {
        <RevenueBySubscriptionResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RevenueBySubscriptionResponseBuilder {
    annual_contract_value: Option<String>,
    annual_recurring_revenue: Option<String>,
    currency: Option<String>,
    monthly_recurring_revenue: Option<String>,
}

impl RevenueBySubscriptionResponseBuilder {
    pub fn annual_contract_value(mut self, value: impl Into<String>) -> Self {
        self.annual_contract_value = Some(value.into());
        self
    }

    pub fn annual_recurring_revenue(mut self, value: impl Into<String>) -> Self {
        self.annual_recurring_revenue = Some(value.into());
        self
    }

    pub fn currency(mut self, value: impl Into<String>) -> Self {
        self.currency = Some(value.into());
        self
    }

    pub fn monthly_recurring_revenue(mut self, value: impl Into<String>) -> Self {
        self.monthly_recurring_revenue = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`RevenueBySubscriptionResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`annual_contract_value`](RevenueBySubscriptionResponseBuilder::annual_contract_value)
    /// - [`annual_recurring_revenue`](RevenueBySubscriptionResponseBuilder::annual_recurring_revenue)
    /// - [`currency`](RevenueBySubscriptionResponseBuilder::currency)
    /// - [`monthly_recurring_revenue`](RevenueBySubscriptionResponseBuilder::monthly_recurring_revenue)
    pub fn build(self) -> Result<RevenueBySubscriptionResponse, BuildError> {
        Ok(RevenueBySubscriptionResponse {
            annual_contract_value: self
                .annual_contract_value
                .ok_or_else(|| BuildError::missing_field("annual_contract_value"))?,
            annual_recurring_revenue: self
                .annual_recurring_revenue
                .ok_or_else(|| BuildError::missing_field("annual_recurring_revenue"))?,
            currency: self
                .currency
                .ok_or_else(|| BuildError::missing_field("currency"))?,
            monthly_recurring_revenue: self
                .monthly_recurring_revenue
                .ok_or_else(|| BuildError::missing_field("monthly_recurring_revenue"))?,
        })
    }
}
