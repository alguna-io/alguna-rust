use crate::api::*;
use crate::{ApiError, ByteStream, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use crate::{AsyncPaginator, PaginationResult};
use reqwest::Method;

pub struct SubscriptionsClient {
    pub http_client: HttpClient,
}

impl SubscriptionsClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Returns a paginated list of subscriptions. Supports filtering by status, customer ID, and tag IDs.
    ///
    /// # Arguments
    ///
    /// * `customer_id` - Filter by customer ID
    /// * `limit` - Number of items per page
    /// * `offset` - Number of items to skip
    /// * `sort` - Sort field and direction in format field:order. Sortable fields: name, status, created_at, activated_at, ended_at, updated_at, contract_start_date, contract_end_date
    /// * `status` - Filter by subscription status. Pass a comma-separated list to match any.
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
    ///         .subscriptions
    ///         .list(
    ///             &SubscriptionsListQueryRequest {
    ///                 customer_id: Some("cust_abc123".to_string()),
    ///                 limit: Some(20),
    ///                 offset: Some(0),
    ///                 sort: Some("created_at:desc".to_string()),
    ///                 status: vec![],
    ///                 tag_ids: vec![],
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn list(
        &self,
        request: &SubscriptionsListQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<SubscriptionListResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                "subscriptions",
                None,
                QueryBuilder::new()
                    .string("customer_id", request.customer_id.clone())
                    .int("limit", request.limit.clone())
                    .int("offset", request.offset.clone())
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
        request: &SubscriptionsListQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<AsyncPaginator<SubscriptionResponse>, ApiError> {
        let http_client = std::sync::Arc::new(self.http_client.clone());
        let base_query_params = QueryBuilder::new()
            .string("customer_id", request.customer_id.clone())
            .int("limit", request.limit.clone())
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
                            "subscriptions",
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
                        .map(serde_json::from_value::<SubscriptionResponse>)
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

    /// Creates a new subscription for a customer. Supports three creation patterns: raw pricing in the request body, from a plan, or from a plan with per-customer overrides. Subscriptions are created in `draft` status and must be activated unless `auto_activate: true` is set. See the [Creating a Subscription](https://alguna.com/docs/api-reference/v2/examples/creating-a-subscription) examples for concrete request bodies and the bundle variants.
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
    ///         .subscriptions
    ///         .create(
    ///             &CreateSubscriptionRequest {
    ///                 customer_id: "cust_abc123".to_string(),
    ///                 auto_activate: None,
    ///                 billing: None,
    ///                 contract: None,
    ///                 currency: None,
    ///                 items: None,
    ///                 metadata: None,
    ///                 name: None,
    ///                 plan_id: None,
    ///                 renewal: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn create(
        &self,
        request: &CreateSubscriptionRequest,
        options: Option<RequestOptions>,
    ) -> Result<SubscriptionResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "subscriptions",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Fetches a single subscription by ID, including the current active version reference and any pending future versions.
    ///
    /// # Arguments
    ///
    /// * `id` - Unique identifier for the subscription
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
    ///     client.subscriptions.get(&"id".to_string(), None).await;
    /// }
    /// ```
    pub async fn get(
        &self,
        id: &str,
        options: Option<RequestOptions>,
    ) -> Result<SubscriptionResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("subscriptions/{}", id),
                None,
                None,
                options,
            )
            .await
    }

    /// Permanently deletes a draft subscription. Only draft subscriptions can be deleted. Use cancel for subscriptions that have already been activated.
    ///
    /// # Arguments
    ///
    /// * `id` - Unique identifier for the subscription
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
    ///     client.subscriptions.delete(&"id".to_string(), None).await;
    /// }
    /// ```
    pub async fn delete(&self, id: &str, options: Option<RequestOptions>) -> Result<(), ApiError> {
        self.http_client
            .execute_request(
                Method::DELETE,
                &format!("subscriptions/{}", id),
                None,
                None,
                options,
            )
            .await
    }

    /// Patches metadata on a subscription: name, description, trial period, billing flags, subscription-level discount, spending thresholds, price escalation, and tags. This endpoint does not modify the priced item set. To change items, see the [Updating a Subscription](https://alguna.com/docs/api-reference/v2/examples/updating-a-subscription) examples. When an edit of a `sent` subscription saves its terms but the offer is not reissued with them, the call fails with a message that says so and the terms stay saved but cannot be signed; send this request again with an empty body to retry issuing the saved terms.
    ///
    /// # Arguments
    ///
    /// * `id` - Unique identifier for the subscription
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
    ///         .subscriptions
    ///         .update(
    ///             &"id".to_string(),
    ///             &UpdateSubscriptionRequest {
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
        request: &UpdateSubscriptionRequest,
        options: Option<RequestOptions>,
    ) -> Result<SubscriptionResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::PATCH,
                &format!("subscriptions/{}", id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Transitions a draft subscription to `active` so it becomes effective for the customer. Idempotent: activating an already-active subscription returns the current state. See the [Creating a Subscription](https://alguna.com/docs/api-reference/v2/examples/creating-a-subscription) examples for the full creation flow.
    ///
    /// # Arguments
    ///
    /// * `id` - Unique identifier for the subscription
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
    ///     client.subscriptions.activate(&"id".to_string(), None).await;
    /// }
    /// ```
    pub async fn activate(
        &self,
        id: &str,
        options: Option<RequestOptions>,
    ) -> Result<SubscriptionResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!("subscriptions/{}/activate", id),
                None,
                None,
                options,
            )
            .await
    }

    /// Returns the current billing cycle range, the list of valid dates for on-cycle version starts, the valid date range for mid-cycle changes, the contract end date, and the next renewal date. Use this to drive date pickers in UI flows that schedule future changes.
    ///
    /// # Arguments
    ///
    /// * `id` - Unique identifier for the subscription
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
    ///         .subscriptions
    ///         .get_billing_dates(&"id".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn get_billing_dates(
        &self,
        id: &str,
        options: Option<RequestOptions>,
    ) -> Result<BillingDatesResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("subscriptions/{}/billing-dates", id),
                None,
                None,
                options,
            )
            .await
    }

    /// Schedules a cancellation for a subscription. The `effective` timing can be a keyword (e.g. `immediate`, `end_of_term`) or a specific date. A cancellation reason code is required and a free-text note is optional.
    ///
    /// # Arguments
    ///
    /// * `id` - Unique identifier for the subscription
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
    ///         .subscriptions
    ///         .cancel(
    ///             &"id".to_string(),
    ///             &CancelSubscriptionRequest {
    ///                 reason: SubscriptionCancelationReason::NoLongerRequired,
    ///                 effective: None,
    ///                 reason_description: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn cancel(
        &self,
        id: &str,
        request: &CancelSubscriptionRequest,
        options: Option<RequestOptions>,
    ) -> Result<SubscriptionResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!("subscriptions/{}/cancel", id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Returns the customer's monetary credit balance applicable to this subscription, evaluated at the `as_of` timestamp (defaults to now). Expired grants are excluded from the returned balance.
    ///
    /// # Arguments
    ///
    /// * `id` - Unique identifier for the subscription
    /// * `as_of` - Point in time to evaluate the balance at (RFC3339). Defaults to now.
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
    ///         .subscriptions
    ///         .get_credit_balance(
    ///             &"id".to_string(),
    ///             &GetCreditBalanceQueryRequest {
    ///                 as_of: Some(DateTime::parse_from_rfc3339("2026-04-01T00:00:00Z").unwrap()),
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn get_credit_balance(
        &self,
        id: &str,
        request: &GetCreditBalanceQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<SubscriptionCreditBalanceResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("subscriptions/{}/credits/balance", id),
                None,
                QueryBuilder::new()
                    .datetime("as_of", request.as_of.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Creates a monetary API-granted credit on a subscription. The grant is immediately available for consumption.
    ///
    /// # Arguments
    ///
    /// * `id` - Unique identifier for the subscription
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
    ///         .subscriptions
    ///         .create_credit_grant(
    ///             &"id".to_string(),
    ///             &CreditGrantRequest {
    ///                 amount: "100.00".to_string(),
    ///                 reason: "API promotional credit".to_string(),
    ///                 credit_type: None,
    ///                 expires_at: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn create_credit_grant(
        &self,
        id: &str,
        request: &CreditGrantRequest,
        options: Option<RequestOptions>,
    ) -> Result<CreditGrantResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!("subscriptions/{}/credits/grant", id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Returns every entitlement active on the subscription's current published version, including static boolean/number/string entitlements and grant-based credit allocations.
    ///
    /// # Arguments
    ///
    /// * `id` - Unique identifier for the subscription
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
    ///         .subscriptions
    ///         .list_entitlements(&"id".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn list_entitlements(
        &self,
        id: &str,
        options: Option<RequestOptions>,
    ) -> Result<EntitlementListResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("subscriptions/{}/entitlements", id),
                None,
                None,
                options,
            )
            .await
    }

    /// Fetches a single active entitlement on a subscription by its entitlement key.
    ///
    /// # Arguments
    ///
    /// * `id` - Unique identifier for the subscription
    /// * `key` - Unique identifier for the entitlement
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
    ///         .subscriptions
    ///         .get_entitlement(&"id".to_string(), &"key".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn get_entitlement(
        &self,
        id: &str,
        key: &str,
        options: Option<RequestOptions>,
    ) -> Result<EntitlementResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("subscriptions/{}/entitlements/{}", id, key),
                None,
                None,
                options,
            )
            .await
    }

    /// Returns the signed contract PDF for a subscription as a binary `application/pdf` response. The document reflects the contract as it was agreed upon with the customer at the point of signing. Returns `404` if the order form was not signed through Alguna and no PDF exists.
    ///
    /// # Arguments
    ///
    /// * `id` - Unique identifier for the subscription
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// Streaming file download (use .into_bytes() to collect or stream chunks)
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
    ///         .subscriptions
    ///         .download_pdf(&"id".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn download_pdf(
        &self,
        id: &str,
        options: Option<RequestOptions>,
    ) -> Result<ByteStream, ApiError> {
        self.http_client
            .execute_stream_request(
                Method::GET,
                &format!("subscriptions/{}/pdf", id),
                None,
                None,
                options,
            )
            .await
    }

    /// Returns the monthly recurring revenue (MRR), annual recurring revenue (ARR), and annual contract value (ACV) for the subscription's current active version.
    ///
    /// # Arguments
    ///
    /// * `id` - Unique identifier for the subscription
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
    ///         .subscriptions
    ///         .get_revenue(&"id".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn get_revenue(
        &self,
        id: &str,
        options: Option<RequestOptions>,
    ) -> Result<SubscriptionRevenueResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("subscriptions/{}/revenue", id),
                None,
                None,
                options,
            )
            .await
    }
}
