pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct MatrixPriceCellResponse {
    /// Stable identity of this matrix cell
    pub cell_key: String,
    /// Fixed pricing details
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fixed_pricing_model: Option<FixedPricingModelResponse>,
    /// Graduated percentage pricing details
    #[serde(skip_serializing_if = "Option::is_none")]
    pub graduated_percentage_pricing_model: Option<MatrixGraduatedPercentagePricingModelResponse>,
    /// Graduated tiered pricing details
    #[serde(skip_serializing_if = "Option::is_none")]
    pub graduated_tiered_pricing_model: Option<MatrixGraduatedTieredPricingModelResponse>,
    /// Invoice line label for this cell, including an explicitly empty label
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    /// Tiered percentage pricing details
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tiered_percentage_pricing_model: Option<MatrixTieredPercentagePricingModelResponse>,
    /// Tiered pricing details
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tiered_pricing_model: Option<MatrixTieredPricingModelResponse>,
    /// Pricing model used by this cell
    pub r#type: MatrixPriceCellResponseType,
    /// Unit pricing details
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_pricing_model: Option<UnitPricingModelResponse>,
    /// Volume percentage pricing details
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume_percentage_pricing_model: Option<VolumePercentagePricingModelResponse>,
}

impl MatrixPriceCellResponse {
    pub fn builder() -> MatrixPriceCellResponseBuilder {
        <MatrixPriceCellResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct MatrixPriceCellResponseBuilder {
    cell_key: Option<String>,
    fixed_pricing_model: Option<FixedPricingModelResponse>,
    graduated_percentage_pricing_model: Option<MatrixGraduatedPercentagePricingModelResponse>,
    graduated_tiered_pricing_model: Option<MatrixGraduatedTieredPricingModelResponse>,
    label: Option<String>,
    tiered_percentage_pricing_model: Option<MatrixTieredPercentagePricingModelResponse>,
    tiered_pricing_model: Option<MatrixTieredPricingModelResponse>,
    r#type: Option<MatrixPriceCellResponseType>,
    unit_pricing_model: Option<UnitPricingModelResponse>,
    volume_percentage_pricing_model: Option<VolumePercentagePricingModelResponse>,
}

impl MatrixPriceCellResponseBuilder {
    pub fn cell_key(mut self, value: impl Into<String>) -> Self {
        self.cell_key = Some(value.into());
        self
    }

    pub fn fixed_pricing_model(mut self, value: FixedPricingModelResponse) -> Self {
        self.fixed_pricing_model = Some(value);
        self
    }

    pub fn graduated_percentage_pricing_model(
        mut self,
        value: MatrixGraduatedPercentagePricingModelResponse,
    ) -> Self {
        self.graduated_percentage_pricing_model = Some(value);
        self
    }

    pub fn graduated_tiered_pricing_model(
        mut self,
        value: MatrixGraduatedTieredPricingModelResponse,
    ) -> Self {
        self.graduated_tiered_pricing_model = Some(value);
        self
    }

    pub fn label(mut self, value: impl Into<String>) -> Self {
        self.label = Some(value.into());
        self
    }

    pub fn tiered_percentage_pricing_model(
        mut self,
        value: MatrixTieredPercentagePricingModelResponse,
    ) -> Self {
        self.tiered_percentage_pricing_model = Some(value);
        self
    }

    pub fn tiered_pricing_model(mut self, value: MatrixTieredPricingModelResponse) -> Self {
        self.tiered_pricing_model = Some(value);
        self
    }

    pub fn r#type(mut self, value: MatrixPriceCellResponseType) -> Self {
        self.r#type = Some(value);
        self
    }

    pub fn unit_pricing_model(mut self, value: UnitPricingModelResponse) -> Self {
        self.unit_pricing_model = Some(value);
        self
    }

    pub fn volume_percentage_pricing_model(
        mut self,
        value: VolumePercentagePricingModelResponse,
    ) -> Self {
        self.volume_percentage_pricing_model = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`MatrixPriceCellResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`cell_key`](MatrixPriceCellResponseBuilder::cell_key)
    /// - [`r#type`](MatrixPriceCellResponseBuilder::r#type)
    pub fn build(self) -> Result<MatrixPriceCellResponse, BuildError> {
        Ok(MatrixPriceCellResponse {
            cell_key: self
                .cell_key
                .ok_or_else(|| BuildError::missing_field("cell_key"))?,
            fixed_pricing_model: self.fixed_pricing_model,
            graduated_percentage_pricing_model: self.graduated_percentage_pricing_model,
            graduated_tiered_pricing_model: self.graduated_tiered_pricing_model,
            label: self.label,
            tiered_percentage_pricing_model: self.tiered_percentage_pricing_model,
            tiered_pricing_model: self.tiered_pricing_model,
            r#type: self
                .r#type
                .ok_or_else(|| BuildError::missing_field("r#type"))?,
            unit_pricing_model: self.unit_pricing_model,
            volume_percentage_pricing_model: self.volume_percentage_pricing_model,
        })
    }
}
