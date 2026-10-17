pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct MatrixNodeRequest {
    /// Route used when no split claims an event
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fallback: Option<MatrixNode>,
    /// Price applied when this node is a cell
    #[serde(skip_serializing_if = "Option::is_none")]
    pub leaf: Option<MatrixPriceCell>,
    /// Ordered routing choices at this node
    #[serde(skip_serializing_if = "Option::is_none")]
    pub splits: Option<Vec<MatrixDimensionRule>>,
}

impl MatrixNodeRequest {
    pub fn builder() -> MatrixNodeRequestBuilder {
        <MatrixNodeRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct MatrixNodeRequestBuilder {
    fallback: Option<MatrixNode>,
    leaf: Option<MatrixPriceCell>,
    splits: Option<Vec<MatrixDimensionRule>>,
}

impl MatrixNodeRequestBuilder {
    pub fn fallback(mut self, value: MatrixNode) -> Self {
        self.fallback = Some(value);
        self
    }

    pub fn leaf(mut self, value: MatrixPriceCell) -> Self {
        self.leaf = Some(value);
        self
    }

    pub fn splits(mut self, value: Vec<MatrixDimensionRule>) -> Self {
        self.splits = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`MatrixNodeRequest`].
    pub fn build(self) -> Result<MatrixNodeRequest, BuildError> {
        Ok(MatrixNodeRequest {
            fallback: self.fallback,
            leaf: self.leaf,
            splits: self.splits,
        })
    }
}
