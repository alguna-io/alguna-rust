use crate::api::*;
use crate::{ApiError, ByteStream, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use crate::{AsyncPaginator, PaginationResult};
use reqwest::Method;

pub struct QuotesClient {
    pub http_client: HttpClient,
}

impl QuotesClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Returns a paginated list of quotes, filterable by subscription, kind, and status.
    ///
    /// # Arguments
    ///
    /// * `kind` - Filter by quote kind
    /// * `limit` - Page size
    /// * `offset` - Number of quotes to skip
    /// * `status` - Filter by document status
    /// * `subscription_id` - Only quotes documenting this subscription
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
    ///         .quotes
    ///         .list(
    ///             &QuotesListQueryRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn list(
        &self,
        request: &QuotesListQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<QuoteListResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                "quotes",
                None,
                QueryBuilder::new()
                    .serialize("kind", request.kind.clone())
                    .int("limit", request.limit.clone())
                    .int("offset", request.offset.clone())
                    .serialize("status", request.status.clone())
                    .string("subscription_id", request.subscription_id.clone())
                    .build(),
                options,
            )
            .await
    }

    pub async fn list_paginated(
        &self,
        request: &QuotesListQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<AsyncPaginator<QuoteResponse>, ApiError> {
        let http_client = std::sync::Arc::new(self.http_client.clone());
        let base_query_params = QueryBuilder::new()
            .serialize("kind", request.kind.clone())
            .int("limit", request.limit.clone())
            .serialize("status", request.status.clone())
            .string("subscription_id", request.subscription_id.clone())
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
                            "quotes",
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
                        .map(serde_json::from_value::<QuoteResponse>)
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

    /// Creates a quote, in `draft`. A new-business quote starts from a plan, duplicates an existing subscription using `source_subscription_id`, or starts from a blank canvas with `items` (single phase) or `phases` (ramp); its phases are read and edited under `/quotes/{id}/phases` until the quote is activated. An amendment quote names the active subscription it amends (`amends_subscription_id`) and carries the changed phase in `phases`; it may be issued immediately, so branch on `status` and read `signing_url` when it is present.
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
    ///         .quotes
    ///         .create(
    ///             &QuoteCreateRequest {
    ///                 customer_id: "cust_abc123".to_string(),
    ///                 amends_subscription_id: None,
    ///                 auto_activate: None,
    ///                 contract: None,
    ///                 expires_at: None,
    ///                 items: None,
    ///                 metadata: None,
    ///                 name: None,
    ///                 phases: None,
    ///                 plan_id: None,
    ///                 price_book_id: None,
    ///                 source_subscription_id: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn create(
        &self,
        request: &QuoteCreateRequest,
        options: Option<RequestOptions>,
    ) -> Result<QuoteResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "quotes",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Returns a quote, including its `signing_url` while a customer signature is pending. Whether the deal is in effect is the state of `subscription_id`, not of the quote: a signed quote stays `signed`, so read that subscription to see activation.
    ///
    /// # Arguments
    ///
    /// * `id` - Unique identifier for the quote
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
    ///     client.quotes.get(&"id".to_string(), None).await;
    /// }
    /// ```
    pub async fn get(
        &self,
        id: &str,
        options: Option<RequestOptions>,
    ) -> Result<QuoteResponse, ApiError> {
        self.http_client
            .execute_request(Method::GET, &format!("quotes/{}", id), None, None, options)
            .await
    }

    /// Deletes a draft quote and the draft subscription it documents, for a quote that was abandoned before anyone saw it. Requires `status` of `draft` and a new-business quote: once sent, a quote is a document someone may hold, so it is withdrawn rather than erased.
    ///
    /// # Arguments
    ///
    /// * `id` - Unique identifier for the quote
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
    ///     client.quotes.discard(&"id".to_string(), None).await;
    /// }
    /// ```
    pub async fn discard(&self, id: &str, options: Option<RequestOptions>) -> Result<(), ApiError> {
        self.http_client
            .execute_request(
                Method::DELETE,
                &format!("quotes/{}", id),
                None,
                None,
                options,
            )
            .await
    }

    /// Updates a quote's name override and its signing deadline. The override names the quote, not the subscription; subscription content is edited through the quote's phases.
    ///
    /// # Arguments
    ///
    /// * `id` - Unique identifier for the quote
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
    ///         .quotes
    ///         .update(
    ///             &"id".to_string(),
    ///             &QuoteUpdateRequest {
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
        request: &QuoteUpdateRequest,
        options: Option<RequestOptions>,
    ) -> Result<QuoteResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::PATCH,
                &format!("quotes/{}", id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Puts a signed quote into effect: requires `status` of `signed`. A new-business quote activates its subscription; an amendment publishes its phase onto the live subscription. Quotes with `auto_activate` take effect on their own once signed (a new-business quote at its contract start), so this is only needed for manual activation. The quote itself stays `signed` — read `subscription_id` to confirm the deal is live.
    ///
    /// # Arguments
    ///
    /// * `id` - Unique identifier for the quote
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
    ///     client.quotes.activate(&"id".to_string(), None).await;
    /// }
    /// ```
    pub async fn activate(
        &self,
        id: &str,
        options: Option<RequestOptions>,
    ) -> Result<QuoteResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!("quotes/{}/activate", id),
                None,
                None,
                options,
            )
            .await
    }

    /// Records a signature captured outside the hosted signing page, for consumers that collect signer consent through their own e-sign flow. Accepts a `draft` quote, issuing it first so no live Alguna signing link has to be created for a document signed elsewhere, or one already `sent`. The quote moves to `signed` and the subscription to `accepted`, exactly as a hosted signature would. When issuing a `draft` quote needs approval, the call fails with the pending approval flow and records no signature; after approval issues the quote, call this again with a signature that applies to the issued terms. `signed_at` must lie between the quote's creation and now.
    ///
    /// # Arguments
    ///
    /// * `id` - Unique identifier for the quote
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
    ///         .quotes
    ///         .mark_signed_offline(
    ///             &"id".to_string(),
    ///             &QuoteMarkSignedOfflineRequest {
    ///                 signer_email: "ada@acme.com".to_string(),
    ///                 signer_name: "Ada Lovelace".to_string(),
    ///                 signed_at: None,
    ///                 signer_role: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn mark_signed_offline(
        &self,
        id: &str,
        request: &QuoteMarkSignedOfflineRequest,
        options: Option<RequestOptions>,
    ) -> Result<QuoteResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!("quotes/{}/mark-signed-offline", id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Returns the quote document as a binary `application/pdf` response: the PDF the customer signed, or the currently issued one while the quote is out for signature. Returns `404` when there is no issued document — a quote still in `draft`, or a withdrawn amendment, whose staged phase the withdrawal discarded.
    ///
    /// # Arguments
    ///
    /// * `id` - Unique identifier for the quote
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
    ///     client.quotes.get_pdf(&"id".to_string(), None).await;
    /// }
    /// ```
    pub async fn get_pdf(
        &self,
        id: &str,
        options: Option<RequestOptions>,
    ) -> Result<ByteStream, ApiError> {
        self.http_client
            .execute_stream_request(
                Method::GET,
                &format!("quotes/{}/pdf", id),
                None,
                None,
                options,
            )
            .await
    }

    /// Returns the priced phases a quote proposes, in schedule order, each with its revenue totals. An amendment quote has the single phase it was issued for. `sequence` is the position in that order, not an identifier — deleting a phase renumbers the rest, so address a phase by its `id`. Each phase is the subscription version it becomes: once the quote is signed the same rows are readable on the subscription.
    ///
    /// # Arguments
    ///
    /// * `id` - Unique identifier for the quote
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
    ///     client.quotes.list_phases(&"id".to_string(), None).await;
    /// }
    /// ```
    pub async fn list_phases(
        &self,
        id: &str,
        options: Option<RequestOptions>,
    ) -> Result<QuotePhaseListResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("quotes/{}/phases", id),
                None,
                None,
                options,
            )
            .await
    }

    /// Stages a further priced phase on a new-business quote that has not been signed. Name its start as a date or as months after the billing start.
    ///
    /// # Arguments
    ///
    /// * `id` - Unique identifier for the quote
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
    ///         .quotes
    ///         .create_phase(
    ///             &"id".to_string(),
    ///             &QuotePhaseInput {
    ///                 items: vec![VersionItemInput {
    ///                     ..Default::default()
    ///                 }],
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn create_phase(
        &self,
        id: &str,
        request: &QuotePhaseInput,
        options: Option<RequestOptions>,
    ) -> Result<QuotePhaseResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!("quotes/{}/phases", id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Returns one of the quote's phases with its revenue totals.
    ///
    /// # Arguments
    ///
    /// * `id` - Unique identifier for the quote
    /// * `pid` - Unique identifier for the phase
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
    ///         .quotes
    ///         .get_phase(&"id".to_string(), &"pid".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn get_phase(
        &self,
        id: &str,
        pid: &str,
        options: Option<RequestOptions>,
    ) -> Result<QuotePhaseResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("quotes/{}/phases/{}", id, pid),
                None,
                None,
                options,
            )
            .await
    }

    /// Replaces the items and start of one of the quote's draft phases. Send the complete item set; items you omit are removed. Phases publish when the quote is sent and cannot change after that.
    ///
    /// # Arguments
    ///
    /// * `id` - Unique identifier for the quote
    /// * `pid` - Unique identifier for the phase
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
    ///         .quotes
    ///         .replace_phase(
    ///             &"id".to_string(),
    ///             &"pid".to_string(),
    ///             &QuotePhaseInput {
    ///                 items: vec![VersionItemInput {
    ///                     ..Default::default()
    ///                 }],
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn replace_phase(
        &self,
        id: &str,
        pid: &str,
        request: &QuotePhaseInput,
        options: Option<RequestOptions>,
    ) -> Result<QuotePhaseResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::PUT,
                &format!("quotes/{}/phases/{}", id, pid),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Discards one of the quote's draft phases. Phases publish when the quote is sent and cannot be deleted after that.
    ///
    /// # Arguments
    ///
    /// * `id` - Unique identifier for the quote
    /// * `pid` - Unique identifier for the phase
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
    ///         .quotes
    ///         .delete_phase(&"id".to_string(), &"pid".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn delete_phase(
        &self,
        id: &str,
        pid: &str,
        options: Option<RequestOptions>,
    ) -> Result<(), ApiError> {
        self.http_client
            .execute_request(
                Method::DELETE,
                &format!("quotes/{}/phases/{}", id, pid),
                None,
                None,
                options,
            )
            .await
    }

    /// Returns the priced items of one of the quote's phases.
    ///
    /// # Arguments
    ///
    /// * `id` - Unique identifier for the quote
    /// * `pid` - Unique identifier for the phase
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
    ///         .quotes
    ///         .list_phase_items(&"id".to_string(), &"pid".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn list_phase_items(
        &self,
        id: &str,
        pid: &str,
        options: Option<RequestOptions>,
    ) -> Result<QuotePhaseItemListResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("quotes/{}/phases/{}/items", id, pid),
                None,
                None,
                options,
            )
            .await
    }

    /// Issues a quote revision (snapshot + rendered PDF) and moves the quote to `sent`, returning it with the `signing_url` for the customer to review and sign. Requires `draft`, `rejected` or `pending_approval`, and only a new-business quote: an amendment is issued when it is created. Omit `signer_email` to send no invitation and share the link yourself. When issuing needs approval, the call fails with the pending approval flow and the quote is not issued until that approval completes; repeating the call meanwhile returns the same pending flow.
    ///
    /// # Arguments
    ///
    /// * `id` - Unique identifier for the quote
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
    ///         .quotes
    ///         .send_for_signature(
    ///             &"id".to_string(),
    ///             &QuoteSendRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn send_for_signature(
        &self,
        id: &str,
        request: &QuoteSendRequest,
        options: Option<RequestOptions>,
    ) -> Result<QuoteResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!("quotes/{}/send", id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Cancels a quote that is out for signature: requires `status` of `sent`. Withdrawing a new-business quote withdraws its subscription and keeps the issued document; withdrawing an amendment discards the staged draft phase, so its document no longer renders. Use Discard a draft quote for one that was never sent.
    ///
    /// # Arguments
    ///
    /// * `id` - Unique identifier for the quote
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
    ///         .quotes
    ///         .withdraw(
    ///             &"id".to_string(),
    ///             &QuoteWithdrawRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn withdraw(
        &self,
        id: &str,
        request: &QuoteWithdrawRequest,
        options: Option<RequestOptions>,
    ) -> Result<QuoteResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!("quotes/{}/withdraw", id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }
}
