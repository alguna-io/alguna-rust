pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CheckoutAddressRequest {
    /// City
    pub city: String,
    /// ISO 3166-1 alpha-3 country code. An alpha-2 code or an English country name is also accepted and normalised to alpha-3. A future API version will require alpha-3.
    pub country: String,
    /// Address line 1
    pub line1: String,
    /// Address line 2
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line2: Option<String>,
    /// Postal or ZIP code
    pub postal_code: String,
    /// State / province / region
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}

impl CheckoutAddressRequest {
    pub fn builder() -> CheckoutAddressRequestBuilder {
        <CheckoutAddressRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CheckoutAddressRequestBuilder {
    city: Option<String>,
    country: Option<String>,
    line1: Option<String>,
    line2: Option<String>,
    postal_code: Option<String>,
    state: Option<String>,
}

impl CheckoutAddressRequestBuilder {
    pub fn city(mut self, value: impl Into<String>) -> Self {
        self.city = Some(value.into());
        self
    }

    pub fn country(mut self, value: impl Into<String>) -> Self {
        self.country = Some(value.into());
        self
    }

    pub fn line1(mut self, value: impl Into<String>) -> Self {
        self.line1 = Some(value.into());
        self
    }

    pub fn line2(mut self, value: impl Into<String>) -> Self {
        self.line2 = Some(value.into());
        self
    }

    pub fn postal_code(mut self, value: impl Into<String>) -> Self {
        self.postal_code = Some(value.into());
        self
    }

    pub fn state(mut self, value: impl Into<String>) -> Self {
        self.state = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CheckoutAddressRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`city`](CheckoutAddressRequestBuilder::city)
    /// - [`country`](CheckoutAddressRequestBuilder::country)
    /// - [`line1`](CheckoutAddressRequestBuilder::line1)
    /// - [`postal_code`](CheckoutAddressRequestBuilder::postal_code)
    pub fn build(self) -> Result<CheckoutAddressRequest, BuildError> {
        Ok(CheckoutAddressRequest {
            city: self.city.ok_or_else(|| BuildError::missing_field("city"))?,
            country: self
                .country
                .ok_or_else(|| BuildError::missing_field("country"))?,
            line1: self
                .line1
                .ok_or_else(|| BuildError::missing_field("line1"))?,
            line2: self.line2,
            postal_code: self
                .postal_code
                .ok_or_else(|| BuildError::missing_field("postal_code"))?,
            state: self.state,
        })
    }
}
