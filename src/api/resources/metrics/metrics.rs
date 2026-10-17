use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use crate::{AsyncPaginator, PaginationResult};
use reqwest::Method;

pub struct MetricsClient {
    pub http_client: HttpClient,
}

impl MetricsClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Returns a paginated list of billable metrics. Supports filtering by event name, aggregation method, aggregation field, and tag IDs. Results can be sorted by name.
    ///
    /// # Arguments
    ///
    /// * `aggregation_field` - Filter by aggregation field (comma-separated to match any)
    /// * `aggregation_method` - Filter by aggregation method (comma-separated to match any)
    /// * `event_name` - Filter by event name (comma-separated to match any)
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
    ///         .metrics
    ///         .list(
    ///             &MetricsListQueryRequest {
    ///                 limit: Some(20),
    ///                 offset: Some(0),
    ///                 sort: Some("name:asc".to_string()),
    ///                 aggregation_field: vec![],
    ///                 aggregation_method: vec![],
    ///                 event_name: vec![],
    ///                 tag_ids: vec![],
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn list(
        &self,
        request: &MetricsListQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<MetricListResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                "metrics",
                None,
                QueryBuilder::new()
                    .string("aggregation_field", {
                        let joined = request
                            .aggregation_field
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
                    .string("aggregation_method", {
                        let joined = request
                            .aggregation_method
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
                    .string("event_name", {
                        let joined = request
                            .event_name
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
        request: &MetricsListQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<AsyncPaginator<MetricResponse>, ApiError> {
        let http_client = std::sync::Arc::new(self.http_client.clone());
        let base_query_params = QueryBuilder::new()
            .string("aggregation_field", {
                let joined = request
                    .aggregation_field
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
            .string("aggregation_method", {
                let joined = request
                    .aggregation_method
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
            .string("event_name", {
                let joined = request
                    .event_name
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
                            "metrics",
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
                        .map(serde_json::from_value::<MetricResponse>)
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

    /// Defines a new billable metric. A metric specifies the event name, optional filter groups, and aggregation logic that turns raw ingested events into a quantity the billing engine can price against.
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
    ///         .metrics
    ///         .create(
    ///             &CreateMetricRequest {
    ///                 aggregation: AggregationRequest {
    ///                     field: None,
    ///                     method: AggregationMethod::Count,
    ///                 },
    ///                 event_name: "api_call".to_string(),
    ///                 name: "API Calls".to_string(),
    ///                 description: None,
    ///                 filter_groups: None,
    ///                 tag_ids: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn create(
        &self,
        request: &CreateMetricRequest,
        options: Option<RequestOptions>,
    ) -> Result<MetricResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "metrics",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Fetches a single billable metric by ID, including its filter groups and aggregation logic.
    ///
    /// # Arguments
    ///
    /// * `id` - Unique identifier for the metric
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
    ///     client.metrics.get(&"id".to_string(), None).await;
    /// }
    /// ```
    pub async fn get(
        &self,
        id: &str,
        options: Option<RequestOptions>,
    ) -> Result<MetricResponse, ApiError> {
        self.http_client
            .execute_request(Method::GET, &format!("metrics/{}", id), None, None, options)
            .await
    }
}
