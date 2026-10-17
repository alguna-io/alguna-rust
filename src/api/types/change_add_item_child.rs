pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ChangeAddItemChild {
    /// Price definition
    pub new_price: PriceInput,
    /// Product ID
    pub product_id: String,
}

impl ChangeAddItemChild {
    pub fn builder() -> ChangeAddItemChildBuilder {
        <ChangeAddItemChildBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ChangeAddItemChildBuilder {
    new_price: Option<PriceInput>,
    product_id: Option<String>,
}

impl ChangeAddItemChildBuilder {
    pub fn new_price(mut self, value: PriceInput) -> Self {
        self.new_price = Some(value);
        self
    }

    pub fn product_id(mut self, value: impl Into<String>) -> Self {
        self.product_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ChangeAddItemChild`].
    /// This method will fail if any of the following fields are not set:
    /// - [`new_price`](ChangeAddItemChildBuilder::new_price)
    /// - [`product_id`](ChangeAddItemChildBuilder::product_id)
    pub fn build(self) -> Result<ChangeAddItemChild, BuildError> {
        Ok(ChangeAddItemChild {
            new_price: self
                .new_price
                .ok_or_else(|| BuildError::missing_field("new_price"))?,
            product_id: self
                .product_id
                .ok_or_else(|| BuildError::missing_field("product_id"))?,
        })
    }
}
