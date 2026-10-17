pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AddressRequest {
    /// City
    pub city: String,
    /// ISO 3166-1 alpha-3 country code
    pub country: String,
    /// Street address line 1
    pub line1: String,
    /// Street address line 2
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line2: Option<String>,
    /// Postal or ZIP code
    pub postal_code: String,
    /// State or province
    pub state: String,
}

impl AddressRequest {
    pub fn builder() -> AddressRequestBuilder {
        <AddressRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AddressRequestBuilder {
    city: Option<String>,
    country: Option<String>,
    line1: Option<String>,
    line2: Option<String>,
    postal_code: Option<String>,
    state: Option<String>,
}

impl AddressRequestBuilder {
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

    /// Consumes the builder and constructs a [`AddressRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`city`](AddressRequestBuilder::city)
    /// - [`country`](AddressRequestBuilder::country)
    /// - [`line1`](AddressRequestBuilder::line1)
    /// - [`postal_code`](AddressRequestBuilder::postal_code)
    /// - [`state`](AddressRequestBuilder::state)
    pub fn build(self) -> Result<AddressRequest, BuildError> {
        Ok(AddressRequest {
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
            state: self
                .state
                .ok_or_else(|| BuildError::missing_field("state"))?,
        })
    }
}
