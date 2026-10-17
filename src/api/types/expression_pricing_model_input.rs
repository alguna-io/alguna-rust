pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ExpressionPricingModelInput {
    /// Expression-based charges
    pub charges: Vec<ExpressionChargeInput>,
    /// Alias-to-metric bindings referenced in expressions
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metric_bindings: Option<Vec<MetricBinding>>,
}

impl ExpressionPricingModelInput {
    pub fn builder() -> ExpressionPricingModelInputBuilder {
        <ExpressionPricingModelInputBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ExpressionPricingModelInputBuilder {
    charges: Option<Vec<ExpressionChargeInput>>,
    metric_bindings: Option<Vec<MetricBinding>>,
}

impl ExpressionPricingModelInputBuilder {
    pub fn charges(mut self, value: Vec<ExpressionChargeInput>) -> Self {
        self.charges = Some(value);
        self
    }

    pub fn metric_bindings(mut self, value: Vec<MetricBinding>) -> Self {
        self.metric_bindings = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ExpressionPricingModelInput`].
    /// This method will fail if any of the following fields are not set:
    /// - [`charges`](ExpressionPricingModelInputBuilder::charges)
    pub fn build(self) -> Result<ExpressionPricingModelInput, BuildError> {
        Ok(ExpressionPricingModelInput {
            charges: self
                .charges
                .ok_or_else(|| BuildError::missing_field("charges"))?,
            metric_bindings: self.metric_bindings,
        })
    }
}
