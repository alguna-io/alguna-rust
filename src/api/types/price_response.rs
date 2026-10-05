pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PriceResponse {
    /// When the price is billed
    pub billing_direction: BillingDirection,
    /// How often the price is billed
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_frequency: Option<BillingFrequency>,
    /// Billing interval
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_interval: Option<BillingInterval>,
    /// Positive count of BillingIntervalUnits per period
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_interval_count: Option<i64>,
    /// Unit of the billing cadence
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_interval_unit: Option<BillingIntervalUnit>,
    /// Whether to charge on contract start date
    pub charge_on_contract_start: bool,
    /// Timestamp when the price was created
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub created_at: DateTime<FixedOffset>,
    /// Currency code
    pub currency: String,
    /// Discount applied to this price
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discount: Option<Discount>,
    /// Display order
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_order: Option<i64>,
    /// Expression-based pricing details
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expression_pricing_model: Option<ExpressionPricingModel>,
    /// Whether the price is fixed or metered
    pub fee_type: PricingClass,
    /// Fixed pricing details
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fixed_pricing_model: Option<FixedPricingModel>,
    /// Graduated percentage pricing details
    #[serde(skip_serializing_if = "Option::is_none")]
    pub graduated_percentage_pricing_model: Option<GraduatedPercentagePricingModel>,
    /// Graduated tiered pricing details
    #[serde(skip_serializing_if = "Option::is_none")]
    pub graduated_tiered_pricing_model: Option<GraduatedTieredPricingModel>,
    /// Unique identifier for the price
    pub id: String,
    /// Matrix pricing details (when type=matrix)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub matrix_pricing_model: Option<MatrixPricingModel>,
    /// Maximum spend cap
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum_spend: Option<SpendingThreshold>,
    /// Associated metric identifiers
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metric_ids: Option<Vec<String>>,
    /// Minimum spend threshold
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimum_spend: Option<SpendingThreshold>,
    /// Prepaid fixed tiered pricing details
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prepaid_fixed_tiered_pricing_model: Option<PrepaidFixedTieredPricingModel>,
    /// Prepaid tiered pricing details
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prepaid_tiered_pricing_model: Option<PrepaidTieredPricingModel>,
    /// The product this price is for
    pub product: PriceProductResponse,
    /// Identifier of the associated product
    pub product_id: String,
    /// Tiered percentage pricing details
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tiered_percentage_pricing_model: Option<TieredPercentagePricingModel>,
    /// Tiered pricing details
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tiered_pricing_model: Option<TieredPricingModel>,
    /// Trial period in days
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trial_period_days: Option<i64>,
    /// The pricing model type
    pub r#type: PricingType,
    /// Unit pricing details
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_pricing_model: Option<UnitPricingModel>,
    /// Timestamp when the price was last updated
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub updated_at: DateTime<FixedOffset>,
    /// Volume percentage pricing details
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume_percentage_pricing_model: Option<VolumePercentagePricingModel>,
}

