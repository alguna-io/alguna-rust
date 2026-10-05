pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct AggregationRequest {
    /// Event property field to aggregate. Required for all methods except count
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field: Option<String>,
    /// Aggregation method
    pub method: AggregationMethod,
}

impl AggregationRequest {
    pub fn builder() -> AggregationRequestBuilder {
        <AggregationRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AggregationRequestBuilder {
    field: Option<String>,
    method: Option<AggregationMethod>,
}

impl AggregationRequestBuilder {
    pub fn field(mut self, value: impl Into<String>) -> Self {
        self.field = Some(value.into());
        self
    }

    pub fn method(mut self, value: AggregationMethod) -> Self {
        self.method = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AggregationRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`method`](AggregationRequestBuilder::method)
    pub fn build(self) -> Result<AggregationRequest, BuildError> {
        Ok(AggregationRequest {
            field: self.field,
            method: self
                .method
                .ok_or_else(|| BuildError::missing_field("method"))?,
        })
    }
}
