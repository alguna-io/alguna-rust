pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PaginationMeta {
    /// Number of items returned per page
    pub per_page: i64,
    /// Total number of pages available
    pub total_pages: i64,
}

impl PaginationMeta {
    pub fn builder() -> PaginationMetaBuilder {
        <PaginationMetaBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PaginationMetaBuilder {
    per_page: Option<i64>,
    total_pages: Option<i64>,
}

impl PaginationMetaBuilder {
    pub fn per_page(mut self, value: i64) -> Self {
        self.per_page = Some(value);
        self
    }

    pub fn total_pages(mut self, value: i64) -> Self {
        self.total_pages = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PaginationMeta`].
    /// This method will fail if any of the following fields are not set:
    /// - [`per_page`](PaginationMetaBuilder::per_page)
    /// - [`total_pages`](PaginationMetaBuilder::total_pages)
    pub fn build(self) -> Result<PaginationMeta, BuildError> {
        Ok(PaginationMeta {
            per_page: self
                .per_page
                .ok_or_else(|| BuildError::missing_field("per_page"))?,
            total_pages: self
                .total_pages
                .ok_or_else(|| BuildError::missing_field("total_pages"))?,
        })
    }
}
