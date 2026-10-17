pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ReservationIdentity {
    /// The usage event that metric counts.
    pub event_name: String,
    /// A billable metric this hold's settles price against.
    pub metric_id: String,
}

impl ReservationIdentity {
    pub fn builder() -> ReservationIdentityBuilder {
        <ReservationIdentityBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ReservationIdentityBuilder {
    event_name: Option<String>,
    metric_id: Option<String>,
}

impl ReservationIdentityBuilder {
    pub fn event_name(mut self, value: impl Into<String>) -> Self {
        self.event_name = Some(value.into());
        self
    }

    pub fn metric_id(mut self, value: impl Into<String>) -> Self {
        self.metric_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ReservationIdentity`].
    /// This method will fail if any of the following fields are not set:
    /// - [`event_name`](ReservationIdentityBuilder::event_name)
    /// - [`metric_id`](ReservationIdentityBuilder::metric_id)
    pub fn build(self) -> Result<ReservationIdentity, BuildError> {
        Ok(ReservationIdentity {
            event_name: self
                .event_name
                .ok_or_else(|| BuildError::missing_field("event_name"))?,
            metric_id: self
                .metric_id
                .ok_or_else(|| BuildError::missing_field("metric_id"))?,
        })
    }
}
