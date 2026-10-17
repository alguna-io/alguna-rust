pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct FilterResponse {
    /// Event property field name
    pub field: String,
    /// Filter comparison operator
    pub operator: String,
    /// Value to match against
    pub value: String,
}

impl FilterResponse {
    pub fn builder() -> FilterResponseBuilder {
        <FilterResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct FilterResponseBuilder {
    field: Option<String>,
    operator: Option<String>,
    value: Option<String>,
}

impl FilterResponseBuilder {
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

    /// Consumes the builder and constructs a [`FilterResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`field`](FilterResponseBuilder::field)
    /// - [`operator`](FilterResponseBuilder::operator)
    /// - [`value`](FilterResponseBuilder::value)
    pub fn build(self) -> Result<FilterResponse, BuildError> {
        Ok(FilterResponse {
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
