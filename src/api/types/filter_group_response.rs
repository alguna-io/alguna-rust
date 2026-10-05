pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct FilterGroupResponse {
    /// Array of filters in this group
    pub filters: Vec<FilterResponse>,
    /// Logical operator combining filters in this group
    pub operator: FilterGroupOperator,
}

impl FilterGroupResponse {
    pub fn builder() -> FilterGroupResponseBuilder {
        <FilterGroupResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct FilterGroupResponseBuilder {
    filters: Option<Vec<FilterResponse>>,
    operator: Option<FilterGroupOperator>,
}

impl FilterGroupResponseBuilder {
    pub fn filters(mut self, value: Vec<FilterResponse>) -> Self {
        self.filters = Some(value);
        self
    }

    pub fn operator(mut self, value: FilterGroupOperator) -> Self {
        self.operator = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`FilterGroupResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`filters`](FilterGroupResponseBuilder::filters)
    /// - [`operator`](FilterGroupResponseBuilder::operator)
    pub fn build(self) -> Result<FilterGroupResponse, BuildError> {
        Ok(FilterGroupResponse {
            filters: self
                .filters
                .ok_or_else(|| BuildError::missing_field("filters"))?,
            operator: self
                .operator
                .ok_or_else(|| BuildError::missing_field("operator"))?,
        })
    }
}
