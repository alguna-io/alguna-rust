pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreateCustomerRequest {
    /// Alternative identifiers for the customer
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aliases: Option<Vec<String>>,
    /// Customer billing address
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_address: Option<AddressRequest>,
    /// Customer contacts
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contacts: Option<Vec<ContactRequest>>,
    /// ISO 4217 currency code
    pub currency: String,
    /// The name of the customer
    pub name: String,
}

impl CreateCustomerRequest {
    pub fn builder() -> CreateCustomerRequestBuilder {
        <CreateCustomerRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateCustomerRequestBuilder {
    aliases: Option<Vec<String>>,
    billing_address: Option<AddressRequest>,
    contacts: Option<Vec<ContactRequest>>,
    currency: Option<String>,
    name: Option<String>,
}

impl CreateCustomerRequestBuilder {
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

    /// Consumes the builder and constructs a [`CreateCustomerRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`currency`](CreateCustomerRequestBuilder::currency)
    /// - [`name`](CreateCustomerRequestBuilder::name)
    pub fn build(self) -> Result<CreateCustomerRequest, BuildError> {
        Ok(CreateCustomerRequest {
            aliases: self.aliases,
            billing_address: self.billing_address,
            contacts: self.contacts,
            currency: self
                .currency
                .ok_or_else(|| BuildError::missing_field("currency"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
        })
    }
}
