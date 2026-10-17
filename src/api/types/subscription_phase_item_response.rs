pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct SubscriptionPhaseItemResponse {
    /// Bundle ID (for bundle items)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bundle_id: Option<String>,
    /// Bundle child items
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<BundleChildItemResponse>>,
    /// Price details (for standalone items)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<SubscriptionPriceResponse>,
    /// Product ID (for standalone items)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_id: Option<String>,
}

impl SubscriptionPhaseItemResponse {
    pub fn builder() -> SubscriptionPhaseItemResponseBuilder {
        <SubscriptionPhaseItemResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SubscriptionPhaseItemResponseBuilder {
    bundle_id: Option<String>,
    items: Option<Vec<BundleChildItemResponse>>,
    price: Option<SubscriptionPriceResponse>,
    product_id: Option<String>,
}

impl SubscriptionPhaseItemResponseBuilder {
    pub fn bundle_id(mut self, value: impl Into<String>) -> Self {
        self.bundle_id = Some(value.into());
        self
    }

    pub fn items(mut self, value: Vec<BundleChildItemResponse>) -> Self {
        self.items = Some(value);
        self
    }

    pub fn price(mut self, value: SubscriptionPriceResponse) -> Self {
        self.price = Some(value);
        self
    }

    pub fn product_id(mut self, value: impl Into<String>) -> Self {
        self.product_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`SubscriptionPhaseItemResponse`].
    pub fn build(self) -> Result<SubscriptionPhaseItemResponse, BuildError> {
        Ok(SubscriptionPhaseItemResponse {
            bundle_id: self.bundle_id,
            items: self.items,
            price: self.price,
            product_id: self.product_id,
        })
    }
}