impl PriceResponse {
    pub fn builder() -> PriceResponseBuilder {
        <PriceResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PriceResponseBuilder {
    billing_direction: Option<BillingDirection>,
    billing_frequency: Option<BillingFrequency>,
    billing_interval: Option<BillingInterval>,
    billing_interval_count: Option<i64>,
    billing_interval_unit: Option<BillingIntervalUnit>,
    charge_on_contract_start: Option<bool>,
    created_at: Option<DateTime<FixedOffset>>,
    currency: Option<String>,
    discount: Option<Discount>,
    display_order: Option<i64>,
    expression_pricing_model: Option<ExpressionPricingModel>,
    fee_type: Option<PricingClass>,
    fixed_pricing_model: Option<FixedPricingModel>,
    graduated_percentage_pricing_model: Option<GraduatedPercentagePricingModel>,
    graduated_tiered_pricing_model: Option<GraduatedTieredPricingModel>,
    id: Option<String>,
    matrix_pricing_model: Option<MatrixPricingModel>,
    maximum_spend: Option<SpendingThreshold>,
    metric_ids: Option<Vec<String>>,
    minimum_spend: Option<SpendingThreshold>,
    prepaid_fixed_tiered_pricing_model: Option<PrepaidFixedTieredPricingModel>,
    prepaid_tiered_pricing_model: Option<PrepaidTieredPricingModel>,
    product: Option<PriceProductResponse>,
    product_id: Option<String>,
    tiered_percentage_pricing_model: Option<TieredPercentagePricingModel>,
    tiered_pricing_model: Option<TieredPricingModel>,
    trial_period_days: Option<i64>,
    r#type: Option<PricingType>,
    unit_pricing_model: Option<UnitPricingModel>,
    updated_at: Option<DateTime<FixedOffset>>,
    volume_percentage_pricing_model: Option<VolumePercentagePricingModel>,
}

impl PriceResponseBuilder {
    pub fn billing_direction(mut self, value: BillingDirection) -> Self {
        self.billing_direction = Some(value);
        self
    }

    pub fn billing_frequency(mut self, value: BillingFrequency) -> Self {
        self.billing_frequency = Some(value);
        self
    }

    pub fn billing_interval(mut self, value: BillingInterval) -> Self {
        self.billing_interval = Some(value);
        self
    }

    pub fn billing_interval_count(mut self, value: i64) -> Self {
        self.billing_interval_count = Some(value);
        self
    }

    pub fn billing_interval_unit(mut self, value: BillingIntervalUnit) -> Self {
        self.billing_interval_unit = Some(value);
        self
    }

    pub fn charge_on_contract_start(mut self, value: bool) -> Self {
        self.charge_on_contract_start = Some(value);
        self
    }

    pub fn created_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.created_at = Some(value);
        self
    }

    pub fn currency(mut self, value: impl Into<String>) -> Self {
        self.currency = Some(value.into());
        self
    }

    pub fn discount(mut self, value: Discount) -> Self {
        self.discount = Some(value);
        self
    }

    pub fn display_order(mut self, value: i64) -> Self {
        self.display_order = Some(value);
        self
    }

    pub fn expression_pricing_model(mut self, value: ExpressionPricingModel) -> Self {
        self.expression_pricing_model = Some(value);
        self
    }

    pub fn fee_type(mut self, value: PricingClass) -> Self {
        self.fee_type = Some(value);
        self
    }

    pub fn fixed_pricing_model(mut self, value: FixedPricingModel) -> Self {
        self.fixed_pricing_model = Some(value);
        self
    }

    pub fn graduated_percentage_pricing_model(
        mut self,
        value: GraduatedPercentagePricingModel,
    ) -> Self {
        self.graduated_percentage_pricing_model = Some(value);
        self
    }

    pub fn graduated_tiered_pricing_model(mut self, value: GraduatedTieredPricingModel) -> Self {
        self.graduated_tiered_pricing_model = Some(value);
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn matrix_pricing_model(mut self, value: MatrixPricingModel) -> Self {
        self.matrix_pricing_model = Some(value);
        self
    }

    pub fn maximum_spend(mut self, value: SpendingThreshold) -> Self {
        self.maximum_spend = Some(value);
        self
    }

    pub fn metric_ids(mut self, value: Vec<String>) -> Self {
        self.metric_ids = Some(value);
        self
    }

    pub fn minimum_spend(mut self, value: SpendingThreshold) -> Self {
        self.minimum_spend = Some(value);
        self
    }

    pub fn prepaid_fixed_tiered_pricing_model(
        mut self,
        value: PrepaidFixedTieredPricingModel,
    ) -> Self {
        self.prepaid_fixed_tiered_pricing_model = Some(value);
        self
    }

    pub fn prepaid_tiered_pricing_model(mut self, value: PrepaidTieredPricingModel) -> Self {
        self.prepaid_tiered_pricing_model = Some(value);
        self
    }

    pub fn product(mut self, value: PriceProductResponse) -> Self {
        self.product = Some(value);
        self
    }

    pub fn product_id(mut self, value: impl Into<String>) -> Self {
        self.product_id = Some(value.into());
        self
    }

    pub fn tiered_percentage_pricing_model(mut self, value: TieredPercentagePricingModel) -> Self {
        self.tiered_percentage_pricing_model = Some(value);
        self
    }

    pub fn tiered_pricing_model(mut self, value: TieredPricingModel) -> Self {
        self.tiered_pricing_model = Some(value);
        self
    }

    pub fn trial_period_days(mut self, value: i64) -> Self {
        self.trial_period_days = Some(value);
        self
    }

    pub fn r#type(mut self, value: PricingType) -> Self {
        self.r#type = Some(value);
        self
    }

    pub fn unit_pricing_model(mut self, value: UnitPricingModel) -> Self {
        self.unit_pricing_model = Some(value);
        self
    }

    pub fn updated_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.updated_at = Some(value);
        self
    }

