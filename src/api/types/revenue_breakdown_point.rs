pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct RevenueBreakdownPoint {
    /// Revenue lost from customer churn
    pub churn_revenue: String,
    /// Revenue lost from customer downgrades
    pub contraction_revenue: String,
    /// End of the period
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub end_date: DateTime<FixedOffset>,
    /// Revenue from existing customers with stable contracts
    pub existing_revenue: String,
    /// Revenue from customer upgrades or expansions
    pub expansion_revenue: String,
    /// Revenue from new customers
    pub new_revenue: String,
    /// Start of the period
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub start_date: DateTime<FixedOffset>,
}

impl RevenueBreakdownPoint {
    pub fn builder() -> RevenueBreakdownPointBuilder {
        <RevenueBreakdownPointBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RevenueBreakdownPointBuilder {
    churn_revenue: Option<String>,
    contraction_revenue: Option<String>,
    end_date: Option<DateTime<FixedOffset>>,
    existing_revenue: Option<String>,
    expansion_revenue: Option<String>,
    new_revenue: Option<String>,
    start_date: Option<DateTime<FixedOffset>>,
}

impl RevenueBreakdownPointBuilder {
    pub fn churn_revenue(mut self, value: impl Into<String>) -> Self {
        self.churn_revenue = Some(value.into());
        self
    }

    pub fn contraction_revenue(mut self, value: impl Into<String>) -> Self {
        self.contraction_revenue = Some(value.into());
        self
    }

    pub fn end_date(mut self, value: DateTime<FixedOffset>) -> Self {
        self.end_date = Some(value);
        self
    }

    pub fn existing_revenue(mut self, value: impl Into<String>) -> Self {
        self.existing_revenue = Some(value.into());
        self
    }

    pub fn expansion_revenue(mut self, value: impl Into<String>) -> Self {
        self.expansion_revenue = Some(value.into());
        self
    }

    pub fn new_revenue(mut self, value: impl Into<String>) -> Self {
        self.new_revenue = Some(value.into());
        self
    }

    pub fn start_date(mut self, value: DateTime<FixedOffset>) -> Self {
        self.start_date = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`RevenueBreakdownPoint`].
    /// This method will fail if any of the following fields are not set:
    /// - [`churn_revenue`](RevenueBreakdownPointBuilder::churn_revenue)
    /// - [`contraction_revenue`](RevenueBreakdownPointBuilder::contraction_revenue)
    /// - [`end_date`](RevenueBreakdownPointBuilder::end_date)
    /// - [`existing_revenue`](RevenueBreakdownPointBuilder::existing_revenue)
    /// - [`expansion_revenue`](RevenueBreakdownPointBuilder::expansion_revenue)
    /// - [`new_revenue`](RevenueBreakdownPointBuilder::new_revenue)
    /// - [`start_date`](RevenueBreakdownPointBuilder::start_date)
    pub fn build(self) -> Result<RevenueBreakdownPoint, BuildError> {
        Ok(RevenueBreakdownPoint {
            churn_revenue: self
                .churn_revenue
                .ok_or_else(|| BuildError::missing_field("churn_revenue"))?,
            contraction_revenue: self
                .contraction_revenue
                .ok_or_else(|| BuildError::missing_field("contraction_revenue"))?,
            end_date: self
                .end_date
                .ok_or_else(|| BuildError::missing_field("end_date"))?,
            existing_revenue: self
                .existing_revenue
                .ok_or_else(|| BuildError::missing_field("existing_revenue"))?,
            expansion_revenue: self
                .expansion_revenue
                .ok_or_else(|| BuildError::missing_field("expansion_revenue"))?,
            new_revenue: self
                .new_revenue
                .ok_or_else(|| BuildError::missing_field("new_revenue"))?,
            start_date: self
                .start_date
                .ok_or_else(|| BuildError::missing_field("start_date"))?,
        })
    }
}
