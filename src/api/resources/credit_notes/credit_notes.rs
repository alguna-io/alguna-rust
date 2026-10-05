use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use crate::{AsyncPaginator, PaginationResult};
use reqwest::Method;

pub struct CreditNotesClient {
    pub http_client: HttpClient,
}

impl CreditNotesClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Returns a paginated list of credit notes for a given customer. The `customer_id` query parameter is required.
    ///
    /// # Arguments
    ///
    /// * `customer_id` - Customer ID to list credit notes for
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
    ///         .credit_notes
    ///         .list(
    ///             &CreditNotesListQueryRequest {
    ///                 customer_id: "cust_01H1VECT".to_string(),
    ///                 limit: Some(20),
    ///                 offset: Some(0),
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn list(
        &self,
        request: &CreditNotesListQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<CreditNoteListResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                "credit-notes",
                None,
                QueryBuilder::new()
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
        request: &CreditNotesListQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<AsyncPaginator<CreditNoteResponse>, ApiError> {
        let http_client = std::sync::Arc::new(self.http_client.clone());
        let base_query_params = QueryBuilder::new()
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
                            "credit-notes",
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
                        .map(serde_json::from_value::<CreditNoteResponse>)
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

    /// Creates a credit note in draft status. Optionally link it to an invoice. Linked paid invoices are supported, and pre-payment credit notes are supported for Alguna-managed invoices in `issued` or `overdue`. Add line items to specify the credited amounts, then apply the credit note to finalize it. Alternatively, set `apply` to create and apply the credit note in one call — the same synchronous flow as calling Apply Credit Note on a draft. Applying alone does not return funds; send `refund` as well to refund the customer, which requires a linked invoice that is paid. If applying fails, the credit note is left in draft and the error identifies it so it can be applied separately.
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
    ///         .credit_notes
    ///         .create(
    ///             &CreateCreditNoteRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn create(
        &self,
        request: &CreateCreditNoteRequest,
        options: Option<RequestOptions>,
    ) -> Result<CreditNoteResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "credit-notes",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Fetches a single credit note by ID, including line items and attachments.
    ///
    /// # Arguments
    ///
    /// * `id` - Unique identifier for the credit-note
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
    ///     client.credit_notes.get(&"id".to_string(), None).await;
    /// }
    /// ```
    pub async fn get(
        &self,
        id: &str,
        options: Option<RequestOptions>,
    ) -> Result<CreditNoteResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("credit-notes/{}", id),
                None,
                None,
                options,
            )
            .await
    }

    /// Applies a draft credit note. Applying to a paid invoice can optionally trigger a refund to the original payment method or a wallet. Applying to an Alguna-managed invoice in `issued` or `overdue` records a pre-payment credit without issuing a refund. Once applied, the credit note cannot be modified.
    ///
    /// # Arguments
    ///
    /// * `id` - Unique identifier for the credit-note
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
    ///         .credit_notes
    ///         .apply(
    ///             &"id".to_string(),
    ///             &ApplyCreditNoteRequest {
    ///                 do_refund: false,
    ///                 refund_destination: None,
    ///                 wallet_id: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn apply(
        &self,
        id: &str,
        request: &ApplyCreditNoteRequest,
        options: Option<RequestOptions>,
    ) -> Result<CreditNoteResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!("credit-notes/{}/apply", id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Creates a draft credit note against an invoice, crediting it at the prices and tax it was issued with rather than calculating tax again. Omit `line_items` to credit everything still uncredited on the invoice. Give `line_items` to credit specific invoice lines, each by the `quantity` of units to credit, which may be fractional. Every line here credits an invoice line item; to credit an amount that is not on the invoice, use create-credit-note instead.
    ///
    /// # Arguments
    ///
    /// * `id` - Unique identifier for the invoice
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
    ///         .credit_notes
    ///         .create_from_invoice(
    ///             &"id".to_string(),
    ///             &CreateCreditNoteFromInvoiceRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn create_from_invoice(
        &self,
        id: &str,
        request: &CreateCreditNoteFromInvoiceRequest,
        options: Option<RequestOptions>,
    ) -> Result<CreditNoteResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!("invoices/{}/credit-notes", id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }
}
