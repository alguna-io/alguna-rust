pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreditNoteAttachmentResponse {
    /// URL to the attachment file
    pub file_url: String,
    /// Title of the attachment
    pub title: String,
}

impl CreditNoteAttachmentResponse {
    pub fn builder() -> CreditNoteAttachmentResponseBuilder {
        <CreditNoteAttachmentResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreditNoteAttachmentResponseBuilder {
    file_url: Option<String>,
    title: Option<String>,
}

impl CreditNoteAttachmentResponseBuilder {
    pub fn file_url(mut self, value: impl Into<String>) -> Self {
        self.file_url = Some(value.into());
        self
    }

    pub fn title(mut self, value: impl Into<String>) -> Self {
        self.title = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CreditNoteAttachmentResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`file_url`](CreditNoteAttachmentResponseBuilder::file_url)
    /// - [`title`](CreditNoteAttachmentResponseBuilder::title)
    pub fn build(self) -> Result<CreditNoteAttachmentResponse, BuildError> {
        Ok(CreditNoteAttachmentResponse {
            file_url: self
                .file_url
                .ok_or_else(|| BuildError::missing_field("file_url"))?,
            title: self
                .title
                .ok_or_else(|| BuildError::missing_field("title"))?,
        })
    }
}
