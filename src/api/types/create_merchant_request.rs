pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreateMerchantRequest {
    /// Square mark for places too small for the full logo. We host it and return its URL as avatar_url.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avatar: Option<ImageUpload>,
    /// Name shown on the documents and emails issued for this merchant
    pub display_name: String,
    /// Your own identifier for the merchant, unique within your organization.
    pub external_id: String,
    /// Logo shown beside yours in the header of this merchant's documents and emails. We host it and return its URL as logo_url.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logo: Option<ImageUpload>,
    /// Support address shown to this merchant's buyers. Omit it to show yours instead.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub support_email: Option<String>,
}

impl CreateMerchantRequest {
    pub fn builder() -> CreateMerchantRequestBuilder {
        <CreateMerchantRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateMerchantRequestBuilder {
    avatar: Option<ImageUpload>,
    display_name: Option<String>,
    external_id: Option<String>,
    logo: Option<ImageUpload>,
    support_email: Option<String>,
}

impl CreateMerchantRequestBuilder {
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

    /// Consumes the builder and constructs a [`CreateMerchantRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`display_name`](CreateMerchantRequestBuilder::display_name)
    /// - [`external_id`](CreateMerchantRequestBuilder::external_id)
    pub fn build(self) -> Result<CreateMerchantRequest, BuildError> {
        Ok(CreateMerchantRequest {
            avatar: self.avatar,
            display_name: self
                .display_name
                .ok_or_else(|| BuildError::missing_field("display_name"))?,
            external_id: self
                .external_id
                .ok_or_else(|| BuildError::missing_field("external_id"))?,
            logo: self.logo,
            support_email: self.support_email,
        })
    }
}
