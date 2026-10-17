pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct CheckCreditsResponse {
    /// Whether the named work is covered by available credit (and, with reserve, whether a hold was opened).
    pub allowed: bool,
    /// The at-zero policy the caller should apply on a deny: stop (hard deny) or overflow (fall through to metered billing). When the work feeds several metrics with differing policies, the strictest (stop) is reported.
    pub at_zero_action: String,
    /// Per-metric, per-ledger availability — one entry per ledger, so a single-ledger account returns one. Always an array, but not always a populated one: it is empty when no availability was resolved to report, which is every response the gate could not read balances for (a back-pressure fail-open, a failed post-hold refresh, a failed report behind a deny) and every response served while the organization's gate is disabled. Check the length before indexing.
    pub balances: Vec<CreditBalance>,
    /// One entry per metric the named work feeds: what the sampled events (or the max_quantity ceiling) price to per metric, and what a hold covers of it. Where no hold was opened — a quote, a denied reserve, a degraded fail-open — covered is zero and overflow carries the whole priced amount as the cash exposure of the described work. A check that named work but sampled none prices to zero throughout. Always an array, length 1 in the single-metric case.
    pub charges: Vec<CreditCharge>,
    /// When the opened hold expires. Present only when a reservation was opened.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<DateTime<FixedOffset>>,
    /// Reservation token to settle later via /track. Empty unless a hold was opened (reserve).
    pub reservation_id: String,
    /// The charges summed. Derived from charges — the breakdown is authoritative.
    pub totals: CreditTotals,
}

impl CheckCreditsResponse {
    pub fn builder() -> CheckCreditsResponseBuilder {
        <CheckCreditsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CheckCreditsResponseBuilder {
    allowed: Option<bool>,
    at_zero_action: Option<String>,
    balances: Option<Vec<CreditBalance>>,
    charges: Option<Vec<CreditCharge>>,
    expires_at: Option<DateTime<FixedOffset>>,
    reservation_id: Option<String>,
    totals: Option<CreditTotals>,
}

impl CheckCreditsResponseBuilder {
    pub fn allowed(mut self, value: bool) -> Self {
        self.allowed = Some(value);
        self
    }

    pub fn at_zero_action(mut self, value: impl Into<String>) -> Self {
        self.at_zero_action = Some(value.into());
        self
    }

    pub fn balances(mut self, value: Vec<CreditBalance>) -> Self {
        self.balances = Some(value);
        self
    }

    pub fn charges(mut self, value: Vec<CreditCharge>) -> Self {
        self.charges = Some(value);
        self
    }

    pub fn expires_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.expires_at = Some(value);
        self
    }

    pub fn reservation_id(mut self, value: impl Into<String>) -> Self {
        self.reservation_id = Some(value.into());
        self
    }

    pub fn totals(mut self, value: CreditTotals) -> Self {
        self.totals = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CheckCreditsResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`allowed`](CheckCreditsResponseBuilder::allowed)
    /// - [`at_zero_action`](CheckCreditsResponseBuilder::at_zero_action)
    /// - [`balances`](CheckCreditsResponseBuilder::balances)
    /// - [`charges`](CheckCreditsResponseBuilder::charges)
    /// - [`reservation_id`](CheckCreditsResponseBuilder::reservation_id)
    /// - [`totals`](CheckCreditsResponseBuilder::totals)
    pub fn build(self) -> Result<CheckCreditsResponse, BuildError> {
        Ok(CheckCreditsResponse {
            allowed: self
                .allowed
                .ok_or_else(|| BuildError::missing_field("allowed"))?,
            at_zero_action: self
                .at_zero_action
                .ok_or_else(|| BuildError::missing_field("at_zero_action"))?,
            balances: self
                .balances
                .ok_or_else(|| BuildError::missing_field("balances"))?,
            charges: self
                .charges
                .ok_or_else(|| BuildError::missing_field("charges"))?,
            expires_at: self.expires_at,
            reservation_id: self
                .reservation_id
                .ok_or_else(|| BuildError::missing_field("reservation_id"))?,
            totals: self
                .totals
                .ok_or_else(|| BuildError::missing_field("totals"))?,
        })
    }
}
