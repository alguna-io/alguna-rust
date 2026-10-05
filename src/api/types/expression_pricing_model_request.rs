pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ExpressionPricingModelRequest {
    /// Expression-based charges
    pub charges: Vec<ExpressionCharge>,
    /// Alias-to-metric bindings referenced in expressions
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metric_bindings: Option<Vec<MetricBinding>>,
}

impl ExpressionPricingModelRequest {
    pub fn builder() -> ExpressionPricingModelRequestBuilder {
        <ExpressionPricingModelRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ExpressionPricingModelRequestBuilder {
    charges: Option<Vec<ExpressionCharge>>,
    metric_bindings: Option<Vec<MetricBinding>>,
}

impl ExpressionPricingModelRequestBuilder {
    pub fn charges(mut self, value: Vec<ExpressionCharge>) -> Self {
        self.charges = Some(value);
        self
    }

    pub fn metric_bindings(mut self, value: Vec<MetricBinding>) -> Self {
        self.metric_bindings = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ExpressionPricingModelRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`charges`](ExpressionPricingModelRequestBuilder::charges)
    pub fn build(self) -> Result<ExpressionPricingModelRequest, BuildError> {
        Ok(ExpressionPricingModelRequest {
            charges: self
                .charges
                .ok_or_else(|| BuildError::missing_field("charges"))?,
            metric_bindings: self.metric_bindings,
        })
    }
}
