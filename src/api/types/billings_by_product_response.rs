pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct BillingsByProductResponse {
    /// ISO 4217 currency code
    pub currency: String,
    /// Products ordered by billings
    pub products: Vec<InsightsProductEntry>,
}

impl BillingsByProductResponse {
    pub fn builder() -> BillingsByProductResponseBuilder {
        <BillingsByProductResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BillingsByProductResponseBuilder {
    currency: Option<String>,
    products: Option<Vec<InsightsProductEntry>>,
}

impl BillingsByProductResponseBuilder {
    pub fn currency(mut self, value: impl Into<String>) -> Self {
        self.currency = Some(value.into());
        self
    }

    pub fn products(mut self, value: Vec<InsightsProductEntry>) -> Self {
        self.products = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`BillingsByProductResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`currency`](BillingsByProductResponseBuilder::currency)
    /// - [`products`](BillingsByProductResponseBuilder::products)
    pub fn build(self) -> Result<BillingsByProductResponse, BuildError> {
        Ok(BillingsByProductResponse {
            currency: self
                .currency
                .ok_or_else(|| BuildError::missing_field("currency"))?,
            products: self
                .products
                .ok_or_else(|| BuildError::missing_field("products"))?,
        })
    }
}
