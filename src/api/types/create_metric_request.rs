pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct CreateMetricRequest {
    /// How events are aggregated into a metric value
    pub aggregation: AggregationRequest,
    /// Description of the metric
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Event name that triggers this metric
    pub event_name: String,
    /// Filter groups to apply to events before aggregation
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter_groups: Option<Vec<FilterGroupRequest>>,
    /// Name of the metric
    pub name: String,
    /// Tag identifiers to associate with this metric
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_ids: Option<Vec<String>>,
}

impl CreateMetricRequest {
    pub fn builder() -> CreateMetricRequestBuilder {
        <CreateMetricRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateMetricRequestBuilder {
    aggregation: Option<AggregationRequest>,
    description: Option<String>,
    event_name: Option<String>,
    filter_groups: Option<Vec<FilterGroupRequest>>,
    name: Option<String>,
    tag_ids: Option<Vec<String>>,
}

impl CreateMetricRequestBuilder {
    pub fn aggregation(mut self, value: AggregationRequest) -> Self {
        self.aggregation = Some(value);
        self
    }

    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
        self
    }

    pub fn event_name(mut self, value: impl Into<String>) -> Self {
        self.event_name = Some(value.into());
        self
    }

    pub fn filter_groups(mut self, value: Vec<FilterGroupRequest>) -> Self {
        self.filter_groups = Some(value);
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn tag_ids(mut self, value: Vec<String>) -> Self {
        self.tag_ids = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CreateMetricRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`aggregation`](CreateMetricRequestBuilder::aggregation)
    /// - [`event_name`](CreateMetricRequestBuilder::event_name)
    /// - [`name`](CreateMetricRequestBuilder::name)
    pub fn build(self) -> Result<CreateMetricRequest, BuildError> {
        Ok(CreateMetricRequest {
            aggregation: self
                .aggregation
                .ok_or_else(|| BuildError::missing_field("aggregation"))?,
            description: self.description,
            event_name: self
                .event_name
                .ok_or_else(|| BuildError::missing_field("event_name"))?,
            filter_groups: self.filter_groups,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            tag_ids: self.tag_ids,
        })
    }
}
