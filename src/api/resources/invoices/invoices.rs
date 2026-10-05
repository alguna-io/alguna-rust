use crate::api::*;
use crate::{ApiError, ByteStream, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use crate::{AsyncPaginator, PaginationResult};
use reqwest::Method;

pub struct InvoicesClient {
    pub http_client: HttpClient,
}

impl InvoicesClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Returns a paginated list of invoices. Supports filtering by status, customer, subscription, autopay flag, external reference, tag IDs, and date ranges on invoicing, issue, due, and paid dates.
    ///
    /// # Arguments
    ///
    /// * `autopay` - Filter by autopay flag
    /// * `customer_id` - Filter by customer ID
    /// * `due_date_gte` - Due date lower bound (inclusive)
    /// * `due_date_lte` - Due date upper bound (inclusive)
    /// * `external_reference` - Filter by external reference
    /// * `invoicing_date_gte` - Invoicing date lower bound (inclusive)
    /// * `invoicing_date_lte` - Invoicing date upper bound (inclusive)
    /// * `issue_date_gte` - Issue date lower bound (inclusive)
    /// * `issue_date_lte` - Issue date upper bound (inclusive)
    /// * `limit` - Number of items to return per page
    /// * `offset` - Number of items to skip
    /// * `paid_date_gte` - Paid date lower bound (inclusive)
    /// * `paid_date_lte` - Paid date upper bound (inclusive)
    /// * `reference` - Filter by invoice reference number
    /// * `sort` - Sort field and direction in format field:order. Sortable fields: created_at, invoicing_date, issue_date, due_date, paid_date, account_name, status
    /// * `status` - Filter by invoice status. Pass a comma-separated list to match any.
    /// * `subscription_id` - Filter by subscription ID
    /// * `tag_ids` - Filter by tag IDs (comma-separated to match any)
    /// * `tax_calculation_id` - Filter by tax calculation ID
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
    ///         .invoices
    ///         .list(
    ///             &InvoicesListQueryRequest {
    ///                 customer_id: Some("cust_abc123".to_string()),
    ///                 due_date_gte: Some(DateTime::parse_from_rfc3339("2026-01-01T00:00:00Z").unwrap()),
    ///                 due_date_lte: Some(DateTime::parse_from_rfc3339("2026-12-31T00:00:00Z").unwrap()),
    ///                 invoicing_date_gte: Some(
    ///                     DateTime::parse_from_rfc3339("2026-01-01T00:00:00Z").unwrap(),
    ///                 ),
    ///                 invoicing_date_lte: Some(
    ///                     DateTime::parse_from_rfc3339("2026-12-31T00:00:00Z").unwrap(),
    ///                 ),
    ///                 issue_date_gte: Some(DateTime::parse_from_rfc3339("2026-01-01T00:00:00Z").unwrap()),
    ///                 issue_date_lte: Some(DateTime::parse_from_rfc3339("2026-12-31T00:00:00Z").unwrap()),
    ///                 limit: Some(20),
    ///                 offset: Some(0),
    ///                 paid_date_gte: Some(DateTime::parse_from_rfc3339("2026-01-01T00:00:00Z").unwrap()),
    ///                 paid_date_lte: Some(DateTime::parse_from_rfc3339("2026-12-31T00:00:00Z").unwrap()),
    ///                 reference: Some("INV-0001".to_string()),
    ///                 sort: Some("created_at:desc".to_string()),
    ///                 subscription_id: Some("sub_abc123".to_string()),
    ///                 tax_calculation_id: Some("taxcalc_abc123".to_string()),
    ///                 autopay: None,
    ///                 external_reference: None,
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
        request: &InvoicesListQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<InvoiceListResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                "invoices",
                None,
                QueryBuilder::new()
                    .bool("autopay", request.autopay.clone())
                    .string("customer_id", request.customer_id.clone())
                    .datetime("due_date_gte", request.due_date_gte.clone())
                    .datetime("due_date_lte", request.due_date_lte.clone())
                    .string("external_reference", request.external_reference.clone())
                    .datetime("invoicing_date_gte", request.invoicing_date_gte.clone())
                    .datetime("invoicing_date_lte", request.invoicing_date_lte.clone())
                    .datetime("issue_date_gte", request.issue_date_gte.clone())
                    .datetime("issue_date_lte", request.issue_date_lte.clone())
                    .int("limit", request.limit.clone())
                    .int("offset", request.offset.clone())
                    .datetime("paid_date_gte", request.paid_date_gte.clone())
                    .datetime("paid_date_lte", request.paid_date_lte.clone())
                    .string("reference", request.reference.clone())
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
                    .string("subscription_id", request.subscription_id.clone())
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
                    .string("tax_calculation_id", request.tax_calculation_id.clone())
                    .build(),
                options,
            )
            .await
    }

    pub async fn list_paginated(
        &self,
        request: &InvoicesListQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<AsyncPaginator<InvoiceListItemResponse>, ApiError> {
        let http_client = std::sync::Arc::new(self.http_client.clone());
        let base_query_params = QueryBuilder::new()
            .bool("autopay", request.autopay.clone())
            .string("customer_id", request.customer_id.clone())
            .datetime("due_date_gte", request.due_date_gte.clone())
            .datetime("due_date_lte", request.due_date_lte.clone())
            .string("external_reference", request.external_reference.clone())
            .datetime("invoicing_date_gte", request.invoicing_date_gte.clone())
            .datetime("invoicing_date_lte", request.invoicing_date_lte.clone())
            .datetime("issue_date_gte", request.issue_date_gte.clone())
            .datetime("issue_date_lte", request.issue_date_lte.clone())
            .int("limit", request.limit.clone())
            .datetime("paid_date_gte", request.paid_date_gte.clone())
            .datetime("paid_date_lte", request.paid_date_lte.clone())
            .string("reference", request.reference.clone())
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
            .string("subscription_id", request.subscription_id.clone())
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
            .string("tax_calculation_id", request.tax_calculation_id.clone())
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
                            "invoices",
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
                        .map(serde_json::from_value::<InvoiceListItemResponse>)
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

    /// Creates an ad-hoc invoice for a customer. Callers can supply line items, a billing period, purchase order number, and an initial status (`draft` or `issued`). Requesting `issued` releases the invoice for issuance, which completes asynchronously — the response returns the invoice in `processing` and it settles to `issued` shortly after, observable via the Get Invoice endpoint. Alternatively, pass `tax_calculation_id` to turn a persisted tax calculation into an invoice: its line items, tax, and buyer are pinned and the invoice is issued synchronously; add a `payment` and `status: paid` to record settlement and mark it paid in the same call.
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
    ///         .invoices
    ///         .create(
    ///             &CreateInvoiceRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn create(
        &self,
        request: &CreateInvoiceRequest,
        options: Option<RequestOptions>,
    ) -> Result<InvoiceResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "invoices",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Fetches a single invoice by ID, including all line items, totals, and status.
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
    ///     client.invoices.get(&"id".to_string(), None).await;
    /// }
    /// ```
    pub async fn get(
        &self,
        id: &str,
        options: Option<RequestOptions>,
    ) -> Result<InvoiceResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("invoices/{}", id),
                None,
                None,
                options,
            )
            .await
    }

    /// Patches descriptive metadata on an existing invoice (description, purchase order number). Line items are managed through the dedicated line item endpoints.
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
    ///         .invoices
    ///         .update(
    ///             &"id".to_string(),
    ///             &UpdateInvoiceRequest {
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
        request: &UpdateInvoiceRequest,
        options: Option<RequestOptions>,
    ) -> Result<InvoiceResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::PATCH,
                &format!("invoices/{}", id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Adds a new line item to an existing invoice. The invoice must not yet be finalized.
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
    ///         .invoices
    ///         .add_line_item(
    ///             &"id".to_string(),
    ///             &CreateLineItemRequest {
    ///                 description: "description".to_string(),
    ///                 quantity: "1".to_string(),
    ///                 unit_price: "100.00".to_string(),
    ///                 billing_period_end: None,
    ///                 billing_period_start: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn add_line_item(
        &self,
        id: &str,
        request: &CreateLineItemRequest,
        options: Option<RequestOptions>,
    ) -> Result<InvoiceResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!("invoices/{}/line-items", id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Removes a line item from an invoice. The invoice must not yet be finalized.
    ///
    /// # Arguments
    ///
    /// * `id` - Unique identifier for the invoice
    /// * `line_item_id` - Unique identifier for the line-item
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
    ///         .invoices
    ///         .delete_line_item(&"id".to_string(), &"line_item_id".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn delete_line_item(
        &self,
        id: &str,
        line_item_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<(), ApiError> {
        self.http_client
            .execute_request(
                Method::DELETE,
                &format!("invoices/{}/line-items/{}", id, line_item_id),
                None,
                None,
                options,
            )
            .await
    }

    /// Patches an existing invoice line item. Only the fields provided in the request body are modified.
    ///
    /// # Arguments
    ///
    /// * `id` - Unique identifier for the invoice
    /// * `line_item_id` - Unique identifier for the line-item
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
    ///         .invoices
    ///         .update_line_item(
    ///             &"id".to_string(),
    ///             &"line_item_id".to_string(),
    ///             &UpdateLineItemRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn update_line_item(
        &self,
        id: &str,
        line_item_id: &str,
        request: &UpdateLineItemRequest,
        options: Option<RequestOptions>,
    ) -> Result<LineItemResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::PATCH,
                &format!("invoices/{}/line-items/{}", id, line_item_id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Records a manual payment against an invoice. Accepts an optional amount (defaults to the invoice total), payment date, external payment source, and free-text note. Transitions the invoice to `paid` when the balance is settled.
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
    ///         .invoices
    ///         .mark_as_paid(
    ///             &"id".to_string(),
    ///             &MarkInvoiceAsPaidRequest {
    ///                 external_payment_source: ExternalPaymentSource::BankTransfer,
    ///                 amount_paid: None,
    ///                 external_payment_note: None,
    ///                 paid_date: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn mark_as_paid(
        &self,
        id: &str,
        request: &MarkInvoiceAsPaidRequest,
        options: Option<RequestOptions>,
    ) -> Result<InvoiceResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!("invoices/{}/mark-as-paid", id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Returns the rendered PDF for an invoice as a binary `application/pdf` response. If a custom PDF has been uploaded for the invoice, it is served directly, otherwise the standard template is rendered on demand.
    ///
    /// # Arguments
    ///
    /// * `id` - Unique identifier for the invoice
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
    ///     client.invoices.generate_pdf(&"id".to_string(), None).await;
    /// }
    /// ```
    pub async fn generate_pdf(
        &self,
        id: &str,
        options: Option<RequestOptions>,
    ) -> Result<ByteStream, ApiError> {
        self.http_client
            .execute_stream_request(
                Method::GET,
                &format!("invoices/{}/pdf", id),
                None,
                None,
                options,
            )
            .await
    }

    /// Voids an invoice, marking it as no longer collectable. Accepts an optional void date (defaults to the current date); it may be backdated but must not be in the future or before the invoice's issue date. Only issued or overdue invoices can be voided: voiding an invoice in any other status returns a 400, and an out-of-range void date returns a 422.
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
    ///         .invoices
    ///         .void(
    ///             &"id".to_string(),
    ///             &VoidInvoiceRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn void(
        &self,
        id: &str,
        request: &VoidInvoiceRequest,
        options: Option<RequestOptions>,
    ) -> Result<InvoiceResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!("invoices/{}/void", id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }
}
