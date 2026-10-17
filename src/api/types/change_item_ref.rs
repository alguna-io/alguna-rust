pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ChangeItemRef {
    /// Bundle ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bundle_id: Option<String>,
    /// Product ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_id: Option<String>,
}

impl ChangeItemRef {
    pub fn builder() -> ChangeItemRefBuilder {
        <ChangeItemRefBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ChangeItemRefBuilder {
    bundle_id: Option<String>,
    product_id: Option<String>,
}

impl ChangeItemRefBuilder {
    pub fn bundle_id(mut self, value: impl Into<String>) -> Self {
        self.bundle_id = Some(value.into());
        self
    }

    pub fn product_id(mut self, value: impl Into<String>) -> Self {
        self.product_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ChangeItemRef`].
    pub fn build(self) -> Result<ChangeItemRef, BuildError> {
        Ok(ChangeItemRef {
            bundle_id: self.bundle_id,
            product_id: self.product_id,
        })
    }
}
