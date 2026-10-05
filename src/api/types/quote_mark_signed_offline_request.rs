pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct QuoteMarkSignedOfflineRequest {
    /// When the signer consented, as captured by your e-sign flow. Defaults to now.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signed_at: Option<DateTime<FixedOffset>>,
    /// Signer's email.
    pub signer_email: String,
    /// Full name of the signer.
    pub signer_name: String,
    /// Signer's job title.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signer_role: Option<String>,
}

impl QuoteMarkSignedOfflineRequest {
    pub fn builder() -> QuoteMarkSignedOfflineRequestBuilder {
        <QuoteMarkSignedOfflineRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct QuoteMarkSignedOfflineRequestBuilder {
    signed_at: Option<DateTime<FixedOffset>>,
    signer_email: Option<String>,
    signer_name: Option<String>,
    signer_role: Option<String>,
}

impl QuoteMarkSignedOfflineRequestBuilder {
    pub fn signed_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.signed_at = Some(value);
        self
    }

    pub fn signer_email(mut self, value: impl Into<String>) -> Self {
        self.signer_email = Some(value.into());
        self
    }

    pub fn signer_name(mut self, value: impl Into<String>) -> Self {
        self.signer_name = Some(value.into());
        self
    }

    pub fn signer_role(mut self, value: impl Into<String>) -> Self {
        self.signer_role = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`QuoteMarkSignedOfflineRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`signer_email`](QuoteMarkSignedOfflineRequestBuilder::signer_email)
    /// - [`signer_name`](QuoteMarkSignedOfflineRequestBuilder::signer_name)
    pub fn build(self) -> Result<QuoteMarkSignedOfflineRequest, BuildError> {
        Ok(QuoteMarkSignedOfflineRequest {
            signed_at: self.signed_at,
            signer_email: self
                .signer_email
                .ok_or_else(|| BuildError::missing_field("signer_email"))?,
            signer_name: self
                .signer_name
                .ok_or_else(|| BuildError::missing_field("signer_name"))?,
            signer_role: self.signer_role,
        })
    }
}
