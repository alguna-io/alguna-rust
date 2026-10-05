pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct MetricResponse {
    /// How events are aggregated into a metric value
    pub aggregation: AggregationResponse,
    /// Timestamp when the metric was created
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub created_at: DateTime<FixedOffset>,
    /// Description of the metric
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Event name that triggers this metric
    pub event_name: String,
    /// Filter groups applied to events before aggregation
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter_groups: Option<Vec<FilterGroupResponse>>,
    /// Unique identifier for the metric
    pub id: String,
    /// Name of the metric
    pub name: String,
    /// Tag identifiers associated with this metric
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_ids: Option<Vec<String>>,
    /// Timestamp when the metric was last updated
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub updated_at: DateTime<FixedOffset>,
}

impl MetricResponse {
    pub fn builder() -> MetricResponseBuilder {
        <MetricResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct MetricResponseBuilder {
    aggregation: Option<AggregationResponse>,
    created_at: Option<DateTime<FixedOffset>>,
    description: Option<String>,
    event_name: Option<String>,
    filter_groups: Option<Vec<FilterGroupResponse>>,
    id: Option<String>,
    name: Option<String>,
    tag_ids: Option<Vec<String>>,
    updated_at: Option<DateTime<FixedOffset>>,
}

impl MetricResponseBuilder {
    pub fn aggregation(mut self, value: AggregationResponse) -> Self {
        self.aggregation = Some(value);
        self
    }

    pub fn created_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.created_at = Some(value);
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

    pub fn filter_groups(mut self, value: Vec<FilterGroupResponse>) -> Self {
        self.filter_groups = Some(value);
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
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

    pub fn updated_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.updated_at = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`MetricResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`aggregation`](MetricResponseBuilder::aggregation)
    /// - [`created_at`](MetricResponseBuilder::created_at)
    /// - [`event_name`](MetricResponseBuilder::event_name)
    /// - [`id`](MetricResponseBuilder::id)
    /// - [`name`](MetricResponseBuilder::name)
    /// - [`updated_at`](MetricResponseBuilder::updated_at)
    pub fn build(self) -> Result<MetricResponse, BuildError> {
        Ok(MetricResponse {
            aggregation: self
                .aggregation
                .ok_or_else(|| BuildError::missing_field("aggregation"))?,
            created_at: self
                .created_at
                .ok_or_else(|| BuildError::missing_field("created_at"))?,
            description: self.description,
            event_name: self
                .event_name
                .ok_or_else(|| BuildError::missing_field("event_name"))?,
            filter_groups: self.filter_groups,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            tag_ids: self.tag_ids,
            updated_at: self
                .updated_at
                .ok_or_else(|| BuildError::missing_field("updated_at"))?,
        })
    }
}
