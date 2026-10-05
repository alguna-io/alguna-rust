pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct MatrixNodeResponse {
    /// Route used when no split claims an event
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fallback: Option<Box<MatrixNodeResponse>>,
    /// Price applied when this node is a cell
    #[serde(skip_serializing_if = "Option::is_none")]
    pub leaf: Option<MatrixPriceCellResponse>,
    /// Ordered routing choices at this node
    #[serde(skip_serializing_if = "Option::is_none")]
    pub splits: Option<Vec<Box<MatrixDimensionRuleResponse>>>,
}

impl MatrixNodeResponse {
    pub fn builder() -> MatrixNodeResponseBuilder {
        <MatrixNodeResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct MatrixNodeResponseBuilder {
    fallback: Option<Box<MatrixNodeResponse>>,
    leaf: Option<MatrixPriceCellResponse>,
    splits: Option<Vec<Box<MatrixDimensionRuleResponse>>>,
}

impl MatrixNodeResponseBuilder {
    pub fn fallback(mut self, value: Box<MatrixNodeResponse>) -> Self {
        self.fallback = Some(value);
        self
    }

    pub fn leaf(mut self, value: MatrixPriceCellResponse) -> Self {
        self.leaf = Some(value);
        self
    }

    pub fn splits(mut self, value: Vec<Box<MatrixDimensionRuleResponse>>) -> Self {
        self.splits = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`MatrixNodeResponse`].
    pub fn build(self) -> Result<MatrixNodeResponse, BuildError> {
        Ok(MatrixNodeResponse {
            fallback: self.fallback,
            leaf: self.leaf,
            splits: self.splits,
        })
    }
}
