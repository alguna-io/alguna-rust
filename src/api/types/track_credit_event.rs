pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct TrackCreditEvent {
    /// Usage event name. On a settlement it must be counted by one of the metrics the reservation covers, or the request is rejected.
    pub event_name: String,
    /// Event properties. For a metric that sums a property, the aggregation field is required here and is the billing input for this event.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<HashMap<String, serde_json::Value>>,
    /// When the usage happened (RFC3339). Defaults to receipt time.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<DateTime<FixedOffset>>,
    /// Stable dedupe identity for this event, max 72 characters. Must be unique within the payload; reuse the same identity on every retry.
    pub unique_id: String,
}

impl TrackCreditEvent {
    pub fn builder() -> TrackCreditEventBuilder {
        <TrackCreditEventBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TrackCreditEventBuilder {
    event_name: Option<String>,
    properties: Option<HashMap<String, serde_json::Value>>,
    timestamp: Option<DateTime<FixedOffset>>,
    unique_id: Option<String>,
}

impl TrackCreditEventBuilder {
    pub fn event_name(mut self, value: impl Into<String>) -> Self {
        self.event_name = Some(value.into());
        self
    }

    pub fn properties(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.properties = Some(value);
        self
    }

    pub fn timestamp(mut self, value: DateTime<FixedOffset>) -> Self {
        self.timestamp = Some(value);
        self
    }

    pub fn unique_id(mut self, value: impl Into<String>) -> Self {
        self.unique_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`TrackCreditEvent`].
    /// This method will fail if any of the following fields are not set:
    /// - [`event_name`](TrackCreditEventBuilder::event_name)
    /// - [`unique_id`](TrackCreditEventBuilder::unique_id)
    pub fn build(self) -> Result<TrackCreditEvent, BuildError> {
        Ok(TrackCreditEvent {
            event_name: self
                .event_name
                .ok_or_else(|| BuildError::missing_field("event_name"))?,
            properties: self.properties,
            timestamp: self.timestamp,
            unique_id: self
                .unique_id
                .ok_or_else(|| BuildError::missing_field("unique_id"))?,
        })
    }
}
