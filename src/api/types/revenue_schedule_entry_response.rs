pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct RevenueScheduleEntryResponse {
    /// End date of the entry period
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub end_date: DateTime<FixedOffset>,
    /// Change in contract asset (unbilled revenue) for this period
    pub new_contract_asset: String,
    /// Change in deferred revenue for this period
    pub new_deferred_revenue: String,
    /// Change in fixed revenue for this period
    pub new_fixed_revenue: String,
    /// Change in metered revenue for this period
    pub new_metered_revenue: String,
    /// Change in one-off revenue for this period
    pub new_one_off_revenue: String,
    /// Change in recognized revenue (fixed + metered) for this period
    pub new_recognized_revenue: String,
    /// Start date of the entry period
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub start_date: DateTime<FixedOffset>,
    /// Cumulative contract asset
    pub total_contract_asset: String,
    /// Cumulative deferred revenue
    pub total_deferred_revenue: String,
    /// Cumulative fixed revenue
    pub total_fixed_revenue: String,
    /// Cumulative metered revenue
    pub total_metered_revenue: String,
    /// Cumulative one-off revenue
    pub total_one_off_revenue: String,
    /// Cumulative recognized revenue
    pub total_recognized_revenue: String,
}

impl RevenueScheduleEntryResponse {
    pub fn builder() -> RevenueScheduleEntryResponseBuilder {
        <RevenueScheduleEntryResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RevenueScheduleEntryResponseBuilder {
    end_date: Option<DateTime<FixedOffset>>,
    new_contract_asset: Option<String>,
    new_deferred_revenue: Option<String>,
    new_fixed_revenue: Option<String>,
    new_metered_revenue: Option<String>,
    new_one_off_revenue: Option<String>,
    new_recognized_revenue: Option<String>,
    start_date: Option<DateTime<FixedOffset>>,
    total_contract_asset: Option<String>,
    total_deferred_revenue: Option<String>,
    total_fixed_revenue: Option<String>,
    total_metered_revenue: Option<String>,
    total_one_off_revenue: Option<String>,
    total_recognized_revenue: Option<String>,
}

impl RevenueScheduleEntryResponseBuilder {
    pub fn end_date(mut self, value: DateTime<FixedOffset>) -> Self {
        self.end_date = Some(value);
        self
    }

    pub fn new_contract_asset(mut self, value: impl Into<String>) -> Self {
        self.new_contract_asset = Some(value.into());
        self
    }

    pub fn new_deferred_revenue(mut self, value: impl Into<String>) -> Self {
        self.new_deferred_revenue = Some(value.into());
        self
    }

    pub fn new_fixed_revenue(mut self, value: impl Into<String>) -> Self {
        self.new_fixed_revenue = Some(value.into());
        self
    }

    pub fn new_metered_revenue(mut self, value: impl Into<String>) -> Self {
        self.new_metered_revenue = Some(value.into());
        self
    }

    pub fn new_one_off_revenue(mut self, value: impl Into<String>) -> Self {
        self.new_one_off_revenue = Some(value.into());
        self
    }

    pub fn new_recognized_revenue(mut self, value: impl Into<String>) -> Self {
        self.new_recognized_revenue = Some(value.into());
        self
    }

    pub fn start_date(mut self, value: DateTime<FixedOffset>) -> Self {
        self.start_date = Some(value);
        self
    }

    pub fn total_contract_asset(mut self, value: impl Into<String>) -> Self {
        self.total_contract_asset = Some(value.into());
        self
    }

    pub fn total_deferred_revenue(mut self, value: impl Into<String>) -> Self {
        self.total_deferred_revenue = Some(value.into());
        self
    }

    pub fn total_fixed_revenue(mut self, value: impl Into<String>) -> Self {
        self.total_fixed_revenue = Some(value.into());
        self
    }

    pub fn total_metered_revenue(mut self, value: impl Into<String>) -> Self {
        self.total_metered_revenue = Some(value.into());
        self
    }

    pub fn total_one_off_revenue(mut self, value: impl Into<String>) -> Self {
        self.total_one_off_revenue = Some(value.into());
        self
    }

    pub fn total_recognized_revenue(mut self, value: impl Into<String>) -> Self {
        self.total_recognized_revenue = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`RevenueScheduleEntryResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`end_date`](RevenueScheduleEntryResponseBuilder::end_date)
    /// - [`new_contract_asset`](RevenueScheduleEntryResponseBuilder::new_contract_asset)
    /// - [`new_deferred_revenue`](RevenueScheduleEntryResponseBuilder::new_deferred_revenue)
    /// - [`new_fixed_revenue`](RevenueScheduleEntryResponseBuilder::new_fixed_revenue)
    /// - [`new_metered_revenue`](RevenueScheduleEntryResponseBuilder::new_metered_revenue)
    /// - [`new_one_off_revenue`](RevenueScheduleEntryResponseBuilder::new_one_off_revenue)
    /// - [`new_recognized_revenue`](RevenueScheduleEntryResponseBuilder::new_recognized_revenue)
    /// - [`start_date`](RevenueScheduleEntryResponseBuilder::start_date)
    /// - [`total_contract_asset`](RevenueScheduleEntryResponseBuilder::total_contract_asset)
    /// - [`total_deferred_revenue`](RevenueScheduleEntryResponseBuilder::total_deferred_revenue)
    /// - [`total_fixed_revenue`](RevenueScheduleEntryResponseBuilder::total_fixed_revenue)
    /// - [`total_metered_revenue`](RevenueScheduleEntryResponseBuilder::total_metered_revenue)
    /// - [`total_one_off_revenue`](RevenueScheduleEntryResponseBuilder::total_one_off_revenue)
    /// - [`total_recognized_revenue`](RevenueScheduleEntryResponseBuilder::total_recognized_revenue)
    pub fn build(self) -> Result<RevenueScheduleEntryResponse, BuildError> {
        Ok(RevenueScheduleEntryResponse {
            end_date: self
                .end_date
                .ok_or_else(|| BuildError::missing_field("end_date"))?,
            new_contract_asset: self
                .new_contract_asset
                .ok_or_else(|| BuildError::missing_field("new_contract_asset"))?,
            new_deferred_revenue: self
                .new_deferred_revenue
                .ok_or_else(|| BuildError::missing_field("new_deferred_revenue"))?,
            new_fixed_revenue: self
                .new_fixed_revenue
                .ok_or_else(|| BuildError::missing_field("new_fixed_revenue"))?,
            new_metered_revenue: self
                .new_metered_revenue
                .ok_or_else(|| BuildError::missing_field("new_metered_revenue"))?,
            new_one_off_revenue: self
                .new_one_off_revenue
                .ok_or_else(|| BuildError::missing_field("new_one_off_revenue"))?,
            new_recognized_revenue: self
                .new_recognized_revenue
                .ok_or_else(|| BuildError::missing_field("new_recognized_revenue"))?,
            start_date: self
                .start_date
                .ok_or_else(|| BuildError::missing_field("start_date"))?,
            total_contract_asset: self
                .total_contract_asset
                .ok_or_else(|| BuildError::missing_field("total_contract_asset"))?,
            total_deferred_revenue: self
                .total_deferred_revenue
                .ok_or_else(|| BuildError::missing_field("total_deferred_revenue"))?,
            total_fixed_revenue: self
                .total_fixed_revenue
                .ok_or_else(|| BuildError::missing_field("total_fixed_revenue"))?,
            total_metered_revenue: self
                .total_metered_revenue
                .ok_or_else(|| BuildError::missing_field("total_metered_revenue"))?,
            total_one_off_revenue: self
                .total_one_off_revenue
                .ok_or_else(|| BuildError::missing_field("total_one_off_revenue"))?,
            total_recognized_revenue: self
                .total_recognized_revenue
                .ok_or_else(|| BuildError::missing_field("total_recognized_revenue"))?,
        })
    }
}
