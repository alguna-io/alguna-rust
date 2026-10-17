use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use crate::{AsyncPaginator, PaginationResult};
use reqwest::Method;

pub struct CustomersClient {
    pub http_client: HttpClient,
}

impl CustomersClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Returns a paginated list of customers. Supports filtering by sales owner, customer success manager, tag IDs, active-subscription status, integration IDs, alias, and contact email.
    ///
    /// # Arguments
    ///
    /// * `alias` - Filter by customer alias (external ID)
    /// * `csm_id` - Filter by customer success manager user ID (comma-separated to match any)
    /// * `email` - Filter by contact email address
    /// * `has_active_subscriptions` - Filter by whether the customer has active subscriptions
    /// * `integration_ids` - Filter by external integration IDs (comma-separated to match any)
    /// * `limit` - Number of items to return per page
    /// * `offset` - Number of items to skip
    /// * `sales_owner_id` - Filter by sales owner user ID (comma-separated to match any)
    /// * `sort` - Sort field and direction in format field:order. Sortable fields: name, created_at
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
    ///         .customers
    ///         .list(
    ///             &CustomersListQueryRequest {
    ///                 alias: Some("ext_customer_1".to_string()),
    ///                 email: Some("john@example.com".to_string()),
    ///                 limit: Some(20),
    ///                 offset: Some(0),
    ///                 sort: Some("created_at:desc".to_string()),
    ///                 csm_id: vec![],
    ///                 has_active_subscriptions: None,
    ///                 integration_ids: vec![],
    ///                 sales_owner_id: vec![],
    ///                 tag_ids: vec![],
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn list(
        &self,
        request: &CustomersListQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<CustomerListResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                "customers",
                None,
                QueryBuilder::new()
                    .string("alias", request.alias.clone())
                    .string("csm_id", {
                        let joined = request
                            .csm_id
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
                    .string("email", request.email.clone())
                    .bool(
                        "has_active_subscriptions",
                        request.has_active_subscriptions.clone(),
                    )
                    .string("integration_ids", {
                        let joined = request
                            .integration_ids
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
                    .string("sales_owner_id", {
                        let joined = request
                            .sales_owner_id
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
        request: &CustomersListQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<AsyncPaginator<CustomerResponse>, ApiError> {
        let http_client = std::sync::Arc::new(self.http_client.clone());
        let base_query_params = QueryBuilder::new()
            .string("alias", request.alias.clone())
            .string("csm_id", {
                let joined = request
                    .csm_id
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
            .string("email", request.email.clone())
            .bool(
                "has_active_subscriptions",
                request.has_active_subscriptions.clone(),
            )
            .string("integration_ids", {
                let joined = request
                    .integration_ids
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
            .string("sales_owner_id", {
                let joined = request
                    .sales_owner_id
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
                            "customers",
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
                        .map(serde_json::from_value::<CustomerResponse>)
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

    /// Creates a new customer with optional contacts, billing address, aliases, and a required default currency.
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
    ///         .customers
    ///         .create(
    ///             &CreateCustomerRequest {
    ///                 currency: "USD".to_string(),
    ///                 name: "Acme Corp".to_string(),
    ///                 aliases: None,
    ///                 billing_address: None,
    ///                 contacts: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn create(
        &self,
        request: &CreateCustomerRequest,
        options: Option<RequestOptions>,
    ) -> Result<CustomerResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "customers",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Resolves an alias to its Alguna customer ID. The `alias` accepts the Alguna account ID, a customer alias, or a connected-account external ID (such as a Stripe customer ID). Returns 404 if no customer in your organization matches. Use this when you hold an external identifier and only need the Alguna ID; to fetch the full customer, use Get a customer instead.
    ///
    /// # Arguments
    ///
    /// * `alias` - Customer alias to resolve — an Alguna account ID, a customer alias, or a connected-account external ID (such as a Stripe customer ID)
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
    ///         .customers
    ///         .lookup(
    ///             &LookupQueryRequest {
    ///                 alias: "cus_abc123".to_string(),
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn lookup(
        &self,
        request: &LookupQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<CustomerLookupResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                "customers/lookup",
                None,
                QueryBuilder::new()
                    .string("alias", request.alias.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Fetches a single customer by ID, including contacts, billing address, and tax details (rate, status, country, type, and taxability reason). The `id` path parameter accepts the Alguna account ID, a customer alias, or a connected-account external ID (such as a Stripe customer ID).
    ///
    /// # Arguments
    ///
    /// * `id` - Unique identifier for the customer
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
    ///     client.customers.get(&"id".to_string(), None).await;
    /// }
    /// ```
    pub async fn get(
        &self,
        id: &str,
        options: Option<RequestOptions>,
    ) -> Result<CustomerResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("customers/{}", id),
                None,
                None,
                options,
            )
            .await
    }

    /// Patches an existing customer. Only the fields provided in the request body are modified. Supplying contacts or aliases replaces the full list. The `id` path parameter accepts the Alguna account ID, a customer alias, or a connected-account external ID (such as a Stripe customer ID).
    ///
    /// # Arguments
    ///
    /// * `id` - Unique identifier for the customer
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
    ///         .customers
    ///         .update(
    ///             &"id".to_string(),
    ///             &UpdateCustomerRequest {
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
        request: &UpdateCustomerRequest,
        options: Option<RequestOptions>,
    ) -> Result<CustomerResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::PATCH,
                &format!("customers/{}", id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Returns the customer's accounts in connected external systems (for example a Stripe customer or a HubSpot company), including the integration each link belongs to, the account's identifier in the external system, and a link to it where available. The `id` path parameter accepts the Alguna account ID, a customer alias, or a connected-account external ID (such as a Stripe customer ID).
    ///
    /// # Arguments
    ///
    /// * `id` - Unique identifier for the customer
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
    ///         .customers
    ///         .list_connected_accounts(&"id".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn list_connected_accounts(
        &self,
        id: &str,
        options: Option<RequestOptions>,
    ) -> Result<ConnectedAccountListResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("customers/{}/connected-accounts", id),
                None,
                None,
                options,
            )
            .await
    }

    /// Links the customer to an account in an external system by recording the integration and the account's identifier in that system. This records the mapping only: it does not create anything in the external system and does not trigger a sync of any data in either direction. The `id` path parameter accepts the Alguna account ID, a customer alias, or an existing connected-account external ID.
    ///
    /// # Arguments
    ///
    /// * `id` - Unique identifier for the customer
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
    ///         .customers
    ///         .create_connected_account(
    ///             &"id".to_string(),
    ///             &ConnectedAccountCreateRequest {
    ///                 external_id: "cus_abc123".to_string(),
    ///                 integration_id: "intg_abc123".to_string(),
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn create_connected_account(
        &self,
        id: &str,
        request: &ConnectedAccountCreateRequest,
        options: Option<RequestOptions>,
    ) -> Result<ConnectedAccountResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!("customers/{}/connected-accounts", id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Removes the link between the customer and an external-system account. This removes the mapping only: it does not delete anything in the external system and does not trigger a sync. The `id` path parameter accepts the Alguna account ID, a customer alias, or an existing connected-account external ID.
    ///
    /// # Arguments
    ///
    /// * `id` - Unique identifier for the customer
    /// * `connected_account_id` - Unique identifier for the connected-account
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
    ///     client
    ///         .customers
    ///         .delete_connected_account(&"id".to_string(), &"connectedAccountId".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn delete_connected_account(
        &self,
        id: &str,
        connected_account_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<(), ApiError> {
        self.http_client
            .execute_request(
                Method::DELETE,
                &format!(
                    "customers/{}/connected-accounts/{}",
                    id, connected_account_id
                ),
                None,
                None,
                options,
            )
            .await
    }

    /// Returns the flattened list of active entitlements across all of the customer's active subscriptions. Each entry is a single entitlement on the current published version of one active subscription. Use the `subscription_id` field on each entry to see which subscription the entitlement comes from. The `id` path parameter accepts the Alguna account ID, a customer alias, or a connected-account external ID (such as a Stripe customer ID).
    ///
    /// # Arguments
    ///
    /// * `id` - Unique identifier for the customer
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
    ///         .customers
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
                &format!("customers/{}/entitlements", id),
                None,
                None,
                options,
            )
            .await
    }

    /// Returns the customer's saved, active payment methods, newest first, with redacted details only: card brand, last four digits and expiry, or the last four digits of a bank account. `is_default` marks the payment method that auto-pay charges when a subscription has no payment method of its own. Payment methods saved through a checkout session with `checkout_intent: vault` appear here once the customer completes the session. The `id` path parameter accepts the Alguna account ID, a customer alias, or a connected-account external ID (such as a Stripe customer ID).
    ///
    /// # Arguments
    ///
    /// * `id` - Unique identifier for the customer
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
    ///         .customers
    ///         .list_payment_methods(&"id".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn list_payment_methods(
        &self,
        id: &str,
        options: Option<RequestOptions>,
    ) -> Result<PaymentMethodListResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("customers/{}/payment-methods", id),
                None,
                None,
                options,
            )
            .await
    }
}
