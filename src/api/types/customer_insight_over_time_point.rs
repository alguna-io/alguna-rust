pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CustomerInsightOverTimePoint {
    /// Number of customers who churned in this period
    pub churned_customer_count: i64,
    /// End of the period
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub end_date: DateTime<FixedOffset>,
    /// Number of existing active customers at end of period
    pub existing_customer_count: i64,
    /// Number of new customers gained in this period
    pub new_customer_count: i64,
    /// Number of reactivated customers in this period
    pub reactivated_customer_count: i64,
    /// Start of the period
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub start_date: DateTime<FixedOffset>,
}

impl CustomerInsightOverTimePoint {
    pub fn builder() -> CustomerInsightOverTimePointBuilder {
        <CustomerInsightOverTimePointBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CustomerInsightOverTimePointBuilder {
    churned_customer_count: Option<i64>,
    end_date: Option<DateTime<FixedOffset>>,
    existing_customer_count: Option<i64>,
    new_customer_count: Option<i64>,
    reactivated_customer_count: Option<i64>,
    start_date: Option<DateTime<FixedOffset>>,
}

impl CustomerInsightOverTimePointBuilder {
    pub fn churned_customer_count(mut self, value: i64) -> Self {
        self.churned_customer_count = Some(value);
        self
    }

    pub fn end_date(mut self, value: DateTime<FixedOffset>) -> Self {
        self.end_date = Some(value);
        self
    }

    pub fn existing_customer_count(mut self, value: i64) -> Self {
        self.existing_customer_count = Some(value);
        self
    }

    pub fn new_customer_count(mut self, value: i64) -> Self {
        self.new_customer_count = Some(value);
        self
    }

    pub fn reactivated_customer_count(mut self, value: i64) -> Self {
        self.reactivated_customer_count = Some(value);
        self
    }

    pub fn start_date(mut self, value: DateTime<FixedOffset>) -> Self {
        self.start_date = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CustomerInsightOverTimePoint`].
    /// This method will fail if any of the following fields are not set:
    /// - [`churned_customer_count`](CustomerInsightOverTimePointBuilder::churned_customer_count)
    /// - [`end_date`](CustomerInsightOverTimePointBuilder::end_date)
    /// - [`existing_customer_count`](CustomerInsightOverTimePointBuilder::existing_customer_count)
    /// - [`new_customer_count`](CustomerInsightOverTimePointBuilder::new_customer_count)
    /// - [`reactivated_customer_count`](CustomerInsightOverTimePointBuilder::reactivated_customer_count)
    /// - [`start_date`](CustomerInsightOverTimePointBuilder::start_date)
    pub fn build(self) -> Result<CustomerInsightOverTimePoint, BuildError> {
        Ok(CustomerInsightOverTimePoint {
            churned_customer_count: self
                .churned_customer_count
                .ok_or_else(|| BuildError::missing_field("churned_customer_count"))?,
            end_date: self
                .end_date
                .ok_or_else(|| BuildError::missing_field("end_date"))?,
            existing_customer_count: self
                .existing_customer_count
                .ok_or_else(|| BuildError::missing_field("existing_customer_count"))?,
            new_customer_count: self
                .new_customer_count
                .ok_or_else(|| BuildError::missing_field("new_customer_count"))?,
            reactivated_customer_count: self
                .reactivated_customer_count
                .ok_or_else(|| BuildError::missing_field("reactivated_customer_count"))?,
            start_date: self
                .start_date
                .ok_or_else(|| BuildError::missing_field("start_date"))?,
        })
    }
}
