pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct MatrixPriceCellRequest {
    /// Stable identity of an existing matrix cell; omitted for a new cell
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cell_key: Option<String>,
    /// Fixed pricing details
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fixed_pricing_model: Option<FixedPricingModel>,
    /// Graduated percentage pricing details
    #[serde(skip_serializing_if = "Option::is_none")]
    pub graduated_percentage_pricing_model: Option<GraduatedPercentagePricingModel>,
    /// Graduated tiered pricing details
    #[serde(skip_serializing_if = "Option::is_none")]
    pub graduated_tiered_pricing_model: Option<GraduatedTieredPricingModel>,
    /// Invoice line label for this cell, including an explicitly empty label
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    /// Tiered percentage pricing details
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tiered_percentage_pricing_model: Option<TieredPercentagePricingModel>,
    /// Tiered pricing details
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tiered_pricing_model: Option<TieredPricingModel>,
    /// Pricing model used by this cell
    pub r#type: MatrixPriceCellRequestType,
    /// Unit pricing details
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_pricing_model: Option<UnitPricingModel>,
    /// Volume percentage pricing details
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume_percentage_pricing_model: Option<VolumePercentagePricingModel>,
}

impl MatrixPriceCellRequest {
    pub fn builder() -> MatrixPriceCellRequestBuilder {
        <MatrixPriceCellRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct MatrixPriceCellRequestBuilder {
    cell_key: Option<String>,
    fixed_pricing_model: Option<FixedPricingModel>,
    graduated_percentage_pricing_model: Option<GraduatedPercentagePricingModel>,
    graduated_tiered_pricing_model: Option<GraduatedTieredPricingModel>,
    label: Option<String>,
    tiered_percentage_pricing_model: Option<TieredPercentagePricingModel>,
    tiered_pricing_model: Option<TieredPricingModel>,
    r#type: Option<MatrixPriceCellRequestType>,
    unit_pricing_model: Option<UnitPricingModel>,
    volume_percentage_pricing_model: Option<VolumePercentagePricingModel>,
}

impl MatrixPriceCellRequestBuilder {
    pub fn cell_key(mut self, value: impl Into<String>) -> Self {
        self.cell_key = Some(value.into());
        self
    }

    pub fn fixed_pricing_model(mut self, value: FixedPricingModel) -> Self {
        self.fixed_pricing_model = Some(value);
        self
    }

    pub fn graduated_percentage_pricing_model(
        mut self,
        value: GraduatedPercentagePricingModel,
    ) -> Self {
        self.graduated_percentage_pricing_model = Some(value);
        self
    }

    pub fn graduated_tiered_pricing_model(mut self, value: GraduatedTieredPricingModel) -> Self {
        self.graduated_tiered_pricing_model = Some(value);
        self
    }

    pub fn label(mut self, value: impl Into<String>) -> Self {
        self.label = Some(value.into());
        self
    }

    pub fn tiered_percentage_pricing_model(mut self, value: TieredPercentagePricingModel) -> Self {
        self.tiered_percentage_pricing_model = Some(value);
        self
    }

    pub fn tiered_pricing_model(mut self, value: TieredPricingModel) -> Self {
        self.tiered_pricing_model = Some(value);
        self
    }

    pub fn r#type(mut self, value: MatrixPriceCellRequestType) -> Self {
        self.r#type = Some(value);
        self
    }

    pub fn unit_pricing_model(mut self, value: UnitPricingModel) -> Self {
        self.unit_pricing_model = Some(value);
        self
    }

    pub fn volume_percentage_pricing_model(mut self, value: VolumePercentagePricingModel) -> Self {
        self.volume_percentage_pricing_model = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`MatrixPriceCellRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`r#type`](MatrixPriceCellRequestBuilder::r#type)
    pub fn build(self) -> Result<MatrixPriceCellRequest, BuildError> {
        Ok(MatrixPriceCellRequest {
            cell_key: self.cell_key,
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
