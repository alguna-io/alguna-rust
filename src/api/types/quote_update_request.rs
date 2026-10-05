pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct QuoteUpdateRequest {
    /// Make the signing offer open-ended.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clear_expires_at: Option<bool>,
    /// Drop the override, falling back to the subscription's name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clear_name: Option<bool>,
    /// Move the signing deadline. Ignored when clear_expires_at is true.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<DateTime<FixedOffset>>,
    /// Name override for this quote. Does not rename the subscription. Ignored when clear_name is true.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl QuoteUpdateRequest {
    pub fn builder() -> QuoteUpdateRequestBuilder {
        <QuoteUpdateRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct QuoteUpdateRequestBuilder {
    clear_expires_at: Option<bool>,
    clear_name: Option<bool>,
    expires_at: Option<DateTime<FixedOffset>>,
    name: Option<String>,
}

impl QuoteUpdateRequestBuilder {
    pub fn clear_expires_at(mut self, value: bool) -> Self {
        self.clear_expires_at = Some(value);
        self
    }

    pub fn clear_name(mut self, value: bool) -> Self {
        self.clear_name = Some(value);
        self
    }

    pub fn expires_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.expires_at = Some(value);
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`QuoteUpdateRequest`].
    pub fn build(self) -> Result<QuoteUpdateRequest, BuildError> {
        Ok(QuoteUpdateRequest {
            clear_expires_at: self.clear_expires_at,
            clear_name: self.clear_name,
            expires_at: self.expires_at,
            name: self.name,
        })
    }
}
