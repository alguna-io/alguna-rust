pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CheckoutCustomerRequest {
    /// Billing address. When provided, checkout skips address re-entry.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<CheckoutAddressRequest>,
    /// Billing email; becomes the account's primary contact
    pub email: String,
    /// Primary contact first name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,
    /// Primary contact last name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
    /// Company or customer name the account is created with
    pub name: String,
    /// Company website. When an account with this website already exists, checkout completion updates it instead of creating a duplicate.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub website_url: Option<String>,
}

impl CheckoutCustomerRequest {
    pub fn builder() -> CheckoutCustomerRequestBuilder {
        <CheckoutCustomerRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CheckoutCustomerRequestBuilder {
    address: Option<CheckoutAddressRequest>,
    email: Option<String>,
    first_name: Option<String>,
    last_name: Option<String>,
    name: Option<String>,
    website_url: Option<String>,
}

impl CheckoutCustomerRequestBuilder {
    pub fn address(mut self, value: CheckoutAddressRequest) -> Self {
        self.address = Some(value);
        self
    }

    pub fn email(mut self, value: impl Into<String>) -> Self {
        self.email = Some(value.into());
        self
    }

    pub fn first_name(mut self, value: impl Into<String>) -> Self {
        self.first_name = Some(value.into());
        self
    }

    pub fn last_name(mut self, value: impl Into<String>) -> Self {
        self.last_name = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn website_url(mut self, value: impl Into<String>) -> Self {
        self.website_url = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CheckoutCustomerRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`email`](CheckoutCustomerRequestBuilder::email)
    /// - [`name`](CheckoutCustomerRequestBuilder::name)
    pub fn build(self) -> Result<CheckoutCustomerRequest, BuildError> {
        Ok(CheckoutCustomerRequest {
            address: self.address,
            email: self
                .email
                .ok_or_else(|| BuildError::missing_field("email"))?,
            first_name: self.first_name,
            last_name: self.last_name,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            website_url: self.website_url,
        })
    }
}
