pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct MatrixExactMatch {
    /// Route taken for this value
    pub child: Box<MatrixNode>,
    /// Exact event property value matched by this branch
    pub value: String,
}

impl MatrixExactMatch {
    pub fn builder() -> MatrixExactMatchBuilder {
        <MatrixExactMatchBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct MatrixExactMatchBuilder {
    child: Option<Box<MatrixNode>>,
    value: Option<String>,
}

impl MatrixExactMatchBuilder {
    pub fn child(mut self, value: Box<MatrixNode>) -> Self {
        self.child = Some(value);
        self
    }

    pub fn value(mut self, value: impl Into<String>) -> Self {
        self.value = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`MatrixExactMatch`].
    /// This method will fail if any of the following fields are not set:
    /// - [`child`](MatrixExactMatchBuilder::child)
    /// - [`value`](MatrixExactMatchBuilder::value)
    pub fn build(self) -> Result<MatrixExactMatch, BuildError> {
        Ok(MatrixExactMatch {
            child: self
                .child
                .ok_or_else(|| BuildError::missing_field("child"))?,
            value: self
                .value
                .ok_or_else(|| BuildError::missing_field("value"))?,
        })
    }
}
