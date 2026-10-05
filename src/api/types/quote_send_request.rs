pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct QuoteSendRequest {
    /// Email the signing link is sent to. When omitted no email is sent; share signing_url yourself.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signer_email: Option<String>,
    /// Name of the person expected to sign, recorded on the signature session.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signer_name: Option<String>,
    /// Signer's job title, recorded on the signature session.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signer_role: Option<String>,
}

impl QuoteSendRequest {
    pub fn builder() -> QuoteSendRequestBuilder {
        <QuoteSendRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct QuoteSendRequestBuilder {
    signer_email: Option<String>,
    signer_name: Option<String>,
    signer_role: Option<String>,
}

impl QuoteSendRequestBuilder {
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

    /// Consumes the builder and constructs a [`QuoteSendRequest`].
    pub fn build(self) -> Result<QuoteSendRequest, BuildError> {
        Ok(QuoteSendRequest {
            signer_email: self.signer_email,
            signer_name: self.signer_name,
            signer_role: self.signer_role,
        })
    }
}
