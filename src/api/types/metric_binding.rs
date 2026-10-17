pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct MetricBinding {
    /// Alias referenced in charge expressions
    pub alias: String,
    /// Metric ID bound to the alias
    pub metric_id: String,
}

impl MetricBinding {
    pub fn builder() -> MetricBindingBuilder {
        <MetricBindingBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct MetricBindingBuilder {
    alias: Option<String>,
    metric_id: Option<String>,
}

impl MetricBindingBuilder {
    pub fn alias(mut self, value: impl Into<String>) -> Self {
        self.alias = Some(value.into());
        self
    }

    pub fn metric_id(mut self, value: impl Into<String>) -> Self {
        self.metric_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`MetricBinding`].
    /// This method will fail if any of the following fields are not set:
    /// - [`alias`](MetricBindingBuilder::alias)
    /// - [`metric_id`](MetricBindingBuilder::metric_id)
    pub fn build(self) -> Result<MetricBinding, BuildError> {
        Ok(MetricBinding {
            alias: self
                .alias
                .ok_or_else(|| BuildError::missing_field("alias"))?,
            metric_id: self
                .metric_id
                .ok_or_else(|| BuildError::missing_field("metric_id"))?,
        })
    }
}
