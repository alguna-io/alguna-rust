pub use crate::prelude::*;

/// Query parameters for lookup
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct LookupQueryRequest {
    /// Customer alias to resolve — an Alguna account ID, a customer alias, or a connected-account external ID (such as a Stripe customer ID)
    pub alias: String,
}

impl LookupQueryRequest {
    pub fn builder() -> LookupQueryRequestBuilder {
        <LookupQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct LookupQueryRequestBuilder {
    alias: Option<String>,
}

impl LookupQueryRequestBuilder {
    pub fn alias(mut self, value: impl Into<String>) -> Self {
        self.alias = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`LookupQueryRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`alias`](LookupQueryRequestBuilder::alias)
    pub fn build(self) -> Result<LookupQueryRequest, BuildError> {
        Ok(LookupQueryRequest {
            alias: self
                .alias
                .ok_or_else(|| BuildError::missing_field("alias"))?,
        })
    }
}
