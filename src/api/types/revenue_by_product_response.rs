pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct RevenueByProductResponse {
    /// ISO 4217 currency code
    pub currency: String,
    /// Products ordered by revenue
    pub products: Vec<InsightsProductEntry>,
}

impl RevenueByProductResponse {
    pub fn builder() -> RevenueByProductResponseBuilder {
        <RevenueByProductResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RevenueByProductResponseBuilder {
    currency: Option<String>,
    products: Option<Vec<InsightsProductEntry>>,
}

impl RevenueByProductResponseBuilder {
    pub fn currency(mut self, value: impl Into<String>) -> Self {
        self.currency = Some(value.into());
        self
    }

    pub fn products(mut self, value: Vec<InsightsProductEntry>) -> Self {
        self.products = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`RevenueByProductResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`currency`](RevenueByProductResponseBuilder::currency)
    /// - [`products`](RevenueByProductResponseBuilder::products)
    pub fn build(self) -> Result<RevenueByProductResponse, BuildError> {
        Ok(RevenueByProductResponse {
            currency: self
                .currency
                .ok_or_else(|| BuildError::missing_field("currency"))?,
            products: self
                .products
                .ok_or_else(|| BuildError::missing_field("products"))?,
        })
    }
}
