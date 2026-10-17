pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UpdateProductRequest {
    /// A brief description of the product
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// The name of the product
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// How revenue is allocated across time periods within a billing cycle
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revenue_allocation_method: Option<RevenueAllocationMethod>,
    /// Stock keeping unit code used in external ERP or revenue recognition systems
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sku: Option<String>,
    /// Identifiers for tags to associate with this product
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_ids: Option<Vec<String>>,
    /// Canonical (provider-neutral) tax code. Omit for 'general' (standard-rated).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_code: Option<String>,
    /// Custom plural term for multiple units of the product (e.g. 'API calls'). Falls back to 'units' when omitted
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_label_plural: Option<String>,
    /// Custom singular term for one unit of the product (e.g. 'API call'). Falls back to 'unit' when omitted
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_label_singular: Option<String>,
}

impl UpdateProductRequest {
    pub fn builder() -> UpdateProductRequestBuilder {
        <UpdateProductRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateProductRequestBuilder {
    description: Option<String>,
    name: Option<String>,
    revenue_allocation_method: Option<RevenueAllocationMethod>,
    sku: Option<String>,
    tag_ids: Option<Vec<String>>,
    tax_code: Option<String>,
    unit_label_plural: Option<String>,
    unit_label_singular: Option<String>,
}

impl UpdateProductRequestBuilder {
    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn revenue_allocation_method(mut self, value: RevenueAllocationMethod) -> Self {
        self.revenue_allocation_method = Some(value);
        self
    }

    pub fn sku(mut self, value: impl Into<String>) -> Self {
        self.sku = Some(value.into());
        self
    }

    pub fn tag_ids(mut self, value: Vec<String>) -> Self {
        self.tag_ids = Some(value);
        self
    }

    pub fn tax_code(mut self, value: impl Into<String>) -> Self {
        self.tax_code = Some(value.into());
        self
    }

    pub fn unit_label_plural(mut self, value: impl Into<String>) -> Self {
        self.unit_label_plural = Some(value.into());
        self
    }

    pub fn unit_label_singular(mut self, value: impl Into<String>) -> Self {
        self.unit_label_singular = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`UpdateProductRequest`].
    pub fn build(self) -> Result<UpdateProductRequest, BuildError> {
        Ok(UpdateProductRequest {
            description: self.description,
            name: self.name,
            revenue_allocation_method: self.revenue_allocation_method,
            sku: self.sku,
            tag_ids: self.tag_ids,
            tax_code: self.tax_code,
            unit_label_plural: self.unit_label_plural,
            unit_label_singular: self.unit_label_singular,
        })
    }
}
