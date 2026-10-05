pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct MatrixPricingModelInput {
    /// Event property dimensions used by the matrix
    pub dimensions: Vec<MatrixDimensionDefinitionInput>,
    /// Default invoice line description template for matrix cells
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line_item_template: Option<String>,
    /// Root of the matrix routing tree
    pub root: MatrixNodeInput,
}

impl MatrixPricingModelInput {
    pub fn builder() -> MatrixPricingModelInputBuilder {
        <MatrixPricingModelInputBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct MatrixPricingModelInputBuilder {
    dimensions: Option<Vec<MatrixDimensionDefinitionInput>>,
    line_item_template: Option<String>,
    root: Option<MatrixNodeInput>,
}

impl MatrixPricingModelInputBuilder {
    pub fn dimensions(mut self, value: Vec<MatrixDimensionDefinitionInput>) -> Self {
        self.dimensions = Some(value);
        self
    }

    pub fn line_item_template(mut self, value: impl Into<String>) -> Self {
        self.line_item_template = Some(value.into());
        self
    }

    pub fn root(mut self, value: MatrixNodeInput) -> Self {
        self.root = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`MatrixPricingModelInput`].
    /// This method will fail if any of the following fields are not set:
    /// - [`dimensions`](MatrixPricingModelInputBuilder::dimensions)
    /// - [`root`](MatrixPricingModelInputBuilder::root)
    pub fn build(self) -> Result<MatrixPricingModelInput, BuildError> {
        Ok(MatrixPricingModelInput {
            dimensions: self
                .dimensions
                .ok_or_else(|| BuildError::missing_field("dimensions"))?,
            line_item_template: self.line_item_template,
            root: self.root.ok_or_else(|| BuildError::missing_field("root"))?,
        })
    }
}
