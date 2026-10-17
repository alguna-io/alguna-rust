pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct MatrixDimensionDefinitionResponse {
    /// Event property key used by this dimension
    pub key: String,
    /// Display label for this dimension
    pub label: String,
    /// Authored values offered for this dimension
    #[serde(skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<String>>,
}

impl MatrixDimensionDefinitionResponse {
    pub fn builder() -> MatrixDimensionDefinitionResponseBuilder {
        <MatrixDimensionDefinitionResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct MatrixDimensionDefinitionResponseBuilder {
    key: Option<String>,
    label: Option<String>,
    values: Option<Vec<String>>,
}

impl MatrixDimensionDefinitionResponseBuilder {
    pub fn key(mut self, value: impl Into<String>) -> Self {
        self.key = Some(value.into());
        self
    }

    pub fn label(mut self, value: impl Into<String>) -> Self {
        self.label = Some(value.into());
        self
    }

    pub fn values(mut self, value: Vec<String>) -> Self {
        self.values = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`MatrixDimensionDefinitionResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`key`](MatrixDimensionDefinitionResponseBuilder::key)
    /// - [`label`](MatrixDimensionDefinitionResponseBuilder::label)
    pub fn build(self) -> Result<MatrixDimensionDefinitionResponse, BuildError> {
        Ok(MatrixDimensionDefinitionResponse {
            key: self.key.ok_or_else(|| BuildError::missing_field("key"))?,
            label: self
                .label
                .ok_or_else(|| BuildError::missing_field("label"))?,
            values: self.values,
        })
    }
}
