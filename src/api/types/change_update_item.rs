pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ChangeUpdateItem {
    /// New items to add to the bundle
    #[serde(skip_serializing_if = "Option::is_none")]
    pub add_items: Option<Vec<ChangeAddItemChild>>,
    /// Partial price adjustment (merge semantics)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub adjust: Option<PriceAdjustInput>,
    /// Bundle ID (for bundle items)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bundle_id: Option<String>,
    /// Bundle child items to adjust or replace
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<ChangeUpdateItemChild>>,
    /// Full price replacement
    #[serde(skip_serializing_if = "Option::is_none")]
    pub new_price: Option<PriceInput>,
    /// Product ID (for standalone items)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_id: Option<String>,
    /// Items to remove from the bundle
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remove_items: Option<Vec<ChangeRemoveItemChild>>,
}

impl ChangeUpdateItem {
    pub fn builder() -> ChangeUpdateItemBuilder {
        <ChangeUpdateItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ChangeUpdateItemBuilder {
    add_items: Option<Vec<ChangeAddItemChild>>,
    adjust: Option<PriceAdjustInput>,
    bundle_id: Option<String>,
    items: Option<Vec<ChangeUpdateItemChild>>,
    new_price: Option<PriceInput>,
    product_id: Option<String>,
    remove_items: Option<Vec<ChangeRemoveItemChild>>,
}

impl ChangeUpdateItemBuilder {
    pub fn add_items(mut self, value: Vec<ChangeAddItemChild>) -> Self {
        self.add_items = Some(value);
        self
    }

    pub fn adjust(mut self, value: PriceAdjustInput) -> Self {
        self.adjust = Some(value);
        self
    }

    pub fn bundle_id(mut self, value: impl Into<String>) -> Self {
        self.bundle_id = Some(value.into());
        self
    }

    pub fn items(mut self, value: Vec<ChangeUpdateItemChild>) -> Self {
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

    pub fn remove_items(mut self, value: Vec<ChangeRemoveItemChild>) -> Self {
        self.remove_items = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ChangeUpdateItem`].
    pub fn build(self) -> Result<ChangeUpdateItem, BuildError> {
        Ok(ChangeUpdateItem {
            add_items: self.add_items,
            adjust: self.adjust,
            bundle_id: self.bundle_id,
            items: self.items,
            new_price: self.new_price,
            product_id: self.product_id,
            remove_items: self.remove_items,
        })
    }
}
