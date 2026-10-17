pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct NetRevenueResponse {
    /// ISO 4217 currency code
    pub currency: String,
    /// Fixed revenue for the previous month
    pub fixed_revenue_last_calendar_month: String,
    /// Fixed (recurring) revenue for the current month
    pub fixed_revenue_this_calendar_month: String,
    /// Annualized fixed revenue based on the current month
    pub fixed_revenue_this_calendar_month_annualized: String,
    /// Annualized fixed revenue from twelve months ago
    pub fixed_revenue_twelve_calendar_months_ago_annualized: String,
    /// Metered revenue for the previous month
    pub metered_revenue_last_calendar_month: String,
    /// Metered (usage-based) revenue for the current month
    pub metered_revenue_this_calendar_month: String,
    /// Annualized metered revenue based on the current month
    pub metered_revenue_this_calendar_month_annualized: String,
    /// Annualized metered revenue from twelve months ago
    pub metered_revenue_twelve_calendar_months_ago_annualized: String,
    /// Net revenue for the previous calendar month
    pub net_revenue_last_calendar_month: String,
    /// Net revenue for the current calendar month
    pub net_revenue_this_calendar_month: String,
    /// Annualized net revenue based on the current month
    pub net_revenue_this_calendar_month_annualized: String,
    /// Annualized net revenue from twelve months ago for comparison
    pub net_revenue_twelve_calendar_months_ago_annualized: String,
    /// Portion of last month's fixed revenue from one-off charges, already included in it
    pub one_off_fixed_revenue_last_calendar_month: String,
    /// Portion of this month's fixed revenue from one-off charges, already included in it
    pub one_off_fixed_revenue_this_calendar_month: String,
    /// Annualized portion of this month's fixed revenue from one-off charges
    pub one_off_fixed_revenue_this_calendar_month_annualized: String,
    /// Annualized one-off fixed revenue from twelve months ago
    pub one_off_fixed_revenue_twelve_calendar_months_ago_annualized: String,
    /// Portion of last month's metered revenue from one-off charges, already included in it
    pub one_off_metered_revenue_last_calendar_month: String,
    /// Portion of this month's metered revenue from one-off charges, already included in it
    pub one_off_metered_revenue_this_calendar_month: String,
    /// Annualized portion of this month's metered revenue from one-off charges
    pub one_off_metered_revenue_this_calendar_month_annualized: String,
    /// Annualized one-off metered revenue from twelve months ago
    pub one_off_metered_revenue_twelve_calendar_months_ago_annualized: String,
}

