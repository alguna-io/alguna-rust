pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct CreateProductRequest {
    /// How often the product is billed. Required for fixed products; metered products are always recurring
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_frequency: Option<BillingFrequency>,
    /// A brief description of the product
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Whether the product is fixed-price or usage-based
    pub fee_type: PricingClass,
    /// Identifiers for metrics to associate with a metered product
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metric_ids: Option<Vec<String>>,
    /// The name of the product
    pub name: String,
    /// When the product is billed relative to the billing period. Metered products must be billed in arrears
    pub payment_terms: BillingDirection,
    /// How revenue is allocated across time periods within a billing cycle
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revenue_allocation_method: Option<RevenueAllocationMethod>,
    /// Stock keeping unit code used in external ERP or revenue recognition systems
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sku: Option<String>,
    /// Identifiers for tags to associate with this product
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_ids: Option<Vec<String>>,
    /// Canonical (provider-neutral) tax code. Omit for 'general' (standard-rated).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_code: Option<String>,
    /// Custom plural term for multiple units of the product (e.g. 'API calls'). Falls back to 'units' when omitted
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_label_plural: Option<String>,
    /// Custom singular term for one unit of the product (e.g. 'API call'). Falls back to 'unit' when omitted
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_label_singular: Option<String>,
}

impl CreateProductRequest {
    pub fn builder() -> CreateProductRequestBuilder {
        <CreateProductRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateProductRequestBuilder {
    billing_frequency: Option<BillingFrequency>,
    description: Option<String>,
    fee_type: Option<PricingClass>,
    metric_ids: Option<Vec<String>>,
    name: Option<String>,
    payment_terms: Option<BillingDirection>,
    revenue_allocation_method: Option<RevenueAllocationMethod>,
    sku: Option<String>,
    tag_ids: Option<Vec<String>>,
    tax_code: Option<String>,
    unit_label_plural: Option<String>,
    unit_label_singular: Option<String>,
}

impl CreateProductRequestBuilder {
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

    pub fn revenue_allocation_method(mut self, value: RevenueAllocationMethod) -> Self {
        self.revenue_allocation_method = Some(value);
        self
    }

    pub fn sku(mut self, value: impl Into<String>) -> Self {
        self.sku = Some(value.into());
        self
    }

    pub fn tag_ids(mut self, value: Vec<String>) -> Self {
        self.tag_ids = Some(value);
        self
    }

    pub fn tax_code(mut self, value: impl Into<String>) -> Self {
        self.tax_code = Some(value.into());
        self
    }

    pub fn unit_label_plural(mut self, value: impl Into<String>) -> Self {
        self.unit_label_plural = Some(value.into());
        self
    }

    pub fn unit_label_singular(mut self, value: impl Into<String>) -> Self {
        self.unit_label_singular = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CreateProductRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`fee_type`](CreateProductRequestBuilder::fee_type)
    /// - [`name`](CreateProductRequestBuilder::name)
    /// - [`payment_terms`](CreateProductRequestBuilder::payment_terms)
    pub fn build(self) -> Result<CreateProductRequest, BuildError> {
        Ok(CreateProductRequest {
            billing_frequency: self.billing_frequency,
            description: self.description,
            fee_type: self
                .fee_type
                .ok_or_else(|| BuildError::missing_field("fee_type"))?,
            metric_ids: self.metric_ids,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            payment_terms: self
                .payment_terms
                .ok_or_else(|| BuildError::missing_field("payment_terms"))?,
            revenue_allocation_method: self.revenue_allocation_method,
            sku: self.sku,
            tag_ids: self.tag_ids,
            tax_code: self.tax_code,
            unit_label_plural: self.unit_label_plural,
            unit_label_singular: self.unit_label_singular,
        })
    }
}
