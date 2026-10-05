pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreateTaxCalculationRequest {
    /// ISO 4217 currency code
    pub currency: String,
    /// Who is being taxed
    pub customer: TaxCalculationCustomerRequest,
    /// Selling legal entity; defaults to the org's default when omitted
    #[serde(skip_serializing_if = "Option::is_none")]
    pub legal_entity_id: Option<String>,
    /// Line items to tax
    pub line_items: Vec<TaxCalculationLineItemRequest>,
    /// Existing sub-merchant the receipt is issued for; you remain the seller of record. Create the merchant before referencing it here. Requires the merchants entitlement
    #[serde(skip_serializing_if = "Option::is_none")]
    pub merchant_id: Option<String>,
    /// Whether line amounts exclude (default) or already include tax
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_behavior: Option<TaxBehavior>,
}

impl CreateTaxCalculationRequest {
    pub fn builder() -> CreateTaxCalculationRequestBuilder {
        <CreateTaxCalculationRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateTaxCalculationRequestBuilder {
    currency: Option<String>,
    customer: Option<TaxCalculationCustomerRequest>,
    legal_entity_id: Option<String>,
    line_items: Option<Vec<TaxCalculationLineItemRequest>>,
    merchant_id: Option<String>,
    tax_behavior: Option<TaxBehavior>,
}

impl CreateTaxCalculationRequestBuilder {
    pub fn currency(mut self, value: impl Into<String>) -> Self {
        self.currency = Some(value.into());
        self
    }

    pub fn customer(mut self, value: TaxCalculationCustomerRequest) -> Self {
        self.customer = Some(value);
        self
    }

    pub fn legal_entity_id(mut self, value: impl Into<String>) -> Self {
        self.legal_entity_id = Some(value.into());
        self
    }

    pub fn line_items(mut self, value: Vec<TaxCalculationLineItemRequest>) -> Self {
        self.line_items = Some(value);
        self
    }

    pub fn merchant_id(mut self, value: impl Into<String>) -> Self {
        self.merchant_id = Some(value.into());
        self
    }

    pub fn tax_behavior(mut self, value: TaxBehavior) -> Self {
        self.tax_behavior = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CreateTaxCalculationRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`currency`](CreateTaxCalculationRequestBuilder::currency)
    /// - [`customer`](CreateTaxCalculationRequestBuilder::customer)
    /// - [`line_items`](CreateTaxCalculationRequestBuilder::line_items)
    pub fn build(self) -> Result<CreateTaxCalculationRequest, BuildError> {
        Ok(CreateTaxCalculationRequest {
            currency: self
                .currency
                .ok_or_else(|| BuildError::missing_field("currency"))?,
            customer: self
                .customer
                .ok_or_else(|| BuildError::missing_field("customer"))?,
            legal_entity_id: self.legal_entity_id,
            line_items: self
                .line_items
                .ok_or_else(|| BuildError::missing_field("line_items"))?,
            merchant_id: self.merchant_id,
            tax_behavior: self.tax_behavior,
        })
    }
}
