use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use crate::{AsyncPaginator, PaginationResult};
use reqwest::Method;

pub struct ProductsClient {
    pub http_client: HttpClient,
}

impl ProductsClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Returns a paginated list of products. Supports filtering by fee type (`fixed` or `metered`), payment terms (`advance` or `arrears`), and associated metric IDs.
    ///
    /// # Arguments
    ///
    /// * `fee_type` - Filter by fee type (comma-separated to match any)
    /// * `limit` - Number of items to return per page
    /// * `metric_ids` - Filter by associated metric IDs (comma-separated to match any)
    /// * `offset` - Number of items to skip
    /// * `payment_terms` - Filter by payment terms (comma-separated to match any)
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use alguna::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         api_key: Some("<token>".to_string()),
    ///         ..Default::default()
    ///     };
    ///     let client = Alguna::new(config).expect("Failed to build client");
    ///     client
    ///         .products
    ///         .list(
    ///             &ProductsListQueryRequest {
    ///                 limit: Some(20),
    ///                 offset: Some(0),
    ///                 fee_type: vec![],
    ///                 metric_ids: vec![],
    ///                 payment_terms: vec![],
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn list(
        &self,
        request: &ProductsListQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<ProductListResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                "products",
                None,
                QueryBuilder::new()
                    .string("fee_type", {
                        let joined = request
                            .fee_type
                            .iter()
                            .flatten()
                            .map(|value| value.to_string())
                            .collect::<Vec<_>>()
                            .join(",");
                        if joined.is_empty() {
                            None
                        } else {
                            Some(joined)
                        }
                    })
                    .int("limit", request.limit.clone())
                    .string("metric_ids", {
                        let joined = request
                            .metric_ids
                            .iter()
                            .flatten()
                            .map(|value| value.to_string())
                            .collect::<Vec<_>>()
                            .join(",");
                        if joined.is_empty() {
                            None
                        } else {
                            Some(joined)
                        }
                    })
                    .int("offset", request.offset.clone())
                    .string("payment_terms", {
                        let joined = request
                            .payment_terms
                            .iter()
                            .flatten()
                            .map(|value| value.to_string())
                            .collect::<Vec<_>>()
                            .join(",");
                        if joined.is_empty() {
                            None
                        } else {
                            Some(joined)
                        }
                    })
                    .build(),
                options,
            )
            .await
    }

    pub async fn list_paginated(
        &self,
        request: &ProductsListQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<AsyncPaginator<ProductResponse>, ApiError> {
        let http_client = std::sync::Arc::new(self.http_client.clone());
        let base_query_params = QueryBuilder::new()
            .string("fee_type", {
                let joined = request
                    .fee_type
                    .iter()
                    .flatten()
                    .map(|value| value.to_string())
                    .collect::<Vec<_>>()
                    .join(",");
                if joined.is_empty() {
                    None
                } else {
                    Some(joined)
                }
            })
            .int("limit", request.limit.clone())
            .string("metric_ids", {
                let joined = request
                    .metric_ids
                    .iter()
                    .flatten()
                    .map(|value| value.to_string())
                    .collect::<Vec<_>>()
                    .join(",");
                if joined.is_empty() {
                    None
                } else {
                    Some(joined)
                }
            })
            .string("payment_terms", {
                let joined = request
                    .payment_terms
                    .iter()
                    .flatten()
                    .map(|value| value.to_string())
                    .collect::<Vec<_>>()
                    .join(",");
                if joined.is_empty() {
                    None
                } else {
                    Some(joined)
                }
            })
            .build();
        let options_clone = options.clone();

        AsyncPaginator::new(
            http_client,
            move |client, page_token| {
                let mut query_params: Vec<(String, String)> =
                    base_query_params.clone().unwrap_or_default();

                // Use page_token as offset/page number (start from 0 if None)
                let current_page = page_token.unwrap_or_else(|| "0".to_string());
                query_params.push(("offset".to_string(), current_page.clone()));
                let page_size: Option<i64> = query_params
                    .iter()
                    .find(|(name, _)| name == "limit")
                    .and_then(|(_, value)| value.parse::<i64>().ok());

                let options_for_request = options_clone.clone();

                // Clone captured variables to move into the async block

                Box::pin(async move {
                    let raw_response = client
                        .execute_request_raw::<serde_json::Value>(
                            Method::GET,
                            "products",
                            None,
                            Some(query_params),
                            options_for_request,
                        )
                        .await?;
                    let response = raw_response.body;

                    // Extract pagination info from response
                    // Generic field extraction for offset pagination
                    let items: Vec<serde_json::Value> = response
                        .get("data")
                        .and_then(|v| v.as_array())
                        .map(|arr| arr.clone())
                        .unwrap_or_default();

                    let has_next_page = match page_size {
                        Some(size) => items.len() as i64 >= size.min(100),
                        None => !items.is_empty(),
                    };
                    let next_cursor: Option<String> = if has_next_page {
                        let current_offset: i64 = current_page.parse().unwrap_or(0);
                        Some((current_offset + items.len() as i64).to_string())
                    } else {
                        None
                    };

                    let items = items
                        .into_iter()
                        .map(serde_json::from_value::<ProductResponse>)
                        .collect::<Result<Vec<_>, _>>()
                        .map_err(ApiError::Serialization)?;
                    Ok(PaginationResult {
                        items,
                        next_cursor,
                        has_next_page,
                        response: Some(response),
                        status_code: raw_response.status_code,
                        headers: raw_response.headers,
                    })
                })
            },
            request.offset.map(|offset| offset.to_string()),
        )
    }

    /// Creates a new product. Products are the billable unit on subscription items and define whether pricing is fixed or metered, the payment terms, the billing frequency, the SKU for downstream ERP systems, and the revenue allocation method. Metered products must be associated with one or more metrics.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use alguna::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         api_key: Some("<token>".to_string()),
    ///         ..Default::default()
    ///     };
    ///     let client = Alguna::new(config).expect("Failed to build client");
    ///     client
    ///         .products
    ///         .create(
    ///             &CreateProductRequest {
    ///                 fee_type: PricingClass::Fixed,
    ///                 name: "Platform Fee".to_string(),
    ///                 payment_terms: BillingDirection::Advance,
    ///                 billing_frequency: None,
    ///                 description: None,
    ///                 metric_ids: None,
    ///                 revenue_allocation_method: None,
    ///                 sku: None,
    ///                 tag_ids: None,
    ///                 tax_code: None,
    ///                 unit_label_plural: None,
    ///                 unit_label_singular: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn create(
        &self,
        request: &CreateProductRequest,
        options: Option<RequestOptions>,
    ) -> Result<ProductResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "products",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Fetches a single product by ID.
    ///
    /// # Arguments
    ///
    /// * `id` - Unique identifier for the product
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use alguna::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         api_key: Some("<token>".to_string()),
    ///         ..Default::default()
    ///     };
    ///     let client = Alguna::new(config).expect("Failed to build client");
    ///     client.products.get(&"id".to_string(), None).await;
    /// }
    /// ```
    pub async fn get(
        &self,
        id: &str,
        options: Option<RequestOptions>,
    ) -> Result<ProductResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("products/{}", id),
                None,
                None,
                options,
            )
            .await
    }

    /// Soft-deletes a product. Products currently used on active subscriptions cannot be deleted.
    ///
    /// # Arguments
    ///
    /// * `id` - Unique identifier for the product
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// Empty response
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use alguna::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         api_key: Some("<token>".to_string()),
    ///         ..Default::default()
    ///     };
    ///     let client = Alguna::new(config).expect("Failed to build client");
    ///     client.products.delete(&"id".to_string(), None).await;
    /// }
    /// ```
    pub async fn delete(&self, id: &str, options: Option<RequestOptions>) -> Result<(), ApiError> {
        self.http_client
            .execute_request(
                Method::DELETE,
                &format!("products/{}", id),
                None,
                None,
                options,
            )
            .await
    }

    /// Patches an existing product. Only the fields provided in the request body are modified.
    ///
    /// # Arguments
    ///
    /// * `id` - Unique identifier for the product
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use alguna::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         api_key: Some("<token>".to_string()),
    ///         ..Default::default()
    ///     };
    ///     let client = Alguna::new(config).expect("Failed to build client");
    ///     client
    ///         .products
    ///         .update(
    ///             &"id".to_string(),
    ///             &UpdateProductRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn update(
        &self,
        id: &str,
        request: &UpdateProductRequest,
        options: Option<RequestOptions>,
    ) -> Result<ProductResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::PATCH,
                &format!("products/{}", id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }
}
