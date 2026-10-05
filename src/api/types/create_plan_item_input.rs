pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct CreatePlanItemInput {
    /// Description for the bundle (bundle items only)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bundle_description: Option<String>,
    /// Source bundle ID (for bundle items)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bundle_id: Option<String>,
    /// Bundle child items (for bundle items)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<CreatePlanBundleChildItemInput>>,
    /// Display name for the bundle (bundle items only)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Price definition (for standalone items)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<PriceInput>,
    /// Product ID (for standalone items)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_id: Option<String>,
}

impl CreatePlanItemInput {
    pub fn builder() -> CreatePlanItemInputBuilder {
        <CreatePlanItemInputBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreatePlanItemInputBuilder {
    bundle_description: Option<String>,
    bundle_id: Option<String>,
    items: Option<Vec<CreatePlanBundleChildItemInput>>,
    name: Option<String>,
    price: Option<PriceInput>,
    product_id: Option<String>,
}

impl CreatePlanItemInputBuilder {
    pub fn bundle_description(mut self, value: impl Into<String>) -> Self {
        self.bundle_description = Some(value.into());
        self
    }

    pub fn bundle_id(mut self, value: impl Into<String>) -> Self {
        self.bundle_id = Some(value.into());
        self
    }

    pub fn items(mut self, value: Vec<CreatePlanBundleChildItemInput>) -> Self {
        self.items = Some(value);
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn price(mut self, value: PriceInput) -> Self {
        self.price = Some(value);
        self
    }

    pub fn product_id(mut self, value: impl Into<String>) -> Self {
        self.product_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CreatePlanItemInput`].
    pub fn build(self) -> Result<CreatePlanItemInput, BuildError> {
        Ok(CreatePlanItemInput {
            bundle_description: self.bundle_description,
            bundle_id: self.bundle_id,
            items: self.items,
            name: self.name,
            price: self.price,
            product_id: self.product_id,
        })
    }
}
