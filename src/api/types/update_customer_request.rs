pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UpdateCustomerRequest {
    /// Alternative identifiers for the customer
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aliases: Option<Vec<String>>,
    /// Customer billing address
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_address: Option<AddressRequest>,
    /// Customer contacts. Replaces all existing contacts when provided
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contacts: Option<Vec<ContactRequest>>,
    /// ISO 4217 currency code
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    /// The name of the customer
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl UpdateCustomerRequest {
    pub fn builder() -> UpdateCustomerRequestBuilder {
        <UpdateCustomerRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateCustomerRequestBuilder {
    aliases: Option<Vec<String>>,
    billing_address: Option<AddressRequest>,
    contacts: Option<Vec<ContactRequest>>,
    currency: Option<String>,
    name: Option<String>,
}

impl UpdateCustomerRequestBuilder {
    pub fn aliases(mut self, value: Vec<String>) -> Self {
        self.aliases = Some(value);
        self
    }

    pub fn billing_address(mut self, value: AddressRequest) -> Self {
        self.billing_address = Some(value);
        self
    }

    pub fn contacts(mut self, value: Vec<ContactRequest>) -> Self {
        self.contacts = Some(value);
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

    /// Consumes the builder and constructs a [`UpdateCustomerRequest`].
    pub fn build(self) -> Result<UpdateCustomerRequest, BuildError> {
        Ok(UpdateCustomerRequest {
            aliases: self.aliases,
            billing_address: self.billing_address,
            contacts: self.contacts,
            currency: self.currency,
            name: self.name,
        })
    }
}
