pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CustomerPortalSessionResponse {
    /// When the session was created
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub created_at: DateTime<FixedOffset>,
    /// Customer ID the session was issued for
    pub customer_id: String,
    /// When the session URL will expire. After this, the customer must be redirected to a freshly created session.
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub expires_at: DateTime<FixedOffset>,
    /// Unique identifier for the session
    pub id: String,
    /// Hosted customer portal URL
    pub url: String,
}

impl CustomerPortalSessionResponse {
    pub fn builder() -> CustomerPortalSessionResponseBuilder {
        <CustomerPortalSessionResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CustomerPortalSessionResponseBuilder {
    created_at: Option<DateTime<FixedOffset>>,
    customer_id: Option<String>,
    expires_at: Option<DateTime<FixedOffset>>,
    id: Option<String>,
    url: Option<String>,
}

impl CustomerPortalSessionResponseBuilder {
    pub fn created_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.created_at = Some(value);
        self
    }

    pub fn customer_id(mut self, value: impl Into<String>) -> Self {
        self.customer_id = Some(value.into());
        self
    }

    pub fn expires_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.expires_at = Some(value);
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn url(mut self, value: impl Into<String>) -> Self {
        self.url = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CustomerPortalSessionResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`created_at`](CustomerPortalSessionResponseBuilder::created_at)
    /// - [`customer_id`](CustomerPortalSessionResponseBuilder::customer_id)
    /// - [`expires_at`](CustomerPortalSessionResponseBuilder::expires_at)
    /// - [`id`](CustomerPortalSessionResponseBuilder::id)
    /// - [`url`](CustomerPortalSessionResponseBuilder::url)
    pub fn build(self) -> Result<CustomerPortalSessionResponse, BuildError> {
        Ok(CustomerPortalSessionResponse {
            created_at: self
                .created_at
                .ok_or_else(|| BuildError::missing_field("created_at"))?,
            customer_id: self
                .customer_id
                .ok_or_else(|| BuildError::missing_field("customer_id"))?,
            expires_at: self
                .expires_at
                .ok_or_else(|| BuildError::missing_field("expires_at"))?,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            url: self.url.ok_or_else(|| BuildError::missing_field("url"))?,
        })
    }
}
