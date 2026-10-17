use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use crate::{AsyncPaginator, PaginationResult};
use reqwest::Method;

pub struct TaxClient {
    pub http_client: HttpClient,
}

impl TaxClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Returns a paginated list of stored calculations, newest first, for reconciling what was quoted against what was issued. Each entry carries the total tax, whether the calculation was issued from and into which invoice, and whether it can still be issued from. Supports filtering by customer reference, currency, issued and expired state, and a tax date range. Per-line tax is on the get endpoint, not here.
    ///
    /// # Arguments
    ///
    /// * `consumed` - Filter by whether an invoice has been issued from the calculation
    /// * `currency` - Filter by currency. Pass a comma-separated list to match any.
    /// * `customer_reference` - Filter by your own buyer id. Pass a comma-separated list to match any.
    /// * `expired` - Filter by whether the calculation can still be issued from
    /// * `limit` - Number of items to return per page
    /// * `offset` - Number of items to skip
    /// * `sort` - Sort field and direction in format field:order. Sortable fields: created_at, tax_date, expires_at, total_tax, currency.
    /// * `tax_date_gte` - Tax date lower bound (inclusive)
    /// * `tax_date_lte` - Tax date upper bound (inclusive)
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
    ///         .tax
    ///         .list_calculations(
    ///             &ListCalculationsQueryRequest {
    ///                 limit: Some(20),
    ///                 offset: Some(0),
    ///                 sort: Some("created_at:desc".to_string()),
    ///                 tax_date_gte: Some(DateTime::parse_from_rfc3339("2026-01-01T00:00:00Z").unwrap()),
    ///                 tax_date_lte: Some(DateTime::parse_from_rfc3339("2026-12-31T00:00:00Z").unwrap()),
    ///                 consumed: None,
    ///                 currency: vec![],
    ///                 customer_reference: vec![],
    ///                 expired: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn list_calculations(
        &self,
        request: &ListCalculationsQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<TaxCalculationListResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                "tax/calculations",
                None,
                QueryBuilder::new()
                    .bool("consumed", request.consumed.clone())
                    .string("currency", {
                        let joined = request
                            .currency
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
                    .string("customer_reference", {
                        let joined = request
                            .customer_reference
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
                    .bool("expired", request.expired.clone())
                    .int("limit", request.limit.clone())
                    .int("offset", request.offset.clone())
                    .string("sort", request.sort.clone())
                    .datetime("tax_date_gte", request.tax_date_gte.clone())
                    .datetime("tax_date_lte", request.tax_date_lte.clone())
                    .build(),
                options,
            )
            .await
    }

    pub async fn list_calculations_paginated(
        &self,
        request: &ListCalculationsQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<AsyncPaginator<TaxCalculationListItemResponse>, ApiError> {
        let http_client = std::sync::Arc::new(self.http_client.clone());
        let base_query_params = QueryBuilder::new()
            .bool("consumed", request.consumed.clone())
            .string("currency", {
                let joined = request
                    .currency
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
            .string("customer_reference", {
                let joined = request
                    .customer_reference
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
            .bool("expired", request.expired.clone())
            .int("limit", request.limit.clone())
            .string("sort", request.sort.clone())
            .datetime("tax_date_gte", request.tax_date_gte.clone())
            .datetime("tax_date_lte", request.tax_date_lte.clone())
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
                            "tax/calculations",
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
                        .map(serde_json::from_value::<TaxCalculationListItemResponse>)
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

    /// Computes tax for inline line items against your legal entity's configured tax engine and stores the result, returning an id (with an expiry) you pass when issuing a receipt to pin these exact line items and amounts. Each call creates a new calculation; send an `Idempotency-Key` header to make a retry return the original calculation instead of a new one. Provide a buyer address (and optional tax ids for reverse-charge / exemption) and per-line canonical tax codes; an omitted code defaults to general business SaaS.
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
    ///         .tax
    ///         .create_calculation(
    ///             &CreateTaxCalculationRequest {
    ///                 currency: "USD".to_string(),
    ///                 customer: TaxCalculationCustomerRequest {
    ///                     address: TaxCalculationAddressRequest {
    ///                         country: "USA".to_string(),
    ///                         ..Default::default()
    ///                     },
    ///                     first_name: "Jane".to_string(),
    ///                     last_name: "Doe".to_string(),
    ///                     ..Default::default()
    ///                 },
    ///                 line_items: vec![TaxCalculationLineItemRequest {
    ///                     amount: "100.00".to_string(),
    ///                     quantity: "1".to_string(),
    ///                     ..Default::default()
    ///                 }],
    ///                 legal_entity_id: None,
    ///                 merchant_id: None,
    ///                 tax_behavior: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn create_calculation(
        &self,
        request: &CreateTaxCalculationRequest,
        options: Option<RequestOptions>,
    ) -> Result<TaxCalculationResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "tax/calculations",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Returns one stored calculation: the lines as they were submitted joined with the engine's answer for each, plus the invoice it was issued into when it has been. A calculation stays readable after it expires or is issued from, so this is the audit record behind an invoice's tax. Read it back with the `tax_calculation_id` returned on the invoice.
    ///
    /// # Arguments
    ///
    /// * `id` - Unique identifier for the calculation
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
    ///     client.tax.get_calculation(&"id".to_string(), None).await;
    /// }
    /// ```
    pub async fn get_calculation(
        &self,
        id: &str,
        options: Option<RequestOptions>,
    ) -> Result<TaxCalculationDetailResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("tax/calculations/{}", id),
                None,
                None,
                options,
            )
            .await
    }
}
