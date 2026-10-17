pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct MerchantResponse {
    /// When the merchant was archived. Absent while the merchant is active; archived merchants take no new document attribution but keep rendering on historical documents.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub archived_at: Option<DateTime<FixedOffset>>,
    /// Square mark for places too small for the full logo, such as list rows
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avatar_url: Option<String>,
    /// When the merchant was created
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub created_at: DateTime<FixedOffset>,
    /// Name shown on the documents and emails issued for this merchant
    pub display_name: String,
    /// Your own identifier for the merchant, unique within your organization
    pub external_id: String,
    /// Unique identifier for the merchant
    pub id: String,
    /// Logo shown beside yours in the header of documents and emails issued for this merchant
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logo_url: Option<String>,
    /// Support address shown to this merchant's buyers on its documents and emails
    #[serde(skip_serializing_if = "Option::is_none")]
    pub support_email: Option<String>,
    /// When the merchant was last updated
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub updated_at: DateTime<FixedOffset>,
}

impl MerchantResponse {
    pub fn builder() -> MerchantResponseBuilder {
        <MerchantResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct MerchantResponseBuilder {
    archived_at: Option<DateTime<FixedOffset>>,
    avatar_url: Option<String>,
    created_at: Option<DateTime<FixedOffset>>,
    display_name: Option<String>,
    external_id: Option<String>,
    id: Option<String>,
    logo_url: Option<String>,
    support_email: Option<String>,
    updated_at: Option<DateTime<FixedOffset>>,
}

impl MerchantResponseBuilder {
    pub fn archived_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.archived_at = Some(value);
        self
    }

    pub fn avatar_url(mut self, value: impl Into<String>) -> Self {
        self.avatar_url = Some(value.into());
        self
    }

    pub fn created_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.created_at = Some(value);
        self
    }

    pub fn display_name(mut self, value: impl Into<String>) -> Self {
        self.display_name = Some(value.into());
        self
    }

    pub fn external_id(mut self, value: impl Into<String>) -> Self {
        self.external_id = Some(value.into());
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn logo_url(mut self, value: impl Into<String>) -> Self {
        self.logo_url = Some(value.into());
        self
    }

    pub fn support_email(mut self, value: impl Into<String>) -> Self {
        self.support_email = Some(value.into());
        self
    }

    pub fn updated_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.updated_at = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`MerchantResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`created_at`](MerchantResponseBuilder::created_at)
    /// - [`display_name`](MerchantResponseBuilder::display_name)
    /// - [`external_id`](MerchantResponseBuilder::external_id)
    /// - [`id`](MerchantResponseBuilder::id)
    /// - [`updated_at`](MerchantResponseBuilder::updated_at)
    pub fn build(self) -> Result<MerchantResponse, BuildError> {
        Ok(MerchantResponse {
            archived_at: self.archived_at,
            avatar_url: self.avatar_url,
            created_at: self
                .created_at
                .ok_or_else(|| BuildError::missing_field("created_at"))?,
            display_name: self
                .display_name
                .ok_or_else(|| BuildError::missing_field("display_name"))?,
            external_id: self
                .external_id
                .ok_or_else(|| BuildError::missing_field("external_id"))?,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            logo_url: self.logo_url,
            support_email: self.support_email,
            updated_at: self
                .updated_at
                .ok_or_else(|| BuildError::missing_field("updated_at"))?,
        })
    }
}
