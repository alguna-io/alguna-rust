pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct EventItem {
    /// Customer account ID or alias
    pub account: String,
    /// Name of the event, used for metric grouping
    pub event_name: String,
    /// Key-value pairs of event properties. Up to 50 properties allowed
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<HashMap<String, serde_json::Value>>,
    /// Event timestamp in RFC3339 format. Defaults to current time if not provided
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<DateTime<FixedOffset>>,
    /// Idempotency key for the event, max 72 characters. Should be a UUID or similar collision-resistant identifier; hash longer natural keys before sending
    pub unique_id: String,
}

impl EventItem {
    pub fn builder() -> EventItemBuilder {
        <EventItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct EventItemBuilder {
    account: Option<String>,
    event_name: Option<String>,
    properties: Option<HashMap<String, serde_json::Value>>,
    timestamp: Option<DateTime<FixedOffset>>,
    unique_id: Option<String>,
}

impl EventItemBuilder {
    pub fn account(mut self, value: impl Into<String>) -> Self {
        self.account = Some(value.into());
        self
    }

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

    /// Consumes the builder and constructs a [`EventItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`account`](EventItemBuilder::account)
    /// - [`event_name`](EventItemBuilder::event_name)
    /// - [`unique_id`](EventItemBuilder::unique_id)
    pub fn build(self) -> Result<EventItem, BuildError> {
        Ok(EventItem {
            account: self
                .account
                .ok_or_else(|| BuildError::missing_field("account"))?,
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
