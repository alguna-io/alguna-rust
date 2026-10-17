pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CreatePlanBundleChildItemInput {
    /// Price definition
    pub price: PriceInput,
    /// Product ID
    pub product_id: String,
}

impl CreatePlanBundleChildItemInput {
    pub fn builder() -> CreatePlanBundleChildItemInputBuilder {
        <CreatePlanBundleChildItemInputBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreatePlanBundleChildItemInputBuilder {
    price: Option<PriceInput>,
    product_id: Option<String>,
}

impl CreatePlanBundleChildItemInputBuilder {
    pub fn price(mut self, value: PriceInput) -> Self {
        self.price = Some(value);
        self
    }

    pub fn product_id(mut self, value: impl Into<String>) -> Self {
        self.product_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CreatePlanBundleChildItemInput`].
    /// This method will fail if any of the following fields are not set:
    /// - [`price`](CreatePlanBundleChildItemInputBuilder::price)
    /// - [`product_id`](CreatePlanBundleChildItemInputBuilder::product_id)
    pub fn build(self) -> Result<CreatePlanBundleChildItemInput, BuildError> {
        Ok(CreatePlanBundleChildItemInput {
            price: self
                .price
                .ok_or_else(|| BuildError::missing_field("price"))?,
            product_id: self
                .product_id
                .ok_or_else(|| BuildError::missing_field("product_id"))?,
        })
    }
}