    pub fn volume_percentage_pricing_model(mut self, value: VolumePercentagePricingModel) -> Self {
        self.volume_percentage_pricing_model = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PriceResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`billing_direction`](PriceResponseBuilder::billing_direction)
    /// - [`charge_on_contract_start`](PriceResponseBuilder::charge_on_contract_start)
    /// - [`created_at`](PriceResponseBuilder::created_at)
    /// - [`currency`](PriceResponseBuilder::currency)
    /// - [`fee_type`](PriceResponseBuilder::fee_type)
    /// - [`id`](PriceResponseBuilder::id)
    /// - [`product`](PriceResponseBuilder::product)
    /// - [`product_id`](PriceResponseBuilder::product_id)
    /// - [`r#type`](PriceResponseBuilder::r#type)
    /// - [`updated_at`](PriceResponseBuilder::updated_at)
    pub fn build(self) -> Result<PriceResponse, BuildError> {
        Ok(PriceResponse {
            billing_direction: self
                .billing_direction
                .ok_or_else(|| BuildError::missing_field("billing_direction"))?,
            billing_frequency: self.billing_frequency,
            billing_interval: self.billing_interval,
            billing_interval_count: self.billing_interval_count,
            billing_interval_unit: self.billing_interval_unit,
            charge_on_contract_start: self
                .charge_on_contract_start
                .ok_or_else(|| BuildError::missing_field("charge_on_contract_start"))?,
            created_at: self
                .created_at
                .ok_or_else(|| BuildError::missing_field("created_at"))?,
            currency: self
                .currency
                .ok_or_else(|| BuildError::missing_field("currency"))?,
            discount: self.discount,
            display_order: self.display_order,
            expression_pricing_model: self.expression_pricing_model,
            fee_type: self
                .fee_type
                .ok_or_else(|| BuildError::missing_field("fee_type"))?,
            fixed_pricing_model: self.fixed_pricing_model,
            graduated_percentage_pricing_model: self.graduated_percentage_pricing_model,
            graduated_tiered_pricing_model: self.graduated_tiered_pricing_model,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            matrix_pricing_model: self.matrix_pricing_model,
            maximum_spend: self.maximum_spend,
            metric_ids: self.metric_ids,
            minimum_spend: self.minimum_spend,
            prepaid_fixed_tiered_pricing_model: self.prepaid_fixed_tiered_pricing_model,
            prepaid_tiered_pricing_model: self.prepaid_tiered_pricing_model,
            product: self
                .product
                .ok_or_else(|| BuildError::missing_field("product"))?,
            product_id: self
                .product_id
                .ok_or_else(|| BuildError::missing_field("product_id"))?,
            tiered_percentage_pricing_model: self.tiered_percentage_pricing_model,
            tiered_pricing_model: self.tiered_pricing_model,
            trial_period_days: self.trial_period_days,
            r#type: self
                .r#type
                .ok_or_else(|| BuildError::missing_field("r#type"))?,
            unit_pricing_model: self.unit_pricing_model,
            updated_at: self
                .updated_at
                .ok_or_else(|| BuildError::missing_field("updated_at"))?,
            volume_percentage_pricing_model: self.volume_percentage_pricing_model,
        })
    }
}
