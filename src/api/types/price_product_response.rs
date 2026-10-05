pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct PriceProductResponse {
    /// Billing frequency
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_frequency: Option<BillingFrequency>,
    /// Product description
    pub description: String,
    /// Product fee type
    pub fee_type: PricingClass,
    /// Product identifier
    pub id: String,
    /// Associated metric identifiers
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metric_ids: Option<Vec<String>>,
    /// Product name
    pub name: String,
    /// Payment terms
    pub payment_terms: BillingDirection,
}

impl PriceProductResponse {
    pub fn builder() -> PriceProductResponseBuilder {
        <PriceProductResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PriceProductResponseBuilder {
    billing_frequency: Option<BillingFrequency>,
    description: Option<String>,
    fee_type: Option<PricingClass>,
    id: Option<String>,
    metric_ids: Option<Vec<String>>,
    name: Option<String>,
    payment_terms: Option<BillingDirection>,
}

impl PriceProductResponseBuilder {
    pub fn billing_frequency(mut self, value: BillingFrequency) -> Self {
        self.billing_frequency = Some(value);
        self
    }

    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
        self
    }

    pub fn fee_type(mut self, value: PricingClass) -> Self {
        self.fee_type = Some(value);
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn metric_ids(mut self, value: Vec<String>) -> Self {
        self.metric_ids = Some(value);
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn payment_terms(mut self, value: BillingDirection) -> Self {
        self.payment_terms = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PriceProductResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`description`](PriceProductResponseBuilder::description)
    /// - [`fee_type`](PriceProductResponseBuilder::fee_type)
    /// - [`id`](PriceProductResponseBuilder::id)
    /// - [`name`](PriceProductResponseBuilder::name)
    /// - [`payment_terms`](PriceProductResponseBuilder::payment_terms)
    pub fn build(self) -> Result<PriceProductResponse, BuildError> {
        Ok(PriceProductResponse {
            billing_frequency: self.billing_frequency,
            description: self
                .description
                .ok_or_else(|| BuildError::missing_field("description"))?,
            fee_type: self
                .fee_type
                .ok_or_else(|| BuildError::missing_field("fee_type"))?,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            metric_ids: self.metric_ids,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            payment_terms: self
                .payment_terms
                .ok_or_else(|| BuildError::missing_field("payment_terms"))?,
        })
    }
}
