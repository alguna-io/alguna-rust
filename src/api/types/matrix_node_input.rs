pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct MatrixNodeInput {
    /// Route used when no split claims an event
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fallback: Option<Box<MatrixNodeInput>>,
    /// Price applied when this node is a cell
    #[serde(skip_serializing_if = "Option::is_none")]
    pub leaf: Option<MatrixPriceCellInput>,
    /// Ordered routing choices at this node
    #[serde(skip_serializing_if = "Option::is_none")]
    pub splits: Option<Vec<Box<MatrixDimensionRuleInput>>>,
}

impl MatrixNodeInput {
    pub fn builder() -> MatrixNodeInputBuilder {
        <MatrixNodeInputBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct MatrixNodeInputBuilder {
    fallback: Option<Box<MatrixNodeInput>>,
    leaf: Option<MatrixPriceCellInput>,
    splits: Option<Vec<Box<MatrixDimensionRuleInput>>>,
}

impl MatrixNodeInputBuilder {
    pub fn fallback(mut self, value: Box<MatrixNodeInput>) -> Self {
        self.fallback = Some(value);
        self
    }

    pub fn leaf(mut self, value: MatrixPriceCellInput) -> Self {
        self.leaf = Some(value);
        self
    }

    pub fn splits(mut self, value: Vec<Box<MatrixDimensionRuleInput>>) -> Self {
        self.splits = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`MatrixNodeInput`].
    pub fn build(self) -> Result<MatrixNodeInput, BuildError> {
        Ok(MatrixNodeInput {
            fallback: self.fallback,
            leaf: self.leaf,
            splits: self.splits,
        })
    }
}
