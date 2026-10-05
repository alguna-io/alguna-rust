pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct MetricListResponse {
    /// List of metrics
    pub data: Vec<MetricResponse>,
    /// Pagination metadata
    pub pagination: PaginationMeta,
}

impl MetricListResponse {
    pub fn builder() -> MetricListResponseBuilder {
        <MetricListResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct MetricListResponseBuilder {
    data: Option<Vec<MetricResponse>>,
    pagination: Option<PaginationMeta>,
}

impl MetricListResponseBuilder {
    pub fn data(mut self, value: Vec<MetricResponse>) -> Self {
        self.data = Some(value);
        self
    }

    pub fn pagination(mut self, value: PaginationMeta) -> Self {
        self.pagination = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`MetricListResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`data`](MetricListResponseBuilder::data)
    /// - [`pagination`](MetricListResponseBuilder::pagination)
    pub fn build(self) -> Result<MetricListResponse, BuildError> {
        Ok(MetricListResponse {
            data: self.data.ok_or_else(|| BuildError::missing_field("data"))?,
            pagination: self
                .pagination
                .ok_or_else(|| BuildError::missing_field("pagination"))?,
        })
    }
}
