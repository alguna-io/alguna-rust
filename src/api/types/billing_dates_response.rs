pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct BillingDatesResponse {
    /// Contract end date
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contract_end_date: Option<NaiveDate>,
    /// Current billing cycle date range
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_cycle: Option<BillingCycleResponse>,
    /// Next billing cycle start date
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_billing_date: Option<NaiveDate>,
    /// Next renewal date
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_renewal_date: Option<NaiveDate>,
    /// Valid date range for mid-cycle changes
    #[serde(skip_serializing_if = "Option::is_none")]
    pub valid_mid_cycle_range: Option<BillingCycleResponse>,
    /// Valid dates for on-cycle version starts
    pub valid_on_cycle_dates: Vec<NaiveDate>,
}

impl BillingDatesResponse {
    pub fn builder() -> BillingDatesResponseBuilder {
        <BillingDatesResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BillingDatesResponseBuilder {
    contract_end_date: Option<NaiveDate>,
    current_cycle: Option<BillingCycleResponse>,
    next_billing_date: Option<NaiveDate>,
    next_renewal_date: Option<NaiveDate>,
    valid_mid_cycle_range: Option<BillingCycleResponse>,
    valid_on_cycle_dates: Option<Vec<NaiveDate>>,
}

impl BillingDatesResponseBuilder {
    pub fn contract_end_date(mut self, value: NaiveDate) -> Self {
        self.contract_end_date = Some(value);
        self
    }

    pub fn current_cycle(mut self, value: BillingCycleResponse) -> Self {
        self.current_cycle = Some(value);
        self
    }

    pub fn next_billing_date(mut self, value: NaiveDate) -> Self {
        self.next_billing_date = Some(value);
        self
    }

    pub fn next_renewal_date(mut self, value: NaiveDate) -> Self {
        self.next_renewal_date = Some(value);
        self
    }

    pub fn valid_mid_cycle_range(mut self, value: BillingCycleResponse) -> Self {
        self.valid_mid_cycle_range = Some(value);
        self
    }

    pub fn valid_on_cycle_dates(mut self, value: Vec<NaiveDate>) -> Self {
        self.valid_on_cycle_dates = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`BillingDatesResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`valid_on_cycle_dates`](BillingDatesResponseBuilder::valid_on_cycle_dates)
    pub fn build(self) -> Result<BillingDatesResponse, BuildError> {
        Ok(BillingDatesResponse {
            contract_end_date: self.contract_end_date,
            current_cycle: self.current_cycle,
            next_billing_date: self.next_billing_date,
            next_renewal_date: self.next_renewal_date,
            valid_mid_cycle_range: self.valid_mid_cycle_range,
            valid_on_cycle_dates: self
                .valid_on_cycle_dates
                .ok_or_else(|| BuildError::missing_field("valid_on_cycle_dates"))?,
        })
    }
}
