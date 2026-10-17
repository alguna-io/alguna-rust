pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreditTotals {
    /// Credits actually covered (drawn or held), summed across every charge.
    pub covered: String,
    /// Credits not covered that bill as cash, summed across every charge. Where nothing was covered — a quote, a denied reserve, a degraded fail-open — this equals used: the cash exposure of the described work if no credit pays for it.
    pub overflow: String,
    /// Metric units that no realtime credit covered and which therefore bill as cash, summed across every charge — each converted at its own metric's rate. Invoice-time credits may still apply to them.
    pub uncovered_metric_quantity: String,
    /// Credits the call priced to, summed across every charge. Computed from the events — never supplied by the caller.
    pub used: String,
}

impl CreditTotals {
    pub fn builder() -> CreditTotalsBuilder {
        <CreditTotalsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreditTotalsBuilder {
    covered: Option<String>,
    overflow: Option<String>,
    uncovered_metric_quantity: Option<String>,
    used: Option<String>,
}

impl CreditTotalsBuilder {
    pub fn covered(mut self, value: impl Into<String>) -> Self {
        self.covered = Some(value.into());
        self
    }

    pub fn overflow(mut self, value: impl Into<String>) -> Self {
        self.overflow = Some(value.into());
        self
    }

    pub fn uncovered_metric_quantity(mut self, value: impl Into<String>) -> Self {
        self.uncovered_metric_quantity = Some(value.into());
        self
    }

    pub fn used(mut self, value: impl Into<String>) -> Self {
        self.used = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CreditTotals`].
    /// This method will fail if any of the following fields are not set:
    /// - [`covered`](CreditTotalsBuilder::covered)
    /// - [`overflow`](CreditTotalsBuilder::overflow)
    /// - [`uncovered_metric_quantity`](CreditTotalsBuilder::uncovered_metric_quantity)
    /// - [`used`](CreditTotalsBuilder::used)
    pub fn build(self) -> Result<CreditTotals, BuildError> {
        Ok(CreditTotals {
            covered: self
                .covered
                .ok_or_else(|| BuildError::missing_field("covered"))?,
            overflow: self
                .overflow
                .ok_or_else(|| BuildError::missing_field("overflow"))?,
            uncovered_metric_quantity: self
                .uncovered_metric_quantity
                .ok_or_else(|| BuildError::missing_field("uncovered_metric_quantity"))?,
            used: self.used.ok_or_else(|| BuildError::missing_field("used"))?,
        })
    }
}
