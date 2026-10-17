pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct MatrixExactMatchResponse {
    /// Route taken for this value
    pub child: Box<MatrixNodeResponse>,
    /// Exact event property value matched by this branch
    pub value: String,
}

impl MatrixExactMatchResponse {
    pub fn builder() -> MatrixExactMatchResponseBuilder {
        <MatrixExactMatchResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct MatrixExactMatchResponseBuilder {
    child: Option<Box<MatrixNodeResponse>>,
    value: Option<String>,
}

impl MatrixExactMatchResponseBuilder {
    pub fn child(mut self, value: Box<MatrixNodeResponse>) -> Self {
        self.child = Some(value);
        self
    }

    pub fn value(mut self, value: impl Into<String>) -> Self {
        self.value = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`MatrixExactMatchResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`child`](MatrixExactMatchResponseBuilder::child)
    /// - [`value`](MatrixExactMatchResponseBuilder::value)
    pub fn build(self) -> Result<MatrixExactMatchResponse, BuildError> {
        Ok(MatrixExactMatchResponse {
            child: self
                .child
                .ok_or_else(|| BuildError::missing_field("child"))?,
            value: self
                .value
                .ok_or_else(|| BuildError::missing_field("value"))?,
        })
    }
}
