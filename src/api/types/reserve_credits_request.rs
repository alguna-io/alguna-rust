pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ReserveCreditsRequest {
    /// Ceiling for an open-ended run, in METRIC UNITS (your events' own vocabulary), never credits. The hold is sized to max_quantity × rate for each credit-consuming metric event_name feeds, summed. Requires event_name; incompatible with events, which size the hold themselves. Omit it (send reserve: {}) to size the hold to exactly the events in this request.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_quantity: Option<String>,
}

impl ReserveCreditsRequest {
    pub fn builder() -> ReserveCreditsRequestBuilder {
        <ReserveCreditsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ReserveCreditsRequestBuilder {
    max_quantity: Option<String>,
}

impl ReserveCreditsRequestBuilder {
    pub fn max_quantity(mut self, value: impl Into<String>) -> Self {
        self.max_quantity = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ReserveCreditsRequest`].
    pub fn build(self) -> Result<ReserveCreditsRequest, BuildError> {
        Ok(ReserveCreditsRequest {
            max_quantity: self.max_quantity,
        })
    }
}