impl NetRevenueResponse {
    pub fn builder() -> NetRevenueResponseBuilder {
        <NetRevenueResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct NetRevenueResponseBuilder {
    currency: Option<String>,
    fixed_revenue_last_calendar_month: Option<String>,
    fixed_revenue_this_calendar_month: Option<String>,
    fixed_revenue_this_calendar_month_annualized: Option<String>,
    fixed_revenue_twelve_calendar_months_ago_annualized: Option<String>,
    metered_revenue_last_calendar_month: Option<String>,
    metered_revenue_this_calendar_month: Option<String>,
    metered_revenue_this_calendar_month_annualized: Option<String>,
    metered_revenue_twelve_calendar_months_ago_annualized: Option<String>,
    net_revenue_last_calendar_month: Option<String>,
    net_revenue_this_calendar_month: Option<String>,
    net_revenue_this_calendar_month_annualized: Option<String>,
    net_revenue_twelve_calendar_months_ago_annualized: Option<String>,
    one_off_fixed_revenue_last_calendar_month: Option<String>,
    one_off_fixed_revenue_this_calendar_month: Option<String>,
    one_off_fixed_revenue_this_calendar_month_annualized: Option<String>,
    one_off_fixed_revenue_twelve_calendar_months_ago_annualized: Option<String>,
    one_off_metered_revenue_last_calendar_month: Option<String>,
    one_off_metered_revenue_this_calendar_month: Option<String>,
    one_off_metered_revenue_this_calendar_month_annualized: Option<String>,
    one_off_metered_revenue_twelve_calendar_months_ago_annualized: Option<String>,
}

impl NetRevenueResponseBuilder {
    pub fn currency(mut self, value: impl Into<String>) -> Self {
        self.currency = Some(value.into());
        self
    }

    pub fn fixed_revenue_last_calendar_month(mut self, value: impl Into<String>) -> Self {
        self.fixed_revenue_last_calendar_month = Some(value.into());
        self
    }

    pub fn fixed_revenue_this_calendar_month(mut self, value: impl Into<String>) -> Self {
        self.fixed_revenue_this_calendar_month = Some(value.into());
        self
    }

    pub fn fixed_revenue_this_calendar_month_annualized(
        mut self,
        value: impl Into<String>,
    ) -> Self {
        self.fixed_revenue_this_calendar_month_annualized = Some(value.into());
        self
    }

    pub fn fixed_revenue_twelve_calendar_months_ago_annualized(
        mut self,
        value: impl Into<String>,
    ) -> Self {
        self.fixed_revenue_twelve_calendar_months_ago_annualized = Some(value.into());
        self
    }

    pub fn metered_revenue_last_calendar_month(mut self, value: impl Into<String>) -> Self {
        self.metered_revenue_last_calendar_month = Some(value.into());
        self
    }

    pub fn metered_revenue_this_calendar_month(mut self, value: impl Into<String>) -> Self {
        self.metered_revenue_this_calendar_month = Some(value.into());
        self
    }

    pub fn metered_revenue_this_calendar_month_annualized(
        mut self,
        value: impl Into<String>,
    ) -> Self {
        self.metered_revenue_this_calendar_month_annualized = Some(value.into());
        self
    }

    pub fn metered_revenue_twelve_calendar_months_ago_annualized(
        mut self,
        value: impl Into<String>,
    ) -> Self {
        self.metered_revenue_twelve_calendar_months_ago_annualized = Some(value.into());
        self
    }

    pub fn net_revenue_last_calendar_month(mut self, value: impl Into<String>) -> Self {
        self.net_revenue_last_calendar_month = Some(value.into());
        self
    }

    pub fn net_revenue_this_calendar_month(mut self, value: impl Into<String>) -> Self {
        self.net_revenue_this_calendar_month = Some(value.into());
        self
    }

    pub fn net_revenue_this_calendar_month_annualized(mut self, value: impl Into<String>) -> Self {
        self.net_revenue_this_calendar_month_annualized = Some(value.into());
        self
    }

    pub fn net_revenue_twelve_calendar_months_ago_annualized(
        mut self,
        value: impl Into<String>,
    ) -> Self {
        self.net_revenue_twelve_calendar_months_ago_annualized = Some(value.into());
        self
    }

    pub fn one_off_fixed_revenue_last_calendar_month(mut self, value: impl Into<String>) -> Self {
        self.one_off_fixed_revenue_last_calendar_month = Some(value.into());
        self
    }

    pub fn one_off_fixed_revenue_this_calendar_month(mut self, value: impl Into<String>) -> Self {
        self.one_off_fixed_revenue_this_calendar_month = Some(value.into());
        self
    }

    pub fn one_off_fixed_revenue_this_calendar_month_annualized(
        mut self,
        value: impl Into<String>,
    ) -> Self {
        self.one_off_fixed_revenue_this_calendar_month_annualized = Some(value.into());
        self
    }

    pub fn one_off_fixed_revenue_twelve_calendar_months_ago_annualized(
        mut self,
        value: impl Into<String>,
    ) -> Self {
        self.one_off_fixed_revenue_twelve_calendar_months_ago_annualized = Some(value.into());
        self
    }

    pub fn one_off_metered_revenue_last_calendar_month(mut self, value: impl Into<String>) -> Self {
        self.one_off_metered_revenue_last_calendar_month = Some(value.into());
        self
    }

    pub fn one_off_metered_revenue_this_calendar_month(mut self, value: impl Into<String>) -> Self {
        self.one_off_metered_revenue_this_calendar_month = Some(value.into());
        self
    }

    pub fn one_off_metered_revenue_this_calendar_month_annualized(
        mut self,
        value: impl Into<String>,
    ) -> Self {
        self.one_off_metered_revenue_this_calendar_month_annualized = Some(value.into());
        self
    }

    pub fn one_off_metered_revenue_twelve_calendar_months_ago_annualized(
        mut self,
        value: impl Into<String>,
    ) -> Self {
        self.one_off_metered_revenue_twelve_calendar_months_ago_annualized = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`NetRevenueResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`currency`](NetRevenueResponseBuilder::currency)
    /// - [`fixed_revenue_last_calendar_month`](NetRevenueResponseBuilder::fixed_revenue_last_calendar_month)
    /// - [`fixed_revenue_this_calendar_month`](NetRevenueResponseBuilder::fixed_revenue_this_calendar_month)
    /// - [`fixed_revenue_this_calendar_month_annualized`](NetRevenueResponseBuilder::fixed_revenue_this_calendar_month_annualized)
    /// - [`fixed_revenue_twelve_calendar_months_ago_annualized`](NetRevenueResponseBuilder::fixed_revenue_twelve_calendar_months_ago_annualized)
    /// - [`metered_revenue_last_calendar_month`](NetRevenueResponseBuilder::metered_revenue_last_calendar_month)
    /// - [`metered_revenue_this_calendar_month`](NetRevenueResponseBuilder::metered_revenue_this_calendar_month)
    /// - [`metered_revenue_this_calendar_month_annualized`](NetRevenueResponseBuilder::metered_revenue_this_calendar_month_annualized)
    /// - [`metered_revenue_twelve_calendar_months_ago_annualized`](NetRevenueResponseBuilder::metered_revenue_twelve_calendar_months_ago_annualized)
    /// - [`net_revenue_last_calendar_month`](NetRevenueResponseBuilder::net_revenue_last_calendar_month)
    /// - [`net_revenue_this_calendar_month`](NetRevenueResponseBuilder::net_revenue_this_calendar_month)
    /// - [`net_revenue_this_calendar_month_annualized`](NetRevenueResponseBuilder::net_revenue_this_calendar_month_annualized)
    /// - [`net_revenue_twelve_calendar_months_ago_annualized`](NetRevenueResponseBuilder::net_revenue_twelve_calendar_months_ago_annualized)
    /// - [`one_off_fixed_revenue_last_calendar_month`](NetRevenueResponseBuilder::one_off_fixed_revenue_last_calendar_month)
    /// - [`one_off_fixed_revenue_this_calendar_month`](NetRevenueResponseBuilder::one_off_fixed_revenue_this_calendar_month)
    /// - [`one_off_fixed_revenue_this_calendar_month_annualized`](NetRevenueResponseBuilder::one_off_fixed_revenue_this_calendar_month_annualized)
    /// - [`one_off_fixed_revenue_twelve_calendar_months_ago_annualized`](NetRevenueResponseBuilder::one_off_fixed_revenue_twelve_calendar_months_ago_annualized)
    /// - [`one_off_metered_revenue_last_calendar_month`](NetRevenueResponseBuilder::one_off_metered_revenue_last_calendar_month)
    /// - [`one_off_metered_revenue_this_calendar_month`](NetRevenueResponseBuilder::one_off_metered_revenue_this_calendar_month)
    /// - [`one_off_metered_revenue_this_calendar_month_annualized`](NetRevenueResponseBuilder::one_off_metered_revenue_this_calendar_month_annualized)
    /// - [`one_off_metered_revenue_twelve_calendar_months_ago_annualized`](NetRevenueResponseBuilder::one_off_metered_revenue_twelve_calendar_months_ago_annualized)
    pub fn build(self) -> Result<NetRevenueResponse, BuildError> {
        Ok(NetRevenueResponse {
            currency: self
                .currency
                .ok_or_else(|| BuildError::missing_field("currency"))?,
            fixed_revenue_last_calendar_month: self
                .fixed_revenue_last_calendar_month
                .ok_or_else(|| BuildError::missing_field("fixed_revenue_last_calendar_month"))?,
            fixed_revenue_this_calendar_month: self
                .fixed_revenue_this_calendar_month
                .ok_or_else(|| BuildError::missing_field("fixed_revenue_this_calendar_month"))?,
            fixed_revenue_this_calendar_month_annualized: self
                .fixed_revenue_this_calendar_month_annualized
                .ok_or_else(|| {
                    BuildError::missing_field("fixed_revenue_this_calendar_month_annualized")
                })?,
            fixed_revenue_twelve_calendar_months_ago_annualized: self
                .fixed_revenue_twelve_calendar_months_ago_annualized
                .ok_or_else(|| {
                    BuildError::missing_field("fixed_revenue_twelve_calendar_months_ago_annualized")
                })?,
            metered_revenue_last_calendar_month: self
                .metered_revenue_last_calendar_month
                .ok_or_else(|| BuildError::missing_field("metered_revenue_last_calendar_month"))?,
            metered_revenue_this_calendar_month: self
                .metered_revenue_this_calendar_month
                .ok_or_else(|| BuildError::missing_field("metered_revenue_this_calendar_month"))?,
            metered_revenue_this_calendar_month_annualized: self
                .metered_revenue_this_calendar_month_annualized
                .ok_or_else(|| {
                    BuildError::missing_field("metered_revenue_this_calendar_month_annualized")
                })?,
            metered_revenue_twelve_calendar_months_ago_annualized: self
                .metered_revenue_twelve_calendar_months_ago_annualized
                .ok_or_else(|| {
                    BuildError::missing_field(
                        "metered_revenue_twelve_calendar_months_ago_annualized",
                    )
                })?,
            net_revenue_last_calendar_month: self
                .net_revenue_last_calendar_month
                .ok_or_else(|| BuildError::missing_field("net_revenue_last_calendar_month"))?,
            net_revenue_this_calendar_month: self
                .net_revenue_this_calendar_month
                .ok_or_else(|| BuildError::missing_field("net_revenue_this_calendar_month"))?,
            net_revenue_this_calendar_month_annualized: self
                .net_revenue_this_calendar_month_annualized
                .ok_or_else(|| {
                    BuildError::missing_field("net_revenue_this_calendar_month_annualized")
                })?,
            net_revenue_twelve_calendar_months_ago_annualized: self
                .net_revenue_twelve_calendar_months_ago_annualized
                .ok_or_else(|| {
                    BuildError::missing_field("net_revenue_twelve_calendar_months_ago_annualized")
                })?,
            one_off_fixed_revenue_last_calendar_month: self
                .one_off_fixed_revenue_last_calendar_month
                .ok_or_else(|| {
                    BuildError::missing_field("one_off_fixed_revenue_last_calendar_month")
                })?,
            one_off_fixed_revenue_this_calendar_month: self
                .one_off_fixed_revenue_this_calendar_month
                .ok_or_else(|| {
                    BuildError::missing_field("one_off_fixed_revenue_this_calendar_month")
                })?,
            one_off_fixed_revenue_this_calendar_month_annualized: self
                .one_off_fixed_revenue_this_calendar_month_annualized
                .ok_or_else(|| {
                    BuildError::missing_field(
                        "one_off_fixed_revenue_this_calendar_month_annualized",
                    )
                })?,
            one_off_fixed_revenue_twelve_calendar_months_ago_annualized: self
                .one_off_fixed_revenue_twelve_calendar_months_ago_annualized
                .ok_or_else(|| {
                    BuildError::missing_field(
                        "one_off_fixed_revenue_twelve_calendar_months_ago_annualized",
                    )
                })?,
            one_off_metered_revenue_last_calendar_month: self
                .one_off_metered_revenue_last_calendar_month
                .ok_or_else(|| {
                    BuildError::missing_field("one_off_metered_revenue_last_calendar_month")
                })?,
            one_off_metered_revenue_this_calendar_month: self
                .one_off_metered_revenue_this_calendar_month
                .ok_or_else(|| {
                    BuildError::missing_field("one_off_metered_revenue_this_calendar_month")
                })?,
            one_off_metered_revenue_this_calendar_month_annualized: self
                .one_off_metered_revenue_this_calendar_month_annualized
                .ok_or_else(|| {
                    BuildError::missing_field(
                        "one_off_metered_revenue_this_calendar_month_annualized",
                    )
                })?,
            one_off_metered_revenue_twelve_calendar_months_ago_annualized: self
                .one_off_metered_revenue_twelve_calendar_months_ago_annualized
                .ok_or_else(|| {
                    BuildError::missing_field(
                        "one_off_metered_revenue_twelve_calendar_months_ago_annualized",
                    )
                })?,
        })
    }
}
