pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ContactResponse {
    /// Contact email address
    pub email: String,
    /// Email notification preferences
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_preferences: Option<Vec<String>>,
    /// Contact first name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,
    /// Unique identifier for the contact
    pub id: String,
    /// Whether this is the primary contact
    pub is_primary: bool,
    /// Contact last name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
}

impl ContactResponse {
    pub fn builder() -> ContactResponseBuilder {
        <ContactResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ContactResponseBuilder {
    email: Option<String>,
    email_preferences: Option<Vec<String>>,
    first_name: Option<String>,
    id: Option<String>,
    is_primary: Option<bool>,
    last_name: Option<String>,
}

impl ContactResponseBuilder {
    pub fn email(mut self, value: impl Into<String>) -> Self {
        self.email = Some(value.into());
        self
    }

    pub fn email_preferences(mut self, value: Vec<String>) -> Self {
        self.email_preferences = Some(value);
        self
    }

    pub fn first_name(mut self, value: impl Into<String>) -> Self {
        self.first_name = Some(value.into());
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn is_primary(mut self, value: bool) -> Self {
        self.is_primary = Some(value);
        self
    }

    pub fn last_name(mut self, value: impl Into<String>) -> Self {
        self.last_name = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ContactResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`email`](ContactResponseBuilder::email)
    /// - [`id`](ContactResponseBuilder::id)
    /// - [`is_primary`](ContactResponseBuilder::is_primary)
    pub fn build(self) -> Result<ContactResponse, BuildError> {
        Ok(ContactResponse {
            email: self
                .email
                .ok_or_else(|| BuildError::missing_field("email"))?,
            email_preferences: self.email_preferences,
            first_name: self.first_name,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            is_primary: self
                .is_primary
                .ok_or_else(|| BuildError::missing_field("is_primary"))?,
            last_name: self.last_name,
        })
    }
}
