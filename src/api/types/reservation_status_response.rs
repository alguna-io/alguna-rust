pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct ReservationStatusResponse {
    /// Credits this reservation fenced when it opened.
    pub block_amount: String,
    /// Every close already committed, oldest first. This is how a client that lost its Idempotency-Key discovers what already landed instead of resending and burning twice.
    pub closes: Vec<ReservationCloseSummary>,
    /// What every close so far priced to: the sum of closes[].used, not of closes[].covered. Under on_envelope_exceeded: record a close can price past the hold, so this can exceed block_amount — and because remaining floors at zero, block_amount − consumed is not a way to derive it. Sum closes[].covered instead for what the ledger actually burned.
    pub consumed: String,
    /// When the hold expires. Absent once terminal.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<DateTime<FixedOffset>>,
    /// The metrics this hold covers — one entry per credit-consuming metric its settles price against. A settle's events must carry event names these identities count.
    pub identities: Vec<ReservationIdentity>,
    /// Whether the reservation can still be settled against.
    pub open: bool,
    /// Credits still authorised: block_amount − consumed, floored at zero. Zero once the reservation is terminal, whatever the arithmetic would otherwise allow.
    pub remaining: String,
    pub reservation_id: String,
    /// open, capturing, captured, released, expired, or recovery_pending.
    pub status: ReservationStatusResponseStatus,
}

impl ReservationStatusResponse {
    pub fn builder() -> ReservationStatusResponseBuilder {
        <ReservationStatusResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ReservationStatusResponseBuilder {
    block_amount: Option<String>,
    closes: Option<Vec<ReservationCloseSummary>>,
    consumed: Option<String>,
    expires_at: Option<DateTime<FixedOffset>>,
    identities: Option<Vec<ReservationIdentity>>,
    open: Option<bool>,
    remaining: Option<String>,
    reservation_id: Option<String>,
    status: Option<ReservationStatusResponseStatus>,
}

impl ReservationStatusResponseBuilder {
    pub fn block_amount(mut self, value: impl Into<String>) -> Self {
        self.block_amount = Some(value.into());
        self
    }

    pub fn closes(mut self, value: Vec<ReservationCloseSummary>) -> Self {
        self.closes = Some(value);
        self
    }

    pub fn consumed(mut self, value: impl Into<String>) -> Self {
        self.consumed = Some(value.into());
        self
    }

    pub fn expires_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.expires_at = Some(value);
        self
    }

    pub fn identities(mut self, value: Vec<ReservationIdentity>) -> Self {
        self.identities = Some(value);
        self
    }

    pub fn open(mut self, value: bool) -> Self {
        self.open = Some(value);
        self
    }

    pub fn remaining(mut self, value: impl Into<String>) -> Self {
        self.remaining = Some(value.into());
        self
    }

    pub fn reservation_id(mut self, value: impl Into<String>) -> Self {
        self.reservation_id = Some(value.into());
        self
    }

    pub fn status(mut self, value: ReservationStatusResponseStatus) -> Self {
        self.status = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ReservationStatusResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`block_amount`](ReservationStatusResponseBuilder::block_amount)
    /// - [`closes`](ReservationStatusResponseBuilder::closes)
    /// - [`consumed`](ReservationStatusResponseBuilder::consumed)
    /// - [`identities`](ReservationStatusResponseBuilder::identities)
    /// - [`open`](ReservationStatusResponseBuilder::open)
    /// - [`remaining`](ReservationStatusResponseBuilder::remaining)
    /// - [`reservation_id`](ReservationStatusResponseBuilder::reservation_id)
    /// - [`status`](ReservationStatusResponseBuilder::status)
    pub fn build(self) -> Result<ReservationStatusResponse, BuildError> {
        Ok(ReservationStatusResponse {
            block_amount: self
                .block_amount
                .ok_or_else(|| BuildError::missing_field("block_amount"))?,
            closes: self
                .closes
                .ok_or_else(|| BuildError::missing_field("closes"))?,
            consumed: self
                .consumed
                .ok_or_else(|| BuildError::missing_field("consumed"))?,
            expires_at: self.expires_at,
            identities: self
                .identities
                .ok_or_else(|| BuildError::missing_field("identities"))?,
            open: self.open.ok_or_else(|| BuildError::missing_field("open"))?,
            remaining: self
                .remaining
                .ok_or_else(|| BuildError::missing_field("remaining"))?,
            reservation_id: self
                .reservation_id
                .ok_or_else(|| BuildError::missing_field("reservation_id"))?,
            status: self
                .status
                .ok_or_else(|| BuildError::missing_field("status"))?,
        })
    }
}
