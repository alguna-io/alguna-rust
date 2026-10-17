pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct FilterRequest {
    /// Event property field name to filter by
    pub field: String,
    /// Filter comparison operator
    pub operator: String,
    /// Value to match against
    pub value: String,
}

impl FilterRequest {
    pub fn builder() -> FilterRequestBuilder {
        <FilterRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct FilterRequestBuilder {
    field: Option<String>,
    operator: Option<String>,
    value: Option<String>,
}

impl FilterRequestBuilder {
    pub fn field(mut self, value: impl Into<String>) -> Self {
        self.field = Some(value.into());
        self
    }

    pub fn operator(mut self, value: impl Into<String>) -> Self {
        self.operator = Some(value.into());
        self
    }

    pub fn value(mut self, value: impl Into<String>) -> Self {
        self.value = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`FilterRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`field`](FilterRequestBuilder::field)
    /// - [`operator`](FilterRequestBuilder::operator)
    /// - [`value`](FilterRequestBuilder::value)
    pub fn build(self) -> Result<FilterRequest, BuildError> {
        Ok(FilterRequest {
            field: self
                .field
                .ok_or_else(|| BuildError::missing_field("field"))?,
            operator: self
                .operator
                .ok_or_else(|| BuildError::missing_field("operator"))?,
            value: self
                .value
                .ok_or_else(|| BuildError::missing_field("value"))?,
        })
    }
}
