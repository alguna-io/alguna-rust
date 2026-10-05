pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PlanListResponse {
    /// List of plans
    pub data: Vec<PlanResponse>,
    /// Pagination metadata
    pub pagination: PaginationMeta,
}

impl PlanListResponse {
    pub fn builder() -> PlanListResponseBuilder {
        <PlanListResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PlanListResponseBuilder {
    data: Option<Vec<PlanResponse>>,
    pagination: Option<PaginationMeta>,
}

impl PlanListResponseBuilder {
    pub fn data(mut self, value: Vec<PlanResponse>) -> Self {
        self.data = Some(value);
        self
    }

    pub fn pagination(mut self, value: PaginationMeta) -> Self {
        self.pagination = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PlanListResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`data`](PlanListResponseBuilder::data)
    /// - [`pagination`](PlanListResponseBuilder::pagination)
    pub fn build(self) -> Result<PlanListResponse, BuildError> {
        Ok(PlanListResponse {
            data: self.data.ok_or_else(|| BuildError::missing_field("data"))?,
            pagination: self
                .pagination
                .ok_or_else(|| BuildError::missing_field("pagination"))?,
        })
    }
}
