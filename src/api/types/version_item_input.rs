pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct VersionItemInput {
    /// Bundle ID (for bundle items)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bundle_id: Option<String>,
    /// Bundle child items with full price definitions
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<VersionBundleChildInput>>,
    /// Full price definition (required for standalone items)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<PriceInput>,
    /// Product ID (for standalone items)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_id: Option<String>,
}

impl VersionItemInput {
    pub fn builder() -> VersionItemInputBuilder {
        <VersionItemInputBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct VersionItemInputBuilder {
    bundle_id: Option<String>,
    items: Option<Vec<VersionBundleChildInput>>,
    price: Option<PriceInput>,
    product_id: Option<String>,
}

impl VersionItemInputBuilder {
    pub fn bundle_id(mut self, value: impl Into<String>) -> Self {
        self.bundle_id = Some(value.into());
        self
    }

    pub fn items(mut self, value: Vec<VersionBundleChildInput>) -> Self {
        self.items = Some(value);
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

    /// Consumes the builder and constructs a [`VersionItemInput`].
    pub fn build(self) -> Result<VersionItemInput, BuildError> {
        Ok(VersionItemInput {
            bundle_id: self.bundle_id,
            items: self.items,
            price: self.price,
            product_id: self.product_id,
        })
    }
}
