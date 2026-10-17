pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct IngestEventsResponse {
    /// List of unique_id values that failed to ingest
    pub failed: Vec<String>,
    /// List of unique_id values that were successfully ingested
    pub ingested: Vec<String>,
}

impl IngestEventsResponse {
    pub fn builder() -> IngestEventsResponseBuilder {
        <IngestEventsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct IngestEventsResponseBuilder {
    failed: Option<Vec<String>>,
    ingested: Option<Vec<String>>,
}

impl IngestEventsResponseBuilder {
    pub fn failed(mut self, value: Vec<String>) -> Self {
        self.failed = Some(value);
        self
    }

    pub fn ingested(mut self, value: Vec<String>) -> Self {
        self.ingested = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`IngestEventsResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`failed`](IngestEventsResponseBuilder::failed)
    /// - [`ingested`](IngestEventsResponseBuilder::ingested)
    pub fn build(self) -> Result<IngestEventsResponse, BuildError> {
        Ok(IngestEventsResponse {
            failed: self
                .failed
                .ok_or_else(|| BuildError::missing_field("failed"))?,
            ingested: self
                .ingested
                .ok_or_else(|| BuildError::missing_field("ingested"))?,
        })
    }
}
