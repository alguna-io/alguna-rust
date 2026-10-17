pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ChangeUpdateItemChild {
    /// Partial price adjustment
    #[serde(skip_serializing_if = "Option::is_none")]
    pub adjust: Option<PriceAdjustInput>,
    /// Full price replacement
    #[serde(skip_serializing_if = "Option::is_none")]
    pub new_price: Option<PriceInput>,
    /// Product ID
    pub product_id: String,
}

impl ChangeUpdateItemChild {
    pub fn builder() -> ChangeUpdateItemChildBuilder {
        <ChangeUpdateItemChildBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ChangeUpdateItemChildBuilder {
    adjust: Option<PriceAdjustInput>,
    new_price: Option<PriceInput>,
    product_id: Option<String>,
}

impl ChangeUpdateItemChildBuilder {
    pub fn adjust(mut self, value: PriceAdjustInput) -> Self {
        self.adjust = Some(value);
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

    /// Consumes the builder and constructs a [`ChangeUpdateItemChild`].
    /// This method will fail if any of the following fields are not set:
    /// - [`product_id`](ChangeUpdateItemChildBuilder::product_id)
    pub fn build(self) -> Result<ChangeUpdateItemChild, BuildError> {
        Ok(ChangeUpdateItemChild {
            adjust: self.adjust,
            new_price: self.new_price,
            product_id: self
                .product_id
                .ok_or_else(|| BuildError::missing_field("product_id"))?,
        })
    }
}
