pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct RevenueScheduleReportResponse {
    /// ISO 4217 currency code
    pub currency: String,
    /// End date of the schedule
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub end_date: DateTime<FixedOffset>,
    /// Chronologically ordered schedule entries
    pub entries: Vec<RevenueScheduleEntryResponse>,
    /// Time interval of entries
    pub interval: RevenueScheduleReportResponseInterval,
    /// Start date of the schedule
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub start_date: DateTime<FixedOffset>,
}

impl RevenueScheduleReportResponse {
    pub fn builder() -> RevenueScheduleReportResponseBuilder {
        <RevenueScheduleReportResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RevenueScheduleReportResponseBuilder {
    currency: Option<String>,
    end_date: Option<DateTime<FixedOffset>>,
    entries: Option<Vec<RevenueScheduleEntryResponse>>,
    interval: Option<RevenueScheduleReportResponseInterval>,
    start_date: Option<DateTime<FixedOffset>>,
}

impl RevenueScheduleReportResponseBuilder {
    pub fn currency(mut self, value: impl Into<String>) -> Self {
        self.currency = Some(value.into());
        self
    }

    pub fn end_date(mut self, value: DateTime<FixedOffset>) -> Self {
        self.end_date = Some(value);
        self
    }

    pub fn entries(mut self, value: Vec<RevenueScheduleEntryResponse>) -> Self {
        self.entries = Some(value);
        self
    }

    pub fn interval(mut self, value: RevenueScheduleReportResponseInterval) -> Self {
        self.interval = Some(value);
        self
    }

    pub fn start_date(mut self, value: DateTime<FixedOffset>) -> Self {
        self.start_date = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`RevenueScheduleReportResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`currency`](RevenueScheduleReportResponseBuilder::currency)
    /// - [`end_date`](RevenueScheduleReportResponseBuilder::end_date)
    /// - [`entries`](RevenueScheduleReportResponseBuilder::entries)
    /// - [`interval`](RevenueScheduleReportResponseBuilder::interval)
    /// - [`start_date`](RevenueScheduleReportResponseBuilder::start_date)
    pub fn build(self) -> Result<RevenueScheduleReportResponse, BuildError> {
        Ok(RevenueScheduleReportResponse {
            currency: self
                .currency
                .ok_or_else(|| BuildError::missing_field("currency"))?,
            end_date: self
                .end_date
                .ok_or_else(|| BuildError::missing_field("end_date"))?,
            entries: self
                .entries
                .ok_or_else(|| BuildError::missing_field("entries"))?,
            interval: self
                .interval
                .ok_or_else(|| BuildError::missing_field("interval"))?,
            start_date: self
                .start_date
                .ok_or_else(|| BuildError::missing_field("start_date"))?,
        })
    }
}
