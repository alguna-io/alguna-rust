pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct TrackCreditsResponse {
    /// Whether the consumption was covered by available credit
    pub allowed: bool,
    /// The reservation's at-zero policy, reported when the envelope was exceeded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub at_zero_action: Option<String>,
    /// Per-metric, per-ledger availability after this call — one entry per ledger, so a single-ledger payload returns one. Always an array, but empty when no availability was resolved to report, including every response served while the organization's gate is disabled. Check the length before indexing.
    pub balances: Vec<CreditBalance>,
    /// One entry per metric this call's events fed: quantity counted, credits priced, credit covered, overflow to cash, and the ledger that paid. Always an array, length 1 in the ordinary single-metric case.
    pub charges: Vec<CreditCharge>,
    /// True when the events priced past the reservation's remaining hold and were recorded anyway because on_envelope_exceeded was record. Worth alarming on for a feature that stops at zero.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub envelope_exceeded: Option<bool>,
    /// How many events this call recorded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub events_accepted: Option<i64>,
    /// How many of those events at least one metric's filters counted toward a charge. A lower number than events_accepted means some events did not qualify, without exposing the filter itself.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub events_priced: Option<i64>,
    /// True when the referenced reservation had expired and the work was re-resolved as a fresh consume
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reservation_expired: Option<bool>,
    /// When the still-open reservation now expires. Each accepted call extends this, so a long run that reports as it goes never expires mid-flight. Absent once the reservation closes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reservation_expires_at: Option<DateTime<FixedOffset>>,
    /// Reservation this call settled or opened. Empty for a direct-consumption draw that opened no block.
    pub reservation_id: String,
    /// Whether the reservation is still open and can be settled again. False even on a close: false call whose events consumed everything held — there is no envelope left to keep open.
    pub reservation_open: bool,
    /// Credits still held by this reservation and available to later calls.
    pub reservation_remaining: String,
    /// The charges summed. Derived from charges — the breakdown is authoritative.
    pub totals: CreditTotals,
}

impl TrackCreditsResponse {
    pub fn builder() -> TrackCreditsResponseBuilder {
        <TrackCreditsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TrackCreditsResponseBuilder {
    allowed: Option<bool>,
    at_zero_action: Option<String>,
    balances: Option<Vec<CreditBalance>>,
    charges: Option<Vec<CreditCharge>>,
    envelope_exceeded: Option<bool>,
    events_accepted: Option<i64>,
    events_priced: Option<i64>,
    reservation_expired: Option<bool>,
    reservation_expires_at: Option<DateTime<FixedOffset>>,
    reservation_id: Option<String>,
    reservation_open: Option<bool>,
    reservation_remaining: Option<String>,
    totals: Option<CreditTotals>,
}

impl TrackCreditsResponseBuilder {
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

    pub fn envelope_exceeded(mut self, value: bool) -> Self {
        self.envelope_exceeded = Some(value);
        self
    }

    pub fn events_accepted(mut self, value: i64) -> Self {
        self.events_accepted = Some(value);
        self
    }

    pub fn events_priced(mut self, value: i64) -> Self {
        self.events_priced = Some(value);
        self
    }

    pub fn reservation_expired(mut self, value: bool) -> Self {
        self.reservation_expired = Some(value);
        self
    }

    pub fn reservation_expires_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.reservation_expires_at = Some(value);
        self
    }

    pub fn reservation_id(mut self, value: impl Into<String>) -> Self {
        self.reservation_id = Some(value.into());
        self
    }

    pub fn reservation_open(mut self, value: bool) -> Self {
        self.reservation_open = Some(value);
        self
    }

    pub fn reservation_remaining(mut self, value: impl Into<String>) -> Self {
        self.reservation_remaining = Some(value.into());
        self
    }

    pub fn totals(mut self, value: CreditTotals) -> Self {
        self.totals = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TrackCreditsResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`allowed`](TrackCreditsResponseBuilder::allowed)
    /// - [`balances`](TrackCreditsResponseBuilder::balances)
    /// - [`charges`](TrackCreditsResponseBuilder::charges)
    /// - [`reservation_id`](TrackCreditsResponseBuilder::reservation_id)
    /// - [`reservation_open`](TrackCreditsResponseBuilder::reservation_open)
    /// - [`reservation_remaining`](TrackCreditsResponseBuilder::reservation_remaining)
    /// - [`totals`](TrackCreditsResponseBuilder::totals)
    pub fn build(self) -> Result<TrackCreditsResponse, BuildError> {
        Ok(TrackCreditsResponse {
            allowed: self
                .allowed
                .ok_or_else(|| BuildError::missing_field("allowed"))?,
            at_zero_action: self.at_zero_action,
            balances: self
                .balances
                .ok_or_else(|| BuildError::missing_field("balances"))?,
            charges: self
                .charges
                .ok_or_else(|| BuildError::missing_field("charges"))?,
            envelope_exceeded: self.envelope_exceeded,
            events_accepted: self.events_accepted,
            events_priced: self.events_priced,
            reservation_expired: self.reservation_expired,
            reservation_expires_at: self.reservation_expires_at,
            reservation_id: self
                .reservation_id
                .ok_or_else(|| BuildError::missing_field("reservation_id"))?,
            reservation_open: self
                .reservation_open
                .ok_or_else(|| BuildError::missing_field("reservation_open"))?,
            reservation_remaining: self
                .reservation_remaining
                .ok_or_else(|| BuildError::missing_field("reservation_remaining"))?,
            totals: self
                .totals
                .ok_or_else(|| BuildError::missing_field("totals"))?,
        })
    }
}
