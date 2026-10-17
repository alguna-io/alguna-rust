pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BundleChildItemResponse {
    /// Price details
    pub price: SubscriptionPriceResponse,
    /// Product ID
    pub product_id: String,
}

impl BundleChildItemResponse {
    pub fn builder() -> BundleChildItemResponseBuilder {
        <BundleChildItemResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BundleChildItemResponseBuilder {
    price: Option<SubscriptionPriceResponse>,
    product_id: Option<String>,
}

impl BundleChildItemResponseBuilder {
    pub fn price(mut self, value: SubscriptionPriceResponse) -> Self {
        self.price = Some(value);
        self
    }

    pub fn product_id(mut self, value: impl Into<String>) -> Self {
        self.product_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`BundleChildItemResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`price`](BundleChildItemResponseBuilder::price)
    /// - [`product_id`](BundleChildItemResponseBuilder::product_id)
    pub fn build(self) -> Result<BundleChildItemResponse, BuildError> {
        Ok(BundleChildItemResponse {
            price: self
                .price
                .ok_or_else(|| BuildError::missing_field("price"))?,
            product_id: self
                .product_id
                .ok_or_else(|| BuildError::missing_field("product_id"))?,
        })
    }
}
