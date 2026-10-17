pub use crate::prelude::*;

/// Query parameters for list
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct MetricsListQueryRequest {
    /// Filter by aggregation field (comma-separated to match any)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aggregation_field: Option<Vec<String>>,
    /// Filter by aggregation method (comma-separated to match any)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aggregation_method: Option<Vec<ListMetricsRequestAggregationMethodItem>>,
    /// Filter by event name (comma-separated to match any)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_name: Option<Vec<String>>,
    /// Number of items to return per page
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// Number of items to skip
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<i64>,
    /// Sort field and direction in format field:order
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort: Option<String>,
    /// Filter by tag IDs (comma-separated to match any)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_ids: Option<Vec<String>>,
}

impl MetricsListQueryRequest {
    pub fn builder() -> MetricsListQueryRequestBuilder {
        <MetricsListQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct MetricsListQueryRequestBuilder {
    aggregation_field: Option<Vec<String>>,
    aggregation_method: Option<Vec<ListMetricsRequestAggregationMethodItem>>,
    event_name: Option<Vec<String>>,
    limit: Option<i64>,
    offset: Option<i64>,
    sort: Option<String>,
    tag_ids: Option<Vec<String>>,
}

impl MetricsListQueryRequestBuilder {
    pub fn aggregation_field(mut self, value: Vec<String>) -> Self {
        self.aggregation_field = Some(value);
        self
    }

    pub fn aggregation_method(
        mut self,
        value: Vec<ListMetricsRequestAggregationMethodItem>,
    ) -> Self {
        self.aggregation_method = Some(value);
        self
    }

    pub fn event_name(mut self, value: Vec<String>) -> Self {
        self.event_name = Some(value);
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

    pub fn tag_ids(mut self, value: Vec<String>) -> Self {
        self.tag_ids = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`MetricsListQueryRequest`].
    pub fn build(self) -> Result<MetricsListQueryRequest, BuildError> {
        Ok(MetricsListQueryRequest {
            aggregation_field: self.aggregation_field,
            aggregation_method: self.aggregation_method,
            event_name: self.event_name,
            limit: self.limit,
            offset: self.offset,
            sort: self.sort,
            tag_ids: self.tag_ids,
        })
    }
}
