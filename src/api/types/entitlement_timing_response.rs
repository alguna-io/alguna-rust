pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct EntitlementTimingResponse {
    /// Timing interval
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interval: Option<String>,
    /// Recurrence type
    pub recurrence: EntitlementTimingResponseRecurrence,
}

impl EntitlementTimingResponse {
    pub fn builder() -> EntitlementTimingResponseBuilder {
        <EntitlementTimingResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct EntitlementTimingResponseBuilder {
    interval: Option<String>,
    recurrence: Option<EntitlementTimingResponseRecurrence>,
}

impl EntitlementTimingResponseBuilder {
    pub fn interval(mut self, value: impl Into<String>) -> Self {
        self.interval = Some(value.into());
        self
    }

    pub fn recurrence(mut self, value: EntitlementTimingResponseRecurrence) -> Self {
        self.recurrence = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`EntitlementTimingResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`recurrence`](EntitlementTimingResponseBuilder::recurrence)
    pub fn build(self) -> Result<EntitlementTimingResponse, BuildError> {
        Ok(EntitlementTimingResponse {
            interval: self.interval,
            recurrence: self
                .recurrence
                .ok_or_else(|| BuildError::missing_field("recurrence"))?,
        })
    }
}
