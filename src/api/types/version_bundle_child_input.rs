pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct VersionBundleChildInput {
    /// Full price definition
    pub price: PriceInput,
    /// Product ID
    pub product_id: String,
}

impl VersionBundleChildInput {
    pub fn builder() -> VersionBundleChildInputBuilder {
        <VersionBundleChildInputBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct VersionBundleChildInputBuilder {
    price: Option<PriceInput>,
    product_id: Option<String>,
}

impl VersionBundleChildInputBuilder {
    pub fn price(mut self, value: PriceInput) -> Self {
        self.price = Some(value);
        self
    }

    pub fn product_id(mut self, value: impl Into<String>) -> Self {
        self.product_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`VersionBundleChildInput`].
    /// This method will fail if any of the following fields are not set:
    /// - [`price`](VersionBundleChildInputBuilder::price)
    /// - [`product_id`](VersionBundleChildInputBuilder::product_id)
    pub fn build(self) -> Result<VersionBundleChildInput, BuildError> {
        Ok(VersionBundleChildInput {
            price: self
                .price
                .ok_or_else(|| BuildError::missing_field("price"))?,
            product_id: self
                .product_id
                .ok_or_else(|| BuildError::missing_field("product_id"))?,
        })
    }
}
