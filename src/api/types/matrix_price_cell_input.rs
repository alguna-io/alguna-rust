pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct MatrixPriceCellInput {
    /// Stable identity of an existing matrix cell; omitted for a new cell
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cell_key: Option<String>,
    /// Fixed pricing details
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fixed_pricing_model: Option<FixedPricingModelInput>,
    /// Graduated percentage pricing details
    #[serde(skip_serializing_if = "Option::is_none")]
    pub graduated_percentage_pricing_model: Option<GraduatedPercentagePricingModelInput>,
    /// Graduated tiered pricing details
    #[serde(skip_serializing_if = "Option::is_none")]
    pub graduated_tiered_pricing_model: Option<GraduatedTieredPricingModelInput>,
    /// Invoice line label for this cell, including an explicitly empty label
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    /// Tiered percentage pricing details
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tiered_percentage_pricing_model: Option<TieredPercentagePricingModelInput>,
    /// Tiered pricing details
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tiered_pricing_model: Option<TieredPricingModelInput>,
    /// Pricing model used by this cell
    pub r#type: MatrixPriceCellInputType,
    /// Unit pricing details
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_pricing_model: Option<UnitPricingModelInput>,
    /// Volume percentage pricing details
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume_percentage_pricing_model: Option<VolumePercentagePricingModelInput>,
}

impl MatrixPriceCellInput {
    pub fn builder() -> MatrixPriceCellInputBuilder {
        <MatrixPriceCellInputBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct MatrixPriceCellInputBuilder {
    cell_key: Option<String>,
    fixed_pricing_model: Option<FixedPricingModelInput>,
    graduated_percentage_pricing_model: Option<GraduatedPercentagePricingModelInput>,
    graduated_tiered_pricing_model: Option<GraduatedTieredPricingModelInput>,
    label: Option<String>,
    tiered_percentage_pricing_model: Option<TieredPercentagePricingModelInput>,
    tiered_pricing_model: Option<TieredPricingModelInput>,
    r#type: Option<MatrixPriceCellInputType>,
    unit_pricing_model: Option<UnitPricingModelInput>,
    volume_percentage_pricing_model: Option<VolumePercentagePricingModelInput>,
}

impl MatrixPriceCellInputBuilder {
    pub fn cell_key(mut self, value: impl Into<String>) -> Self {
        self.cell_key = Some(value.into());
        self
    }

    pub fn fixed_pricing_model(mut self, value: FixedPricingModelInput) -> Self {
        self.fixed_pricing_model = Some(value);
        self
    }

    pub fn graduated_percentage_pricing_model(
        mut self,
        value: GraduatedPercentagePricingModelInput,
    ) -> Self {
        self.graduated_percentage_pricing_model = Some(value);
        self
    }

    pub fn graduated_tiered_pricing_model(
        mut self,
        value: GraduatedTieredPricingModelInput,
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
        value: TieredPercentagePricingModelInput,
    ) -> Self {
        self.tiered_percentage_pricing_model = Some(value);
        self
    }

    pub fn tiered_pricing_model(mut self, value: TieredPricingModelInput) -> Self {
        self.tiered_pricing_model = Some(value);
        self
    }

    pub fn r#type(mut self, value: MatrixPriceCellInputType) -> Self {
        self.r#type = Some(value);
        self
    }

    pub fn unit_pricing_model(mut self, value: UnitPricingModelInput) -> Self {
        self.unit_pricing_model = Some(value);
        self
    }

    pub fn volume_percentage_pricing_model(
        mut self,
        value: VolumePercentagePricingModelInput,
    ) -> Self {
        self.volume_percentage_pricing_model = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`MatrixPriceCellInput`].
    /// This method will fail if any of the following fields are not set:
    /// - [`r#type`](MatrixPriceCellInputBuilder::r#type)
    pub fn build(self) -> Result<MatrixPriceCellInput, BuildError> {
        Ok(MatrixPriceCellInput {
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
