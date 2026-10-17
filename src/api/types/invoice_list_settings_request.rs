pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct InvoiceListSettingsRequest {
    /// Whether the invoice list section is shown. Falls back to the organization's default when omitted.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_section: Option<bool>,
}

impl InvoiceListSettingsRequest {
    pub fn builder() -> InvoiceListSettingsRequestBuilder {
        <InvoiceListSettingsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct InvoiceListSettingsRequestBuilder {
    show_section: Option<bool>,
}

impl InvoiceListSettingsRequestBuilder {
    pub fn show_section(mut self, value: bool) -> Self {
        self.show_section = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`InvoiceListSettingsRequest`].
    pub fn build(self) -> Result<InvoiceListSettingsRequest, BuildError> {
        Ok(InvoiceListSettingsRequest {
            show_section: self.show_section,
        })
    }
}
