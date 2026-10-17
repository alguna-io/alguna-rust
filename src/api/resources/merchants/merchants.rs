use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use crate::{AsyncPaginator, PaginationResult};
use reqwest::Method;

pub struct MerchantsClient {
    pub http_client: HttpClient,
}

impl MerchantsClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Returns a paginated list of merchants, by display name. Archived merchants are excluded unless you ask for them.
    ///
    /// # Arguments
    ///
    /// * `display_name` - Return only merchants with these display names, comma-separated. Matched exactly, including case.
    /// * `external_id` - Return only merchants with these external ids, comma-separated
    /// * `include_archived` - Include archived merchants, which are excluded by default
    /// * `limit` - Merchants to return per page
    /// * `offset` - Merchants to skip before the first returned
    /// * `sort` - Sort field and direction in format field:order. Sortable fields: display_name, external_id, support_email, created_at
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
    ///         .merchants
    ///         .list(
    ///             &MerchantsListQueryRequest {
    ///                 limit: Some(20),
    ///                 offset: Some(0),
    ///                 sort: Some("display_name:asc".to_string()),
    ///                 display_name: vec![],
    ///                 external_id: vec![],
    ///                 include_archived: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn list(
        &self,
        request: &MerchantsListQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<MerchantListResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                "merchants",
                None,
                QueryBuilder::new()
                    .string("display_name", {
                        let joined = request
                            .display_name
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
                    .string("external_id", {
                        let joined = request
                            .external_id
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
                    .bool("include_archived", request.include_archived.clone())
                    .int("limit", request.limit.clone())
                    .int("offset", request.offset.clone())
                    .string("sort", request.sort.clone())
                    .build(),
                options,
            )
            .await
    }

    pub async fn list_paginated(
        &self,
        request: &MerchantsListQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<AsyncPaginator<MerchantResponse>, ApiError> {
        let http_client = std::sync::Arc::new(self.http_client.clone());
        let base_query_params = QueryBuilder::new()
            .string("display_name", {
                let joined = request
                    .display_name
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
            .string("external_id", {
                let joined = request
                    .external_id
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
            .bool("include_archived", request.include_archived.clone())
            .int("limit", request.limit.clone())
            .string("sort", request.sort.clone())
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
                            "merchants",
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
                        .map(serde_json::from_value::<MerchantResponse>)
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

    /// Registers a business you sell on behalf of. Invoices and receipts issued for a merchant carry its name and logo, while your legal entity stays the seller of record and keeps the contract, the payment and the tax liability. A merchant is display identity only: it has no tax or legal configuration of its own.
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
    ///         .merchants
    ///         .create(
    ///             &CreateMerchantRequest {
    ///                 display_name: "Northwind Traders".to_string(),
    ///                 external_id: "acct_492".to_string(),
    ///                 avatar: None,
    ///                 logo: None,
    ///                 support_email: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn create(
        &self,
        request: &CreateMerchantRequest,
        options: Option<RequestOptions>,
    ) -> Result<MerchantResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "merchants",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Fetches a single merchant. Archived merchants still resolve, so documents issued before they were archived keep rendering.
    ///
    /// # Arguments
    ///
    /// * `id` - Unique identifier for the merchant
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
    ///     client.merchants.get(&"id".to_string(), None).await;
    /// }
    /// ```
    pub async fn get(
        &self,
        id: &str,
        options: Option<RequestOptions>,
    ) -> Result<MerchantResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("merchants/{}", id),
                None,
                None,
                options,
            )
            .await
    }

    /// Retires a merchant. Merchants are never deleted, because the documents already issued for them still need their branding: an archived merchant keeps rendering on those, but takes no new attribution — a tax calculation naming it is rejected. Idempotent, and the original archive time is kept. Returns the merchant with `archived_at` set.
    ///
    /// # Arguments
    ///
    /// * `id` - Unique identifier for the merchant
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
    ///     client.merchants.delete(&"id".to_string(), None).await;
    /// }
    /// ```
    pub async fn delete(
        &self,
        id: &str,
        options: Option<RequestOptions>,
    ) -> Result<MerchantResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::DELETE,
                &format!("merchants/{}", id),
                None,
                None,
                options,
            )
            .await
    }

    /// Updates a merchant. Only the fields you send are changed: omit a field to leave it alone, send an empty value to clear it. Renames and new branding apply the next time a document is rendered, including documents issued earlier.
    ///
    /// # Arguments
    ///
    /// * `id` - Unique identifier for the merchant
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
    ///         .merchants
    ///         .update(
    ///             &"id".to_string(),
    ///             &UpdateMerchantRequest {
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
        request: &UpdateMerchantRequest,
        options: Option<RequestOptions>,
    ) -> Result<MerchantResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::PATCH,
                &format!("merchants/{}", id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Returns an archived merchant to active use, so new documents can be issued for it again. Idempotent.
    ///
    /// # Arguments
    ///
    /// * `id` - Unique identifier for the merchant
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
    ///     client.merchants.unarchive(&"id".to_string(), None).await;
    /// }
    /// ```
    pub async fn unarchive(
        &self,
        id: &str,
        options: Option<RequestOptions>,
    ) -> Result<MerchantResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!("merchants/{}/unarchive", id),
                None,
                None,
                options,
            )
            .await
    }
}
