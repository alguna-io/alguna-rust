pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct MatrixPricingModelResponse {
    /// Event property dimensions used by the matrix
    pub dimensions: Vec<MatrixDimensionDefinitionResponse>,
    /// Default invoice line description template for matrix cells
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line_item_template: Option<String>,
    /// Root of the matrix routing tree
    pub root: MatrixNodeResponse,
}

impl MatrixPricingModelResponse {
    pub fn builder() -> MatrixPricingModelResponseBuilder {
        <MatrixPricingModelResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct MatrixPricingModelResponseBuilder {
    dimensions: Option<Vec<MatrixDimensionDefinitionResponse>>,
    line_item_template: Option<String>,
    root: Option<MatrixNodeResponse>,
}

impl MatrixPricingModelResponseBuilder {
    pub fn dimensions(mut self, value: Vec<MatrixDimensionDefinitionResponse>) -> Self {
        self.dimensions = Some(value);
        self
    }

    pub fn line_item_template(mut self, value: impl Into<String>) -> Self {
        self.line_item_template = Some(value.into());
        self
    }

    pub fn root(mut self, value: MatrixNodeResponse) -> Self {
        self.root = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`MatrixPricingModelResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`dimensions`](MatrixPricingModelResponseBuilder::dimensions)
    /// - [`root`](MatrixPricingModelResponseBuilder::root)
    pub fn build(self) -> Result<MatrixPricingModelResponse, BuildError> {
        Ok(MatrixPricingModelResponse {
            dimensions: self
                .dimensions
                .ok_or_else(|| BuildError::missing_field("dimensions"))?,
            line_item_template: self.line_item_template,
            root: self.root.ok_or_else(|| BuildError::missing_field("root"))?,
        })
    }
}
