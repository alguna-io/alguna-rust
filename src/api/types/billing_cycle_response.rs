pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct BillingCycleResponse {
    /// Cycle end date
    pub end_date: NaiveDate,
    /// Cycle start date
    pub start_date: NaiveDate,
}

impl BillingCycleResponse {
    pub fn builder() -> BillingCycleResponseBuilder {
        <BillingCycleResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BillingCycleResponseBuilder {
    end_date: Option<NaiveDate>,
    start_date: Option<NaiveDate>,
}

impl BillingCycleResponseBuilder {
    pub fn end_date(mut self, value: NaiveDate) -> Self {
        self.end_date = Some(value);
        self
    }

    pub fn start_date(mut self, value: NaiveDate) -> Self {
        self.start_date = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`BillingCycleResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`end_date`](BillingCycleResponseBuilder::end_date)
    /// - [`start_date`](BillingCycleResponseBuilder::start_date)
    pub fn build(self) -> Result<BillingCycleResponse, BuildError> {
        Ok(BillingCycleResponse {
            end_date: self
                .end_date
                .ok_or_else(|| BuildError::missing_field("end_date"))?,
            start_date: self
                .start_date
                .ok_or_else(|| BuildError::missing_field("start_date"))?,
        })
    }
}
