pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UpdateMerchantRequest {
    /// Replacement square mark. Send it with an empty content to clear the current one.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avatar: Option<ImageUpload>,
    /// Name shown on co-branded documents and emails
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    /// Your own identifier for the merchant, unique within your organization
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_id: Option<String>,
    /// Replacement logo image. Send it with an empty content to clear the current one.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logo: Option<ImageUpload>,
    /// Support address shown to this merchant's buyers. Send an empty string to fall back to yours.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub support_email: Option<String>,
}

impl UpdateMerchantRequest {
    pub fn builder() -> UpdateMerchantRequestBuilder {
        <UpdateMerchantRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateMerchantRequestBuilder {
    avatar: Option<ImageUpload>,
    display_name: Option<String>,
    external_id: Option<String>,
    logo: Option<ImageUpload>,
    support_email: Option<String>,
}

impl UpdateMerchantRequestBuilder {
    pub fn avatar(mut self, value: ImageUpload) -> Self {
        self.avatar = Some(value);
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

    pub fn logo(mut self, value: ImageUpload) -> Self {
        self.logo = Some(value);
        self
    }

    pub fn support_email(mut self, value: impl Into<String>) -> Self {
        self.support_email = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`UpdateMerchantRequest`].
    pub fn build(self) -> Result<UpdateMerchantRequest, BuildError> {
        Ok(UpdateMerchantRequest {
            avatar: self.avatar,
            display_name: self.display_name,
            external_id: self.external_id,
            logo: self.logo,
            support_email: self.support_email,
        })
    }
}
