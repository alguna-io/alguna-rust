pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct MatrixExactMatchInput {
    /// Route taken for this value
    pub child: Box<MatrixNodeInput>,
    /// Exact event property value matched by this branch
    pub value: String,
}

impl MatrixExactMatchInput {
    pub fn builder() -> MatrixExactMatchInputBuilder {
        <MatrixExactMatchInputBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct MatrixExactMatchInputBuilder {
    child: Option<Box<MatrixNodeInput>>,
    value: Option<String>,
}

impl MatrixExactMatchInputBuilder {
    pub fn child(mut self, value: Box<MatrixNodeInput>) -> Self {
        self.child = Some(value);
        self
    }

    pub fn value(mut self, value: impl Into<String>) -> Self {
        self.value = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`MatrixExactMatchInput`].
    /// This method will fail if any of the following fields are not set:
    /// - [`child`](MatrixExactMatchInputBuilder::child)
    /// - [`value`](MatrixExactMatchInputBuilder::value)
    pub fn build(self) -> Result<MatrixExactMatchInput, BuildError> {
        Ok(MatrixExactMatchInput {
            child: self
                .child
                .ok_or_else(|| BuildError::missing_field("child"))?,
            value: self
                .value
                .ok_or_else(|| BuildError::missing_field("value"))?,
        })
    }
}
