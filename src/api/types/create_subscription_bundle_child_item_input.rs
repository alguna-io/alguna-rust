pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CreateSubscriptionBundleChildItemInput {
    /// Price definition
    pub price: PriceInput,
    /// Product ID
    pub product_id: String,
}

impl CreateSubscriptionBundleChildItemInput {
    pub fn builder() -> CreateSubscriptionBundleChildItemInputBuilder {
        <CreateSubscriptionBundleChildItemInputBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateSubscriptionBundleChildItemInputBuilder {
    price: Option<PriceInput>,
    product_id: Option<String>,
}

impl CreateSubscriptionBundleChildItemInputBuilder {
    pub fn price(mut self, value: PriceInput) -> Self {
        self.price = Some(value);
        self
    }

    pub fn product_id(mut self, value: impl Into<String>) -> Self {
        self.product_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CreateSubscriptionBundleChildItemInput`].
    /// This method will fail if any of the following fields are not set:
    /// - [`price`](CreateSubscriptionBundleChildItemInputBuilder::price)
    /// - [`product_id`](CreateSubscriptionBundleChildItemInputBuilder::product_id)
    pub fn build(self) -> Result<CreateSubscriptionBundleChildItemInput, BuildError> {
        Ok(CreateSubscriptionBundleChildItemInput {
            price: self
                .price
                .ok_or_else(|| BuildError::missing_field("price"))?,
            product_id: self
                .product_id
                .ok_or_else(|| BuildError::missing_field("product_id"))?,
        })
    }
}
