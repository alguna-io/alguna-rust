pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct InvoiceListResponse {
    /// List of invoices
    pub data: Vec<InvoiceListItemResponse>,
    /// Pagination metadata
    pub pagination: PaginationMeta,
}

impl InvoiceListResponse {
    pub fn builder() -> InvoiceListResponseBuilder {
        <InvoiceListResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct InvoiceListResponseBuilder {
    data: Option<Vec<InvoiceListItemResponse>>,
    pagination: Option<PaginationMeta>,
}

impl InvoiceListResponseBuilder {
    pub fn data(mut self, value: Vec<InvoiceListItemResponse>) -> Self {
        self.data = Some(value);
        self
    }

    pub fn pagination(mut self, value: PaginationMeta) -> Self {
        self.pagination = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`InvoiceListResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`data`](InvoiceListResponseBuilder::data)
    /// - [`pagination`](InvoiceListResponseBuilder::pagination)
    pub fn build(self) -> Result<InvoiceListResponse, BuildError> {
        Ok(InvoiceListResponse {
            data: self.data.ok_or_else(|| BuildError::missing_field("data"))?,
            pagination: self
                .pagination
                .ok_or_else(|| BuildError::missing_field("pagination"))?,
        })
    }
}
