pub use crate::prelude::*;

/// Query parameters for list
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ProductsListQueryRequest {
    /// Filter by fee type (comma-separated to match any)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fee_type: Option<Vec<ListProductsRequestFeeTypeItem>>,
    /// Number of items to return per page
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// Filter by associated metric IDs (comma-separated to match any)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metric_ids: Option<Vec<String>>,
    /// Number of items to skip
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<i64>,
    /// Filter by payment terms (comma-separated to match any)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_terms: Option<Vec<ListProductsRequestPaymentTermsItem>>,
}

impl ProductsListQueryRequest {
    pub fn builder() -> ProductsListQueryRequestBuilder {
        <ProductsListQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ProductsListQueryRequestBuilder {
    fee_type: Option<Vec<ListProductsRequestFeeTypeItem>>,
    limit: Option<i64>,
    metric_ids: Option<Vec<String>>,
    offset: Option<i64>,
    payment_terms: Option<Vec<ListProductsRequestPaymentTermsItem>>,
}

impl ProductsListQueryRequestBuilder {
    pub fn fee_type(mut self, value: Vec<ListProductsRequestFeeTypeItem>) -> Self {
        self.fee_type = Some(value);
        self
    }

    pub fn limit(mut self, value: i64) -> Self {
        self.limit = Some(value);
        self
    }

    pub fn metric_ids(mut self, value: Vec<String>) -> Self {
        self.metric_ids = Some(value);
        self
    }

    pub fn offset(mut self, value: i64) -> Self {
        self.offset = Some(value);
        self
    }

    pub fn payment_terms(mut self, value: Vec<ListProductsRequestPaymentTermsItem>) -> Self {
        self.payment_terms = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ProductsListQueryRequest`].
    pub fn build(self) -> Result<ProductsListQueryRequest, BuildError> {
        Ok(ProductsListQueryRequest {
            fee_type: self.fee_type,
            limit: self.limit,
            metric_ids: self.metric_ids,
            offset: self.offset,
            payment_terms: self.payment_terms,
        })
    }
}
