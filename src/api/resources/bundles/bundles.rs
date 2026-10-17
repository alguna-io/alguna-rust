use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use crate::{AsyncPaginator, PaginationResult};
use reqwest::Method;

pub struct BundlesClient {
    pub http_client: HttpClient,
}

impl BundlesClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Returns a paginated list of product bundles.
    ///
    /// # Arguments
    ///
    /// * `limit` - Number of items to return per page
    /// * `offset` - Number of items to skip
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
    ///         .bundles
    ///         .list(
    ///             &BundlesListQueryRequest {
    ///                 limit: Some(20),
    ///                 offset: Some(0),
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn list(
        &self,
        request: &BundlesListQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<BundleListResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                "bundles",
                None,
                QueryBuilder::new()
                    .int("limit", request.limit.clone())
                    .int("offset", request.offset.clone())
                    .build(),
                options,
            )
            .await
    }

    pub async fn list_paginated(
        &self,
        request: &BundlesListQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<AsyncPaginator<BundleResponse>, ApiError> {
        let http_client = std::sync::Arc::new(self.http_client.clone());
        let base_query_params = QueryBuilder::new()
            .int("limit", request.limit.clone())
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
                            "bundles",
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
                        .map(serde_json::from_value::<BundleResponse>)
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

    /// Creates a new product bundle. Product bundles group multiple priced products together so they can be referenced by a single ID in subscriptions, plans, and quotes.
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
    ///         .bundles
    ///         .create(
    ///             &CreateBundleRequest {
    ///                 name: "Standard Plan".to_string(),
    ///                 prices: vec![PriceCreateRequest {
    ///                     billing_direction: BillingDirection::Advance,
    ///                     billing_frequency: None,
    ///                     billing_interval: None,
    ///                     billing_interval_count: None,
    ///                     billing_interval_unit: None,
    ///                     charge_on_contract_start: true,
    ///                     discount: None,
    ///                     display_order: None,
    ///                     expression_pricing_model: None,
    ///                     fee_type: PricingClass::Fixed,
    ///                     fixed_pricing_model: None,
    ///                     graduated_percentage_pricing_model: None,
    ///                     graduated_tiered_pricing_model: None,
    ///                     matrix_pricing_model: None,
    ///                     maximum_spend: None,
    ///                     metric_ids: None,
    ///                     minimum_spend: None,
    ///                     prepaid_fixed_tiered_pricing_model: None,
    ///                     prepaid_tiered_pricing_model: None,
    ///                     product_id: "product_id".to_string(),
    ///                     recurring_instalment_interval_count: None,
    ///                     recurring_instalment_interval_unit: None,
    ///                     tiered_percentage_pricing_model: None,
    ///                     tiered_pricing_model: None,
    ///                     trial_period_days: None,
    ///                     r#type: PricingType::Unit,
    ///                     unit_pricing_model: None,
    ///                     volume_percentage_pricing_model: None,
    ///                 }],
    ///                 description: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn create(
        &self,
        request: &CreateBundleRequest,
        options: Option<RequestOptions>,
    ) -> Result<BundleResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "bundles",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Fetches a single product bundle by ID, including its default prices.
    ///
    /// # Arguments
    ///
    /// * `id` - Unique identifier for the bundle
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
    ///     client.bundles.get(&"id".to_string(), None).await;
    /// }
    /// ```
    pub async fn get(
        &self,
        id: &str,
        options: Option<RequestOptions>,
    ) -> Result<BundleResponse, ApiError> {
        self.http_client
            .execute_request(Method::GET, &format!("bundles/{}", id), None, None, options)
            .await
    }

    /// Patches an existing product bundle. Supplying `prices` replaces the bundle's full price set.
    ///
    /// # Arguments
    ///
    /// * `id` - Unique identifier for the bundle
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
    ///         .bundles
    ///         .update(
    ///             &"id".to_string(),
    ///             &UpdateBundleRequest {
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
        request: &UpdateBundleRequest,
        options: Option<RequestOptions>,
    ) -> Result<BundleResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::PATCH,
                &format!("bundles/{}", id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }
}
