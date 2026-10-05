pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct TrackCreditsRequest {
    /// Customer account ID or alias; direct-draw mode. Required unless reservation_id is set, where the account comes from the reservation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account: Option<String>,
    /// Whether this is the final call for the reservation. Required whenever reservation_id is set: there is no safe default, since true would end runs that meant to continue and false would leak reservations until they expire. The reservation also closes on its own if the events consume everything held. Not valid on a direct draw.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub close: Option<bool>,
    /// The usage being recorded, up to 100 events. The amount charged is computed from these against whichever billable metrics count them; there is no credit amount in the request. On a settlement, an empty array with close: true releases the whole reservation, charging nothing, which is the recovery when work is abandoned.
    pub events: Vec<TrackCreditEvent>,
    /// What to do when a settle cannot be fully covered by credit on a metric that stops at zero — either because it prices past the reservation's authorized hold, or because credit ran out before a late settle arrived. "reject" (the default) refuses the settle, burning and recording nothing. "record" burns what is available and reports the remainder in the charges' overflow, deliberately weakening a hard stop you configured. Reservation-settlement mode only. The refusal body's reason says which condition fired: adding credit resolves coverage_shortfall, but not envelope_exceeded, whose limit is the hold's size.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_envelope_exceeded: Option<TrackCreditsRequestOnEnvelopeExceeded>,
    /// Reservation token returned by /check, to close it; reservation-settlement mode. Mutually exclusive with account. The events sent alongside are both metered and charged, so do not send them to /events as well.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reservation_id: Option<String>,
}

impl TrackCreditsRequest {
    pub fn builder() -> TrackCreditsRequestBuilder {
        <TrackCreditsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TrackCreditsRequestBuilder {
    account: Option<String>,
    close: Option<bool>,
    events: Option<Vec<TrackCreditEvent>>,
    on_envelope_exceeded: Option<TrackCreditsRequestOnEnvelopeExceeded>,
    reservation_id: Option<String>,
}

impl TrackCreditsRequestBuilder {
    pub fn account(mut self, value: impl Into<String>) -> Self {
        self.account = Some(value.into());
        self
    }

    pub fn close(mut self, value: bool) -> Self {
        self.close = Some(value);
        self
    }

    pub fn events(mut self, value: Vec<TrackCreditEvent>) -> Self {
        self.events = Some(value);
        self
    }

    pub fn on_envelope_exceeded(mut self, value: TrackCreditsRequestOnEnvelopeExceeded) -> Self {
        self.on_envelope_exceeded = Some(value);
        self
    }

    pub fn reservation_id(mut self, value: impl Into<String>) -> Self {
        self.reservation_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`TrackCreditsRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`events`](TrackCreditsRequestBuilder::events)
    pub fn build(self) -> Result<TrackCreditsRequest, BuildError> {
        Ok(TrackCreditsRequest {
            account: self.account,
            close: self.close,
            events: self
                .events
                .ok_or_else(|| BuildError::missing_field("events"))?,
            on_envelope_exceeded: self.on_envelope_exceeded,
            reservation_id: self.reservation_id,
        })
    }
}
