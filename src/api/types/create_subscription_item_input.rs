pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct CreateSubscriptionItemInput {
    /// Bundle ID (for bundle items)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bundle_id: Option<String>,
    /// Bundle child items (for bundle items)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<CreateSubscriptionBundleChildItemInput>>,
    /// Price definition (for standalone items)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<PriceInput>,
    /// Product ID (for standalone items)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_id: Option<String>,
}

impl CreateSubscriptionItemInput {
    pub fn builder() -> CreateSubscriptionItemInputBuilder {
        <CreateSubscriptionItemInputBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateSubscriptionItemInputBuilder {
    bundle_id: Option<String>,
    items: Option<Vec<CreateSubscriptionBundleChildItemInput>>,
    price: Option<PriceInput>,
    product_id: Option<String>,
}

impl CreateSubscriptionItemInputBuilder {
    pub fn bundle_id(mut self, value: impl Into<String>) -> Self {
        self.bundle_id = Some(value.into());
        self
    }

    pub fn items(mut self, value: Vec<CreateSubscriptionBundleChildItemInput>) -> Self {
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

    /// Consumes the builder and constructs a [`CreateSubscriptionItemInput`].
    pub fn build(self) -> Result<CreateSubscriptionItemInput, BuildError> {
        Ok(CreateSubscriptionItemInput {
            bundle_id: self.bundle_id,
            items: self.items,
            price: self.price,
            product_id: self.product_id,
        })
    }
}
