use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use crate::{AsyncPaginator, PaginationResult};
use reqwest::Method;

pub struct PlansClient {
    pub http_client: HttpClient,
}

impl PlansClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Returns a paginated list of plans. Supports filtering by tag IDs.
    ///
    /// # Arguments
    ///
    /// * `limit` - Number of items to return per page
    /// * `offset` - Number of items to skip
    /// * `sort` - Sort field and direction in format field:order
    /// * `tag_ids` - Filter by tag IDs (comma-separated to match any)
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
    ///         .plans
    ///         .list(
    ///             &PlansListQueryRequest {
    ///                 limit: Some(20),
    ///                 offset: Some(0),
    ///                 sort: Some("created_at:desc".to_string()),
    ///                 tag_ids: vec![],
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn list(
        &self,
        request: &PlansListQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<PlanListResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                "plans",
                None,
                QueryBuilder::new()
                    .int("limit", request.limit.clone())
                    .int("offset", request.offset.clone())
                    .string("sort", request.sort.clone())
                    .string("tag_ids", {
                        let joined = request
                            .tag_ids
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
        request: &PlansListQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<AsyncPaginator<PlanResponse>, ApiError> {
        let http_client = std::sync::Arc::new(self.http_client.clone());
        let base_query_params = QueryBuilder::new()
            .int("limit", request.limit.clone())
            .string("sort", request.sort.clone())
            .string("tag_ids", {
                let joined = request
                    .tag_ids
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
                            "plans",
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
                        .map(serde_json::from_value::<PlanResponse>)
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

    /// Creates a new plan with its full pricing template — name, currency, contract configuration, and the items (standalone priced products and/or bundles) that subscriptions will inherit. Optional fields cover plan-level discount, spend thresholds, price escalation, entitlements, tags, and invoice/email behavior.
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
    ///         .plans
    ///         .create(
    ///             &CreatePlanRequest {
    ///                 currency: "USD".to_string(),
    ///                 name: "Enterprise Plan".to_string(),
    ///                 additional_terms: None,
    ///                 auto_activate: None,
    ///                 auto_renew: None,
    ///                 contract_duration: None,
    ///                 contract_period_type: None,
    ///                 description: None,
    ///                 discount: None,
    ///                 entitlements: None,
    ///                 invoice_settings: None,
    ///                 items: None,
    ///                 maximum_spend: None,
    ///                 minimum_spend: None,
    ///                 payment_terms: None,
    ///                 price_escalation: None,
    ///                 renewal_duration: None,
    ///                 renewal_period_type: None,
    ///                 tags: None,
    ///                 trial_period_days: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn create(
        &self,
        request: &CreatePlanRequest,
        options: Option<RequestOptions>,
    ) -> Result<PlanResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "plans",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Fetches a single plan by ID, including its item set and pricing.
    ///
    /// # Arguments
    ///
    /// * `id` - Unique identifier for the plan
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
    ///     client.plans.get(&"id".to_string(), None).await;
    /// }
    /// ```
    pub async fn get(
        &self,
        id: &str,
        options: Option<RequestOptions>,
    ) -> Result<PlanResponse, ApiError> {
        self.http_client
            .execute_request(Method::GET, &format!("plans/{}", id), None, None, options)
            .await
    }
}
