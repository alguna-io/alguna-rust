pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UpdateWalletRequest {
    /// Whether the wallet is active
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,
    /// Currency code
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    /// Name of the wallet
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Product IDs that can be purchased with this wallet
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_ids: Option<Vec<String>>,
}

impl UpdateWalletRequest {
    pub fn builder() -> UpdateWalletRequestBuilder {
        <UpdateWalletRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateWalletRequestBuilder {
    active: Option<bool>,
    currency: Option<String>,
    name: Option<String>,
    product_ids: Option<Vec<String>>,
}

impl UpdateWalletRequestBuilder {
    pub fn active(mut self, value: bool) -> Self {
        self.active = Some(value);
        self
    }

    pub fn currency(mut self, value: impl Into<String>) -> Self {
        self.currency = Some(value.into());
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

    /// Consumes the builder and constructs a [`UpdateWalletRequest`].
    pub fn build(self) -> Result<UpdateWalletRequest, BuildError> {
        Ok(UpdateWalletRequest {
            active: self.active,
            currency: self.currency,
            name: self.name,
            product_ids: self.product_ids,
        })
    }
}
