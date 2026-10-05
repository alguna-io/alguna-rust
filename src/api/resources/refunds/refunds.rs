use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use crate::{AsyncPaginator, PaginationResult};
use reqwest::Method;

pub struct RefundsClient {
    pub http_client: HttpClient,
}

impl RefundsClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Returns a paginated list of refunds. Supports filtering by status, currency, customer, and payment. Sortable by status, amount, currency, and created date.
    ///
    /// # Arguments
    ///
    /// * `currency` - Filter by ISO 4217 currency code
    /// * `customer_id` - Filter by customer ID
    /// * `limit` - Number of items to return per page
    /// * `offset` - Number of items to skip
    /// * `payment_id` - Filter by payment ID
    /// * `sort` - Sort field and direction in format field:order
    /// * `status` - Filter by refund status. Pass a comma-separated list to match any.
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
    ///         .refunds
    ///         .list(
    ///             &RefundsListQueryRequest {
    ///                 currency: Some("USD".to_string()),
    ///                 customer_id: Some("cust_abc123".to_string()),
    ///                 limit: Some(20),
    ///                 offset: Some(0),
    ///                 payment_id: Some("pay_abc123".to_string()),
    ///                 sort: Some("created_at:desc".to_string()),
    ///                 status: vec![],
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn list(
        &self,
        request: &RefundsListQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<RefundListResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                "refunds",
                None,
                QueryBuilder::new()
                    .string("currency", request.currency.clone())
                    .string("customer_id", request.customer_id.clone())
                    .int("limit", request.limit.clone())
                    .int("offset", request.offset.clone())
                    .string("payment_id", request.payment_id.clone())
                    .string("sort", request.sort.clone())
                    .string("status", {
                        let joined = request
                            .status
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
        request: &RefundsListQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<AsyncPaginator<RefundResponse>, ApiError> {
        let http_client = std::sync::Arc::new(self.http_client.clone());
        let base_query_params = QueryBuilder::new()
            .string("currency", request.currency.clone())
            .string("customer_id", request.customer_id.clone())
            .int("limit", request.limit.clone())
            .string("payment_id", request.payment_id.clone())
            .string("sort", request.sort.clone())
            .string("status", {
                let joined = request
                    .status
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
                            "refunds",
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
                        .map(serde_json::from_value::<RefundResponse>)
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

    /// Creates a refund against a payment. For partial or full refunds of invoices, prefer issuing a credit note first via Create Credit Note and applying it, which handles accounting and gives the customer a paper trail. Direct refunds are best for cases where no credit note is needed.
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
    ///         .refunds
    ///         .create(
    ///             &CreateRefundRequest {
    ///                 amount: "99.00".to_string(),
    ///                 payment_id: "pay_01H1VECT".to_string(),
    ///                 reason: None,
    ///                 statement_descriptor: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn create(
        &self,
        request: &CreateRefundRequest,
        options: Option<RequestOptions>,
    ) -> Result<RefundResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "refunds",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Fetches a single refund by ID.
    ///
    /// # Arguments
    ///
    /// * `id` - Unique identifier for the refund
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
    ///     client.refunds.get(&"id".to_string(), None).await;
    /// }
    /// ```
    pub async fn get(
        &self,
        id: &str,
        options: Option<RequestOptions>,
    ) -> Result<RefundResponse, ApiError> {
        self.http_client
            .execute_request(Method::GET, &format!("refunds/{}", id), None, None, options)
            .await
    }
}
