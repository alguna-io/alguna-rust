pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct TaxCalculationListResponse {
    /// List of tax calculations
    pub data: Vec<TaxCalculationListItemResponse>,
    /// Pagination metadata
    pub pagination: PaginationMeta,
}

impl TaxCalculationListResponse {
    pub fn builder() -> TaxCalculationListResponseBuilder {
        <TaxCalculationListResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TaxCalculationListResponseBuilder {
    data: Option<Vec<TaxCalculationListItemResponse>>,
    pagination: Option<PaginationMeta>,
}

impl TaxCalculationListResponseBuilder {
    pub fn data(mut self, value: Vec<TaxCalculationListItemResponse>) -> Self {
        self.data = Some(value);
        self
    }

    pub fn pagination(mut self, value: PaginationMeta) -> Self {
        self.pagination = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TaxCalculationListResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`data`](TaxCalculationListResponseBuilder::data)
    /// - [`pagination`](TaxCalculationListResponseBuilder::pagination)
    pub fn build(self) -> Result<TaxCalculationListResponse, BuildError> {
        Ok(TaxCalculationListResponse {
            data: self.data.ok_or_else(|| BuildError::missing_field("data"))?,
            pagination: self
                .pagination
                .ok_or_else(|| BuildError::missing_field("pagination"))?,
        })
    }
}
