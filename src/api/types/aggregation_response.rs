pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct AggregationResponse {
    /// Event property field to aggregate. Not required for count method
    pub field: String,
    /// Aggregation method
    pub method: AggregationMethod,
}

impl AggregationResponse {
    pub fn builder() -> AggregationResponseBuilder {
        <AggregationResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AggregationResponseBuilder {
    field: Option<String>,
    method: Option<AggregationMethod>,
}

impl AggregationResponseBuilder {
    pub fn field(mut self, value: impl Into<String>) -> Self {
        self.field = Some(value.into());
        self
    }

    pub fn method(mut self, value: AggregationMethod) -> Self {
        self.method = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AggregationResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`field`](AggregationResponseBuilder::field)
    /// - [`method`](AggregationResponseBuilder::method)
    pub fn build(self) -> Result<AggregationResponse, BuildError> {
        Ok(AggregationResponse {
            field: self
                .field
                .ok_or_else(|| BuildError::missing_field("field"))?,
            method: self
                .method
                .ok_or_else(|| BuildError::missing_field("method"))?,
        })
    }
}
