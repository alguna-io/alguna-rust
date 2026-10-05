pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ChangeRemoveItem {
    /// Bundle ID to remove (cascade removes all children)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bundle_id: Option<String>,
    /// Product ID to remove
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_id: Option<String>,
}

impl ChangeRemoveItem {
    pub fn builder() -> ChangeRemoveItemBuilder {
        <ChangeRemoveItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ChangeRemoveItemBuilder {
    bundle_id: Option<String>,
    product_id: Option<String>,
}

impl ChangeRemoveItemBuilder {
    pub fn bundle_id(mut self, value: impl Into<String>) -> Self {
        self.bundle_id = Some(value.into());
        self
    }

    pub fn product_id(mut self, value: impl Into<String>) -> Self {
        self.product_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ChangeRemoveItem`].
    pub fn build(self) -> Result<ChangeRemoveItem, BuildError> {
        Ok(ChangeRemoveItem {
            bundle_id: self.bundle_id,
            product_id: self.product_id,
        })
    }
}
