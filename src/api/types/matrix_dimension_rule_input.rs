pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct MatrixDimensionRuleInput {
    /// Exact property-value branches
    pub branches: Vec<Box<MatrixExactMatchInput>>,
    /// Dimension evaluated by this split
    pub dimension_key: String,
    /// Route for a present property value without an exact branch
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wildcard: Option<Box<MatrixNodeInput>>,
}

impl MatrixDimensionRuleInput {
    pub fn builder() -> MatrixDimensionRuleInputBuilder {
        <MatrixDimensionRuleInputBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct MatrixDimensionRuleInputBuilder {
    branches: Option<Vec<Box<MatrixExactMatchInput>>>,
    dimension_key: Option<String>,
    wildcard: Option<Box<MatrixNodeInput>>,
}

impl MatrixDimensionRuleInputBuilder {
    pub fn branches(mut self, value: Vec<Box<MatrixExactMatchInput>>) -> Self {
        self.branches = Some(value);
        self
    }

    pub fn dimension_key(mut self, value: impl Into<String>) -> Self {
        self.dimension_key = Some(value.into());
        self
    }

    pub fn wildcard(mut self, value: Box<MatrixNodeInput>) -> Self {
        self.wildcard = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`MatrixDimensionRuleInput`].
    /// This method will fail if any of the following fields are not set:
    /// - [`branches`](MatrixDimensionRuleInputBuilder::branches)
    /// - [`dimension_key`](MatrixDimensionRuleInputBuilder::dimension_key)
    pub fn build(self) -> Result<MatrixDimensionRuleInput, BuildError> {
        Ok(MatrixDimensionRuleInput {
            branches: self
                .branches
                .ok_or_else(|| BuildError::missing_field("branches"))?,
            dimension_key: self
                .dimension_key
                .ok_or_else(|| BuildError::missing_field("dimension_key"))?,
            wildcard: self.wildcard,
        })
    }
}
