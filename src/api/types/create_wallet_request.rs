pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreateWalletRequest {
    /// Currency code
    pub currency: String,
    /// Identifier of the customer
    pub customer_id: String,
    /// Name of the wallet
    pub name: String,
    /// Product IDs that can be purchased with this wallet
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_ids: Option<Vec<String>>,
}

impl CreateWalletRequest {
    pub fn builder() -> CreateWalletRequestBuilder {
        <CreateWalletRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateWalletRequestBuilder {
    currency: Option<String>,
    customer_id: Option<String>,
    name: Option<String>,
    product_ids: Option<Vec<String>>,
}

impl CreateWalletRequestBuilder {
    pub fn currency(mut self, value: impl Into<String>) -> Self {
        self.currency = Some(value.into());
        self
    }

    pub fn customer_id(mut self, value: impl Into<String>) -> Self {
        self.customer_id = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn product_ids(mut self, value: Vec<String>) -> Self {
        self.product_ids = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CreateWalletRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`currency`](CreateWalletRequestBuilder::currency)
    /// - [`customer_id`](CreateWalletRequestBuilder::customer_id)
    /// - [`name`](CreateWalletRequestBuilder::name)
    pub fn build(self) -> Result<CreateWalletRequest, BuildError> {
        Ok(CreateWalletRequest {
            currency: self
                .currency
                .ok_or_else(|| BuildError::missing_field("currency"))?,
            customer_id: self
                .customer_id
                .ok_or_else(|| BuildError::missing_field("customer_id"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            product_ids: self.product_ids,
        })
    }
}
