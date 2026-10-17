use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use crate::{AsyncPaginator, PaginationResult};
use reqwest::Method;

pub struct WalletsClient {
    pub http_client: HttpClient,
}

impl WalletsClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Returns every grant attached to a specific wallet.
    ///
    /// # Arguments
    ///
    /// * `id` - Unique identifier for the wallet
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
    ///     client.wallets.list_grants(&"id".to_string(), None).await;
    /// }
    /// ```
    pub async fn list_grants(
        &self,
        id: &str,
        options: Option<RequestOptions>,
    ) -> Result<WalletGrantListResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("wallets/{}/grants", id),
                None,
                None,
                options,
            )
            .await
    }

    /// Returns a paginated list of wallets. Supports filtering by customer ID and active status.
    ///
    /// # Arguments
    ///
    /// * `active` - Filter by active status
    /// * `customer_id` - Filter by customer ID
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
    ///         .wallets
    ///         .list(
    ///             &WalletsListQueryRequest {
    ///                 customer_id: Some("cust_01H2ABC".to_string()),
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
        request: &WalletsListQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<WalletListResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                "wallets",
                None,
                QueryBuilder::new()
                    .bool("active", request.active.clone())
                    .string("customer_id", request.customer_id.clone())
                    .int("limit", request.limit.clone())
                    .int("offset", request.offset.clone())
                    .build(),
                options,
            )
            .await
    }

    pub async fn list_paginated(
        &self,
        request: &WalletsListQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<AsyncPaginator<WalletResponse>, ApiError> {
        let http_client = std::sync::Arc::new(self.http_client.clone());
        let base_query_params = QueryBuilder::new()
            .bool("active", request.active.clone())
            .string("customer_id", request.customer_id.clone())
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
                            "wallets",
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
                        .map(serde_json::from_value::<WalletResponse>)
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

    /// Creates a new wallet for a customer. Wallets hold a balance in a single currency and can optionally be restricted to a specific set of products.
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
    ///         .wallets
    ///         .create(
    ///             &CreateWalletRequest {
    ///                 currency: "USD".to_string(),
    ///                 customer_id: "cust_01H2ABC".to_string(),
    ///                 name: "Main Wallet".to_string(),
    ///                 product_ids: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn create(
        &self,
        request: &CreateWalletRequest,
        options: Option<RequestOptions>,
    ) -> Result<WalletResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "wallets",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Fetches a single wallet by ID.
    ///
    /// # Arguments
    ///
    /// * `id` - Unique identifier for the wallet
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
    ///     client.wallets.get(&"id".to_string(), None).await;
    /// }
    /// ```
    pub async fn get(
        &self,
        id: &str,
        options: Option<RequestOptions>,
    ) -> Result<WalletResponse, ApiError> {
        self.http_client
            .execute_request(Method::GET, &format!("wallets/{}", id), None, None, options)
            .await
    }

    /// Deletes a wallet. Wallets with a non-zero balance cannot be deleted.
    ///
    /// # Arguments
    ///
    /// * `id` - Unique identifier for the wallet
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
    ///     client.wallets.delete(&"id".to_string(), None).await;
    /// }
    /// ```
    pub async fn delete(&self, id: &str, options: Option<RequestOptions>) -> Result<(), ApiError> {
        self.http_client
            .execute_request(
                Method::DELETE,
                &format!("wallets/{}", id),
                None,
                None,
                options,
            )
            .await
    }

    /// Patches an existing wallet. Supports changing the name, currency, active flag, and the list of products the wallet can be used against.
    ///
    /// # Arguments
    ///
    /// * `id` - Unique identifier for the wallet
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
    ///         .wallets
    ///         .update(
    ///             &"id".to_string(),
    ///             &UpdateWalletRequest {
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
        request: &UpdateWalletRequest,
        options: Option<RequestOptions>,
    ) -> Result<WalletResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::PATCH,
                &format!("wallets/{}", id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Returns the current balance for a wallet.
    ///
    /// # Arguments
    ///
    /// * `id` - Unique identifier for the wallet
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
    ///     client.wallets.get_balance(&"id".to_string(), None).await;
    /// }
    /// ```
    pub async fn get_balance(
        &self,
        id: &str,
        options: Option<RequestOptions>,
    ) -> Result<WalletBalanceResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("wallets/{}/balance", id),
                None,
                None,
                options,
            )
            .await
    }

    /// Credits a wallet by the specified amount and records the transaction type, reference ID, and optional description on the ledger.
    ///
    /// # Arguments
    ///
    /// * `id` - Unique identifier for the wallet
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
    ///         .wallets
    ///         .credit(
    ///             &"id".to_string(),
    ///             &CreditDebitWalletRequest {
    ///                 amount: "amount".to_string(),
    ///                 description: None,
    ///                 reference_id: "pay_01H3DEF".to_string(),
    ///                 transaction_type: LedgerEntryTransactionType::CreditGrant,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn credit(
        &self,
        id: &str,
        request: &CreditDebitWalletRequest,
        options: Option<RequestOptions>,
    ) -> Result<WalletResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!("wallets/{}/credit", id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Debits a wallet by the specified amount. Fails with `422` if the wallet has insufficient balance.
    ///
    /// # Arguments
    ///
    /// * `id` - Unique identifier for the wallet
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
    ///         .wallets
    ///         .debit(
    ///             &"id".to_string(),
    ///             &CreditDebitWalletRequest {
    ///                 amount: "amount".to_string(),
    ///                 description: None,
    ///                 reference_id: "pay_01H3DEF".to_string(),
    ///                 transaction_type: LedgerEntryTransactionType::CreditGrant,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn debit(
        &self,
        id: &str,
        request: &CreditDebitWalletRequest,
        options: Option<RequestOptions>,
    ) -> Result<WalletResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!("wallets/{}/debit", id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }
}
