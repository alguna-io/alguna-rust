pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct MatrixPricingModelRequest {
    /// Event property dimensions used by the matrix
    pub dimensions: Vec<MatrixDimensionDefinition>,
    /// Default invoice line description template for matrix cells
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line_item_template: Option<String>,
    /// Root of the matrix routing tree
    pub root: MatrixNode,
}

impl MatrixPricingModelRequest {
    pub fn builder() -> MatrixPricingModelRequestBuilder {
        <MatrixPricingModelRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct MatrixPricingModelRequestBuilder {
    dimensions: Option<Vec<MatrixDimensionDefinition>>,
    line_item_template: Option<String>,
    root: Option<MatrixNode>,
}

impl MatrixPricingModelRequestBuilder {
    pub fn dimensions(mut self, value: Vec<MatrixDimensionDefinition>) -> Self {
        self.dimensions = Some(value);
        self
    }

    pub fn line_item_template(mut self, value: impl Into<String>) -> Self {
        self.line_item_template = Some(value.into());
        self
    }

    pub fn root(mut self, value: MatrixNode) -> Self {
        self.root = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`MatrixPricingModelRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`dimensions`](MatrixPricingModelRequestBuilder::dimensions)
    /// - [`root`](MatrixPricingModelRequestBuilder::root)
    pub fn build(self) -> Result<MatrixPricingModelRequest, BuildError> {
        Ok(MatrixPricingModelRequest {
            dimensions: self
                .dimensions
                .ok_or_else(|| BuildError::missing_field("dimensions"))?,
            line_item_template: self.line_item_template,
            root: self.root.ok_or_else(|| BuildError::missing_field("root"))?,
        })
    }
}
