pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct CustomerInsightsOverTimeResponse {
    /// Chronologically ordered data points
    pub points: Vec<CustomerInsightOverTimePoint>,
    /// Time resolution of data points
    pub resolution: CustomerInsightsOverTimeResponseResolution,
}

impl CustomerInsightsOverTimeResponse {
    pub fn builder() -> CustomerInsightsOverTimeResponseBuilder {
        <CustomerInsightsOverTimeResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CustomerInsightsOverTimeResponseBuilder {
    points: Option<Vec<CustomerInsightOverTimePoint>>,
    resolution: Option<CustomerInsightsOverTimeResponseResolution>,
}

impl CustomerInsightsOverTimeResponseBuilder {
    pub fn points(mut self, value: Vec<CustomerInsightOverTimePoint>) -> Self {
        self.points = Some(value);
        self
    }

    pub fn resolution(mut self, value: CustomerInsightsOverTimeResponseResolution) -> Self {
        self.resolution = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CustomerInsightsOverTimeResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`points`](CustomerInsightsOverTimeResponseBuilder::points)
    /// - [`resolution`](CustomerInsightsOverTimeResponseBuilder::resolution)
    pub fn build(self) -> Result<CustomerInsightsOverTimeResponse, BuildError> {
        Ok(CustomerInsightsOverTimeResponse {
            points: self
                .points
                .ok_or_else(|| BuildError::missing_field("points"))?,
            resolution: self
                .resolution
                .ok_or_else(|| BuildError::missing_field("resolution"))?,
        })
    }
}
