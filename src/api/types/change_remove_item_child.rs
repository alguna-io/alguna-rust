pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ChangeRemoveItemChild {
    /// Product ID to remove
    pub product_id: String,
}

impl ChangeRemoveItemChild {
    pub fn builder() -> ChangeRemoveItemChildBuilder {
        <ChangeRemoveItemChildBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ChangeRemoveItemChildBuilder {
    product_id: Option<String>,
}

impl ChangeRemoveItemChildBuilder {
    pub fn product_id(mut self, value: impl Into<String>) -> Self {
        self.product_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ChangeRemoveItemChild`].
    /// This method will fail if any of the following fields are not set:
    /// - [`product_id`](ChangeRemoveItemChildBuilder::product_id)
    pub fn build(self) -> Result<ChangeRemoveItemChild, BuildError> {
        Ok(ChangeRemoveItemChild {
            product_id: self
                .product_id
                .ok_or_else(|| BuildError::missing_field("product_id"))?,
        })
    }
}
