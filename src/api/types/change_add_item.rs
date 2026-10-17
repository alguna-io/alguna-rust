pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ChangeAddItem {
    /// Bundle ID (for bundle items)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bundle_id: Option<String>,
    /// Bundle child items with prices
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<ChangeAddItemChild>>,
    /// Price definition for the new product
    #[serde(skip_serializing_if = "Option::is_none")]
    pub new_price: Option<PriceInput>,
    /// Product ID (for standalone items)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_id: Option<String>,
}

impl ChangeAddItem {
    pub fn builder() -> ChangeAddItemBuilder {
        <ChangeAddItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ChangeAddItemBuilder {
    bundle_id: Option<String>,
    items: Option<Vec<ChangeAddItemChild>>,
    new_price: Option<PriceInput>,
    product_id: Option<String>,
}

impl ChangeAddItemBuilder {
    pub fn bundle_id(mut self, value: impl Into<String>) -> Self {
        self.bundle_id = Some(value.into());
        self
    }

    pub fn items(mut self, value: Vec<ChangeAddItemChild>) -> Self {
        self.items = Some(value);
        self
    }

    pub fn new_price(mut self, value: PriceInput) -> Self {
        self.new_price = Some(value);
        self
    }

    pub fn product_id(mut self, value: impl Into<String>) -> Self {
        self.product_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ChangeAddItem`].
    pub fn build(self) -> Result<ChangeAddItem, BuildError> {
        Ok(ChangeAddItem {
            bundle_id: self.bundle_id,
            items: self.items,
            new_price: self.new_price,
            product_id: self.product_id,
        })
    }
}
