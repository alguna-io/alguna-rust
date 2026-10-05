pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct MatrixExactMatchRequest {
    /// Route taken for this value
    pub child: MatrixNode,
    /// Exact event property value matched by this branch
    pub value: String,
}

impl MatrixExactMatchRequest {
    pub fn builder() -> MatrixExactMatchRequestBuilder {
        <MatrixExactMatchRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct MatrixExactMatchRequestBuilder {
    child: Option<MatrixNode>,
    value: Option<String>,
}

impl MatrixExactMatchRequestBuilder {
    pub fn child(mut self, value: MatrixNode) -> Self {
        self.child = Some(value);
        self
    }

    pub fn value(mut self, value: impl Into<String>) -> Self {
        self.value = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`MatrixExactMatchRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`child`](MatrixExactMatchRequestBuilder::child)
    /// - [`value`](MatrixExactMatchRequestBuilder::value)
    pub fn build(self) -> Result<MatrixExactMatchRequest, BuildError> {
        Ok(MatrixExactMatchRequest {
            child: self
                .child
                .ok_or_else(|| BuildError::missing_field("child"))?,
            value: self
                .value
                .ok_or_else(|| BuildError::missing_field("value"))?,
        })
    }
}
