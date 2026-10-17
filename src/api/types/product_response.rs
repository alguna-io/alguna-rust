pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct ProductResponse {
    /// How often the product is billed
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_frequency: Option<BillingFrequency>,
    /// Timestamp when the product was created
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub created_at: DateTime<FixedOffset>,
    /// A brief description of the product
    pub description: String,
    /// Whether the product is fixed-price or usage-based
    pub fee_type: PricingClass,
    /// Unique identifier for the product
    pub id: String,
    /// Identifiers for associated metrics. Applicable for metered products
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metric_ids: Option<Vec<String>>,
    /// The name of the product
    pub name: String,
    /// When the product is billed relative to the billing period
    pub payment_terms: BillingDirection,
    /// How revenue is allocated across time periods within a billing cycle
    pub revenue_allocation_method: RevenueAllocationMethod,
    /// Stock keeping unit code used in external ERP or revenue recognition systems
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sku: Option<String>,
    /// Identifiers for tags associated with this product
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_ids: Option<Vec<String>>,
    /// Canonical (provider-neutral) tax code; omitted behaves as 'general'
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_code: Option<String>,
    /// Custom plural term for multiple units of the product (e.g. 'API calls'). Falls back to 'units' when omitted
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_label_plural: Option<String>,
    /// Custom singular term for one unit of the product (e.g. 'API call'). Falls back to 'unit' when omitted
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_label_singular: Option<String>,
    /// Timestamp when the product was last updated
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub updated_at: DateTime<FixedOffset>,
}

impl ProductResponse {
    pub fn builder() -> ProductResponseBuilder {
        <ProductResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ProductResponseBuilder {
    billing_frequency: Option<BillingFrequency>,
    created_at: Option<DateTime<FixedOffset>>,
    description: Option<String>,
    fee_type: Option<PricingClass>,
    id: Option<String>,
    metric_ids: Option<Vec<String>>,
    name: Option<String>,
    payment_terms: Option<BillingDirection>,
    revenue_allocation_method: Option<RevenueAllocationMethod>,
    sku: Option<String>,
    tag_ids: Option<Vec<String>>,
    tax_code: Option<String>,
    unit_label_plural: Option<String>,
    unit_label_singular: Option<String>,
    updated_at: Option<DateTime<FixedOffset>>,
}

impl ProductResponseBuilder {
    pub fn billing_frequency(mut self, value: BillingFrequency) -> Self {
        self.billing_frequency = Some(value);
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

    pub fn updated_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.updated_at = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ProductResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`created_at`](ProductResponseBuilder::created_at)
    /// - [`description`](ProductResponseBuilder::description)
    /// - [`fee_type`](ProductResponseBuilder::fee_type)
    /// - [`id`](ProductResponseBuilder::id)
    /// - [`name`](ProductResponseBuilder::name)
    /// - [`payment_terms`](ProductResponseBuilder::payment_terms)
    /// - [`revenue_allocation_method`](ProductResponseBuilder::revenue_allocation_method)
    /// - [`updated_at`](ProductResponseBuilder::updated_at)
    pub fn build(self) -> Result<ProductResponse, BuildError> {
        Ok(ProductResponse {
            billing_frequency: self.billing_frequency,
            created_at: self
                .created_at
                .ok_or_else(|| BuildError::missing_field("created_at"))?,
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
            revenue_allocation_method: self
                .revenue_allocation_method
                .ok_or_else(|| BuildError::missing_field("revenue_allocation_method"))?,
            sku: self.sku,
            tag_ids: self.tag_ids,
            tax_code: self.tax_code,
            unit_label_plural: self.unit_label_plural,
            unit_label_singular: self.unit_label_singular,
            updated_at: self
                .updated_at
                .ok_or_else(|| BuildError::missing_field("updated_at"))?,
        })
    }
}
