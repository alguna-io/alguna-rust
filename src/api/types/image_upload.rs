pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct ImageUpload {
    /// Base64-encoded image bytes, with no data-URL prefix. Up to 1MB decoded. On an update, send an empty string to clear the image.
    pub content: String,
    /// Media type of the bytes; it must match what the bytes actually are
    pub content_type: ImageUploadContentType,
}

impl ImageUpload {
    pub fn builder() -> ImageUploadBuilder {
        <ImageUploadBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ImageUploadBuilder {
    content: Option<String>,
    content_type: Option<ImageUploadContentType>,
}

impl ImageUploadBuilder {
    pub fn content(mut self, value: impl Into<String>) -> Self {
        self.content = Some(value.into());
        self
    }

    pub fn content_type(mut self, value: ImageUploadContentType) -> Self {
        self.content_type = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ImageUpload`].
    /// This method will fail if any of the following fields are not set:
    /// - [`content`](ImageUploadBuilder::content)
    /// - [`content_type`](ImageUploadBuilder::content_type)
    pub fn build(self) -> Result<ImageUpload, BuildError> {
        Ok(ImageUpload {
            content: self
                .content
                .ok_or_else(|| BuildError::missing_field("content"))?,
            content_type: self
                .content_type
                .ok_or_else(|| BuildError::missing_field("content_type"))?,
        })
    }
}
