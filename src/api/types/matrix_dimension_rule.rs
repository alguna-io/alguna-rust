pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct MatrixDimensionRule {
    /// Exact property-value branches
    pub branches: Vec<Box<MatrixExactMatch>>,
    /// Dimension evaluated by this split
    pub dimension_key: String,
    /// Route for a present property value without an exact branch
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wildcard: Option<Box<MatrixNode>>,
}

impl MatrixDimensionRule {
    pub fn builder() -> MatrixDimensionRuleBuilder {
        <MatrixDimensionRuleBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct MatrixDimensionRuleBuilder {
    branches: Option<Vec<Box<MatrixExactMatch>>>,
    dimension_key: Option<String>,
    wildcard: Option<Box<MatrixNode>>,
}

impl MatrixDimensionRuleBuilder {
    pub fn branches(mut self, value: Vec<Box<MatrixExactMatch>>) -> Self {
        self.branches = Some(value);
        self
    }

    pub fn dimension_key(mut self, value: impl Into<String>) -> Self {
        self.dimension_key = Some(value.into());
        self
    }

    pub fn wildcard(mut self, value: Box<MatrixNode>) -> Self {
        self.wildcard = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`MatrixDimensionRule`].
    /// This method will fail if any of the following fields are not set:
    /// - [`branches`](MatrixDimensionRuleBuilder::branches)
    /// - [`dimension_key`](MatrixDimensionRuleBuilder::dimension_key)
    pub fn build(self) -> Result<MatrixDimensionRule, BuildError> {
        Ok(MatrixDimensionRule {
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
