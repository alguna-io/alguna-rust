pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct MatrixNode {
    /// Route used when no split claims an event
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fallback: Option<Box<MatrixNode>>,
    /// Price applied when this node is a cell
    #[serde(skip_serializing_if = "Option::is_none")]
    pub leaf: Option<MatrixPriceCell>,
    /// Ordered routing choices at this node
    #[serde(skip_serializing_if = "Option::is_none")]
    pub splits: Option<Vec<Box<MatrixDimensionRule>>>,
}

impl MatrixNode {
    pub fn builder() -> MatrixNodeBuilder {
        <MatrixNodeBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct MatrixNodeBuilder {
    fallback: Option<Box<MatrixNode>>,
    leaf: Option<MatrixPriceCell>,
    splits: Option<Vec<Box<MatrixDimensionRule>>>,
}

impl MatrixNodeBuilder {
    pub fn fallback(mut self, value: Box<MatrixNode>) -> Self {
        self.fallback = Some(value);
        self
    }

    pub fn leaf(mut self, value: MatrixPriceCell) -> Self {
        self.leaf = Some(value);
        self
    }

    pub fn splits(mut self, value: Vec<Box<MatrixDimensionRule>>) -> Self {
        self.splits = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`MatrixNode`].
    pub fn build(self) -> Result<MatrixNode, BuildError> {
        Ok(MatrixNode {
            fallback: self.fallback,
            leaf: self.leaf,
            splits: self.splits,
        })
    }
}
