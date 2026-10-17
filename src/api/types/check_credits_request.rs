pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct CheckCreditsRequest {
    /// Customer account ID or alias the credits belong to
    pub account: String,
    /// The usage event the work will produce. This is what resolves which billable metrics the check is against — you name what you are about to do, not which metric counts it. An event that feeds no credit-consuming metric is rejected with 422 — that usage belongs on POST /events; an event feeding several credit-consuming metrics is checked (and held) against all of them at once. Optional when events carries the sample; required for reserve.max_quantity.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_name: Option<String>,
    /// A sample of the work about to happen. The response's charges carry what that work would cost per metric, priced exactly as /track would price it. Nothing is recorded and no credit moves — this is how you learn a price without performing the work. With reserve: {}, the hold is sized to exactly these events.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub events: Option<Vec<TrackCreditEvent>>,
    /// Event properties used to resolve customer-specific filter groups for this check. Not a billing input: this call records nothing.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<HashMap<String, serde_json::Value>>,
    /// Open a durable hold alongside the check. An object, never a number: send {} to hold enough for exactly the events in this request, or {"max_quantity": "…"} with event_name for an open-ended ceiling in metric units. The old numeric credit-amount form is rejected — the request no longer carries credit amounts anywhere. Omit for a cost-free preflight or quote (no hold is opened). Requires an Idempotency-Key.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reserve: Option<ReserveCreditsRequest>,
    /// How long the hold stays open, in seconds. Defaults to 900 (15 minutes) and cannot exceed 14400 (4 hours), the maximum lifetime any hold may reach. Use it for a run that cannot report progress as it goes; a run that can should send partial settlements to /track instead, since each one slides this same TTL forward. Requires reserve. Not part of idempotency identity: a retry under the same Idempotency-Key replays the original hold with its original expires_at, whatever ttl_seconds it now carries.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ttl_seconds: Option<i64>,
}

impl CheckCreditsRequest {
    pub fn builder() -> CheckCreditsRequestBuilder {
        <CheckCreditsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CheckCreditsRequestBuilder {
    account: Option<String>,
    event_name: Option<String>,
    events: Option<Vec<TrackCreditEvent>>,
    properties: Option<HashMap<String, serde_json::Value>>,
    reserve: Option<ReserveCreditsRequest>,
    ttl_seconds: Option<i64>,
}

impl CheckCreditsRequestBuilder {
    pub fn account(mut self, value: impl Into<String>) -> Self {
        self.account = Some(value.into());
        self
    }

    pub fn event_name(mut self, value: impl Into<String>) -> Self {
        self.event_name = Some(value.into());
        self
    }

    pub fn events(mut self, value: Vec<TrackCreditEvent>) -> Self {
        self.events = Some(value);
        self
    }

    pub fn properties(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.properties = Some(value);
        self
    }

    pub fn reserve(mut self, value: ReserveCreditsRequest) -> Self {
        self.reserve = Some(value);
        self
    }

    pub fn ttl_seconds(mut self, value: i64) -> Self {
        self.ttl_seconds = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CheckCreditsRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`account`](CheckCreditsRequestBuilder::account)
    pub fn build(self) -> Result<CheckCreditsRequest, BuildError> {
        Ok(CheckCreditsRequest {
            account: self
                .account
                .ok_or_else(|| BuildError::missing_field("account"))?,
            event_name: self.event_name,
            events: self.events,
            properties: self.properties,
            reserve: self.reserve,
            ttl_seconds: self.ttl_seconds,
        })
    }
}
