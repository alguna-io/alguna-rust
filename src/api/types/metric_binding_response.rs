pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct MetricBindingResponse {
    /// Alias referenced in charge expressions
    pub alias: String,
    /// Metric ID bound to the alias
    pub metric_id: String,
}

impl MetricBindingResponse {
    pub fn builder() -> MetricBindingResponseBuilder {
        <MetricBindingResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct MetricBindingResponseBuilder {
    alias: Option<String>,
    metric_id: Option<String>,
}

impl MetricBindingResponseBuilder {
    pub fn alias(mut self, value: impl Into<String>) -> Self {
        self.alias = Some(value.into());
        self
    }

    pub fn metric_id(mut self, value: impl Into<String>) -> Self {
        self.metric_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`MetricBindingResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`alias`](MetricBindingResponseBuilder::alias)
    /// - [`metric_id`](MetricBindingResponseBuilder::metric_id)
    pub fn build(self) -> Result<MetricBindingResponse, BuildError> {
        Ok(MetricBindingResponse {
            alias: self
                .alias
                .ok_or_else(|| BuildError::missing_field("alias"))?,
            metric_id: self
                .metric_id
                .ok_or_else(|| BuildError::missing_field("metric_id"))?,
        })
    }
}
