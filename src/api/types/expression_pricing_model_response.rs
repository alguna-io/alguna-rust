pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ExpressionPricingModelResponse {
    /// Expression-based charges
    pub charges: Vec<ExpressionChargeResponse>,
    /// Alias-to-metric bindings referenced in expressions
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metric_bindings: Option<Vec<MetricBinding>>,
}

impl ExpressionPricingModelResponse {
    pub fn builder() -> ExpressionPricingModelResponseBuilder {
        <ExpressionPricingModelResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ExpressionPricingModelResponseBuilder {
    charges: Option<Vec<ExpressionChargeResponse>>,
    metric_bindings: Option<Vec<MetricBinding>>,
}

impl ExpressionPricingModelResponseBuilder {
    pub fn charges(mut self, value: Vec<ExpressionChargeResponse>) -> Self {
        self.charges = Some(value);
        self
    }

    pub fn metric_bindings(mut self, value: Vec<MetricBinding>) -> Self {
        self.metric_bindings = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ExpressionPricingModelResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`charges`](ExpressionPricingModelResponseBuilder::charges)
    pub fn build(self) -> Result<ExpressionPricingModelResponse, BuildError> {
        Ok(ExpressionPricingModelResponse {
            charges: self
                .charges
                .ok_or_else(|| BuildError::missing_field("charges"))?,
            metric_bindings: self.metric_bindings,
        })
    }
}
