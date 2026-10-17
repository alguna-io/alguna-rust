pub use crate::prelude::*;

/// Query parameters for list
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct MerchantsListQueryRequest {
    /// Return only merchants with these display names, comma-separated. Matched exactly, including case.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<Vec<String>>,
    /// Return only merchants with these external ids, comma-separated
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_id: Option<Vec<String>>,
    /// Include archived merchants, which are excluded by default
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_archived: Option<bool>,
    /// Merchants to return per page
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// Merchants to skip before the first returned
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<i64>,
    /// Sort field and direction in format field:order. Sortable fields: display_name, external_id, support_email, created_at
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort: Option<String>,
}

impl MerchantsListQueryRequest {
    pub fn builder() -> MerchantsListQueryRequestBuilder {
        <MerchantsListQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct MerchantsListQueryRequestBuilder {
    display_name: Option<Vec<String>>,
    external_id: Option<Vec<String>>,
    include_archived: Option<bool>,
    limit: Option<i64>,
    offset: Option<i64>,
    sort: Option<String>,
}

impl MerchantsListQueryRequestBuilder {
    pub fn display_name(mut self, value: Vec<String>) -> Self {
        self.display_name = Some(value);
        self
    }

    pub fn external_id(mut self, value: Vec<String>) -> Self {
        self.external_id = Some(value);
        self
    }

    pub fn include_archived(mut self, value: bool) -> Self {
        self.include_archived = Some(value);
        self
    }

    pub fn limit(mut self, value: i64) -> Self {
        self.limit = Some(value);
        self
    }

    pub fn offset(mut self, value: i64) -> Self {
        self.offset = Some(value);
        self
    }

    pub fn sort(mut self, value: impl Into<String>) -> Self {
        self.sort = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`MerchantsListQueryRequest`].
    pub fn build(self) -> Result<MerchantsListQueryRequest, BuildError> {
        Ok(MerchantsListQueryRequest {
            display_name: self.display_name,
            external_id: self.external_id,
            include_archived: self.include_archived,
            limit: self.limit,
            offset: self.offset,
            sort: self.sort,
        })
    }
}
