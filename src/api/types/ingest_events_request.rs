pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct IngestEventsRequest {
    /// Array of events to ingest. Maximum 100 events per request
    pub events: Vec<EventItem>,
}

impl IngestEventsRequest {
    pub fn builder() -> IngestEventsRequestBuilder {
        <IngestEventsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct IngestEventsRequestBuilder {
    events: Option<Vec<EventItem>>,
}

impl IngestEventsRequestBuilder {
    pub fn events(mut self, value: Vec<EventItem>) -> Self {
        self.events = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`IngestEventsRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`events`](IngestEventsRequestBuilder::events)
    pub fn build(self) -> Result<IngestEventsRequest, BuildError> {
        Ok(IngestEventsRequest {
            events: self
                .events
                .ok_or_else(|| BuildError::missing_field("events"))?,
        })
    }
}
