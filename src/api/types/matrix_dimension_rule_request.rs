pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct MatrixDimensionRuleRequest {
    /// Exact property-value branches
    pub branches: Vec<MatrixExactMatch>,
    /// Dimension evaluated by this split
    pub dimension_key: String,
    /// Route for a present property value without an exact branch
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wildcard: Option<MatrixNode>,
}

impl MatrixDimensionRuleRequest {
    pub fn builder() -> MatrixDimensionRuleRequestBuilder {
        <MatrixDimensionRuleRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct MatrixDimensionRuleRequestBuilder {
    branches: Option<Vec<MatrixExactMatch>>,
    dimension_key: Option<String>,
    wildcard: Option<MatrixNode>,
}

impl MatrixDimensionRuleRequestBuilder {
    pub fn branches(mut self, value: Vec<MatrixExactMatch>) -> Self {
        self.branches = Some(value);
        self
    }

    pub fn dimension_key(mut self, value: impl Into<String>) -> Self {
        self.dimension_key = Some(value.into());
        self
    }

    pub fn wildcard(mut self, value: MatrixNode) -> Self {
        self.wildcard = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`MatrixDimensionRuleRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`branches`](MatrixDimensionRuleRequestBuilder::branches)
    /// - [`dimension_key`](MatrixDimensionRuleRequestBuilder::dimension_key)
    pub fn build(self) -> Result<MatrixDimensionRuleRequest, BuildError> {
        Ok(MatrixDimensionRuleRequest {
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
