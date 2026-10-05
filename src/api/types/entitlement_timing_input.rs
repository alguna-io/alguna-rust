pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct EntitlementTimingInput {
    /// Timing interval
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interval: Option<BillingInterval>,
    /// Recurrence type
    pub recurrence: RecurrenceType,
}

impl EntitlementTimingInput {
    pub fn builder() -> EntitlementTimingInputBuilder {
        <EntitlementTimingInputBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct EntitlementTimingInputBuilder {
    interval: Option<BillingInterval>,
    recurrence: Option<RecurrenceType>,
}

impl EntitlementTimingInputBuilder {
    pub fn interval(mut self, value: BillingInterval) -> Self {
        self.interval = Some(value);
        self
    }

    pub fn recurrence(mut self, value: RecurrenceType) -> Self {
        self.recurrence = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`EntitlementTimingInput`].
    /// This method will fail if any of the following fields are not set:
    /// - [`recurrence`](EntitlementTimingInputBuilder::recurrence)
    pub fn build(self) -> Result<EntitlementTimingInput, BuildError> {
        Ok(EntitlementTimingInput {
            interval: self.interval,
            recurrence: self
                .recurrence
                .ok_or_else(|| BuildError::missing_field("recurrence"))?,
        })
    }
}
