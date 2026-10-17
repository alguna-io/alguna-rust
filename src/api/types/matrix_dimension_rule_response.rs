pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct MatrixDimensionRuleResponse {
    /// Exact property-value branches
    pub branches: Vec<Box<MatrixExactMatchResponse>>,
    /// Dimension evaluated by this split
    pub dimension_key: String,
    /// Route for a present property value without an exact branch
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wildcard: Option<Box<MatrixNodeResponse>>,
}

impl MatrixDimensionRuleResponse {
    pub fn builder() -> MatrixDimensionRuleResponseBuilder {
        <MatrixDimensionRuleResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct MatrixDimensionRuleResponseBuilder {
    branches: Option<Vec<Box<MatrixExactMatchResponse>>>,
    dimension_key: Option<String>,
    wildcard: Option<Box<MatrixNodeResponse>>,
}

impl MatrixDimensionRuleResponseBuilder {
    pub fn branches(mut self, value: Vec<Box<MatrixExactMatchResponse>>) -> Self {
        self.branches = Some(value);
        self
    }

    pub fn dimension_key(mut self, value: impl Into<String>) -> Self {
        self.dimension_key = Some(value.into());
        self
    }

    pub fn wildcard(mut self, value: Box<MatrixNodeResponse>) -> Self {
        self.wildcard = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`MatrixDimensionRuleResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`branches`](MatrixDimensionRuleResponseBuilder::branches)
    /// - [`dimension_key`](MatrixDimensionRuleResponseBuilder::dimension_key)
    pub fn build(self) -> Result<MatrixDimensionRuleResponse, BuildError> {
        Ok(MatrixDimensionRuleResponse {
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
