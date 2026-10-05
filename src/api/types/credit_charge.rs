pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreditCharge {
    /// The part of credits that credit covered — drawn on a /track, fenced by the hold on a /check with reserve, zero on a quote or preflight where nothing moves.
    pub covered: String,
    /// What that quantity priced to in credits, at this metric's rate.
    pub credits: String,
    /// The credit ledger the covered part drew from. Empty when nothing was covered, and for a metric that consumes no credit. A charge cascading across ledgers names the primary (first-drawn) ledger; balances carries the per-ledger availability. On an idempotent resend of a settle (a replay), per-metric attribution is not reconstructable from the recorded close, so every charge names the hold's primary ledger — reconcile per-ledger figures from the first response, not a replay.
    pub ledger_id: String,
    /// The billable metric this charge counts against.
    pub metric_id: String,
    /// credits − covered: the part billing as cash, in credits. It is an exposure figure, not a record of movement — on a quote, a denied reserve, or a degraded fail-open nothing moves and covered is zero, so this carries the full credits amount: what the work would bill as cash if no credit paid for it.
    pub overflow: String,
    /// Metric units this call counted for this metric.
    pub quantity: String,
    /// overflow converted back into metric units — the denomination the invoice bills them in. For a metric that consumes no credit, the full quantity.
    pub uncovered_quantity: String,
}

impl CreditCharge {
    pub fn builder() -> CreditChargeBuilder {
        <CreditChargeBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreditChargeBuilder {
    covered: Option<String>,
    credits: Option<String>,
    ledger_id: Option<String>,
    metric_id: Option<String>,
    overflow: Option<String>,
    quantity: Option<String>,
    uncovered_quantity: Option<String>,
}

impl CreditChargeBuilder {
    pub fn covered(mut self, value: impl Into<String>) -> Self {
        self.covered = Some(value.into());
        self
    }

    pub fn credits(mut self, value: impl Into<String>) -> Self {
        self.credits = Some(value.into());
        self
    }

    pub fn ledger_id(mut self, value: impl Into<String>) -> Self {
        self.ledger_id = Some(value.into());
        self
    }

    pub fn metric_id(mut self, value: impl Into<String>) -> Self {
        self.metric_id = Some(value.into());
        self
    }

    pub fn overflow(mut self, value: impl Into<String>) -> Self {
        self.overflow = Some(value.into());
        self
    }

    pub fn quantity(mut self, value: impl Into<String>) -> Self {
        self.quantity = Some(value.into());
        self
    }

    pub fn uncovered_quantity(mut self, value: impl Into<String>) -> Self {
        self.uncovered_quantity = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CreditCharge`].
    /// This method will fail if any of the following fields are not set:
    /// - [`covered`](CreditChargeBuilder::covered)
    /// - [`credits`](CreditChargeBuilder::credits)
    /// - [`ledger_id`](CreditChargeBuilder::ledger_id)
    /// - [`metric_id`](CreditChargeBuilder::metric_id)
    /// - [`overflow`](CreditChargeBuilder::overflow)
    /// - [`quantity`](CreditChargeBuilder::quantity)
    /// - [`uncovered_quantity`](CreditChargeBuilder::uncovered_quantity)
    pub fn build(self) -> Result<CreditCharge, BuildError> {
        Ok(CreditCharge {
            covered: self
                .covered
                .ok_or_else(|| BuildError::missing_field("covered"))?,
            credits: self
                .credits
                .ok_or_else(|| BuildError::missing_field("credits"))?,
            ledger_id: self
                .ledger_id
                .ok_or_else(|| BuildError::missing_field("ledger_id"))?,
            metric_id: self
                .metric_id
                .ok_or_else(|| BuildError::missing_field("metric_id"))?,
            overflow: self
                .overflow
                .ok_or_else(|| BuildError::missing_field("overflow"))?,
            quantity: self
                .quantity
                .ok_or_else(|| BuildError::missing_field("quantity"))?,
            uncovered_quantity: self
                .uncovered_quantity
                .ok_or_else(|| BuildError::missing_field("uncovered_quantity"))?,
        })
    }
}
