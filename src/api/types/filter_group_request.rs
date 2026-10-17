pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct FilterGroupRequest {
    /// Array of filters in this group
    pub filters: Vec<FilterRequest>,
    /// Logical operator combining filters in this group
    pub operator: FilterGroupOperator,
}

impl FilterGroupRequest {
    pub fn builder() -> FilterGroupRequestBuilder {
        <FilterGroupRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct FilterGroupRequestBuilder {
    filters: Option<Vec<FilterRequest>>,
    operator: Option<FilterGroupOperator>,
}

impl FilterGroupRequestBuilder {
    pub fn filters(mut self, value: Vec<FilterRequest>) -> Self {
        self.filters = Some(value);
        self
    }

    pub fn operator(mut self, value: FilterGroupOperator) -> Self {
        self.operator = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`FilterGroupRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`filters`](FilterGroupRequestBuilder::filters)
    /// - [`operator`](FilterGroupRequestBuilder::operator)
    pub fn build(self) -> Result<FilterGroupRequest, BuildError> {
        Ok(FilterGroupRequest {
            filters: self
                .filters
                .ok_or_else(|| BuildError::missing_field("filters"))?,
            operator: self
                .operator
                .ok_or_else(|| BuildError::missing_field("operator"))?,
        })
    }
}
