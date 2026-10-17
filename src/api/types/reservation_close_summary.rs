pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ReservationCloseSummary {
    /// This close's position in the reservation's capture sequence, oldest first.
    pub capture_seq: i64,
    /// This close's identity: the Idempotency-Key it was sent under (namespaced ik:), or a hash of its events' unique_ids for a keyless close (namespaced uids:).
    pub close_key: String,
    /// Whether this close terminated the reservation.
    pub closed: bool,
    /// Credits this close actually burned. Below used whenever the close priced past what the hold still had; the difference billed as cash.
    pub covered: String,
    /// Credits this close was priced at. This — not covered — is what accumulates into the reservation's consumed.
    pub used: String,
}

impl ReservationCloseSummary {
    pub fn builder() -> ReservationCloseSummaryBuilder {
        <ReservationCloseSummaryBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ReservationCloseSummaryBuilder {
    capture_seq: Option<i64>,
    close_key: Option<String>,
    closed: Option<bool>,
    covered: Option<String>,
    used: Option<String>,
}

impl ReservationCloseSummaryBuilder {
    pub fn capture_seq(mut self, value: i64) -> Self {
        self.capture_seq = Some(value);
        self
    }

    pub fn close_key(mut self, value: impl Into<String>) -> Self {
        self.close_key = Some(value.into());
        self
    }

    pub fn closed(mut self, value: bool) -> Self {
        self.closed = Some(value);
        self
    }

    pub fn covered(mut self, value: impl Into<String>) -> Self {
        self.covered = Some(value.into());
        self
    }

    pub fn used(mut self, value: impl Into<String>) -> Self {
        self.used = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ReservationCloseSummary`].
    /// This method will fail if any of the following fields are not set:
    /// - [`capture_seq`](ReservationCloseSummaryBuilder::capture_seq)
    /// - [`close_key`](ReservationCloseSummaryBuilder::close_key)
    /// - [`closed`](ReservationCloseSummaryBuilder::closed)
    /// - [`covered`](ReservationCloseSummaryBuilder::covered)
    /// - [`used`](ReservationCloseSummaryBuilder::used)
    pub fn build(self) -> Result<ReservationCloseSummary, BuildError> {
        Ok(ReservationCloseSummary {
            capture_seq: self
                .capture_seq
                .ok_or_else(|| BuildError::missing_field("capture_seq"))?,
            close_key: self
                .close_key
                .ok_or_else(|| BuildError::missing_field("close_key"))?,
            closed: self
                .closed
                .ok_or_else(|| BuildError::missing_field("closed"))?,
            covered: self
                .covered
                .ok_or_else(|| BuildError::missing_field("covered"))?,
            used: self.used.ok_or_else(|| BuildError::missing_field("used"))?,
        })
    }
}
