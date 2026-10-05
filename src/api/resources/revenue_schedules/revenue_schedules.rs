use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;

pub struct RevenueSchedulesClient {
    pub http_client: HttpClient,
}

impl RevenueSchedulesClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Returns the revenue schedule for the entire organization, showing projected recognized, deferred, and metered revenue over time.
    ///
    /// # Arguments
    ///
    /// * `currency` - Target currency for conversion. Uses the entity's default if not specified.
    /// * `interval` - Time interval for schedule entries
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
    ///         .revenue_schedules
    ///         .get(
    ///             &GetQueryRequest {
    ///                 currency: Some("USD".to_string()),
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn get(
        &self,
        request: &GetQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<RevenueScheduleReportResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                "revenue-schedules",
                None,
                QueryBuilder::new()
                    .string("currency", request.currency.clone())
                    .serialize("interval", request.interval.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Returns the aggregated revenue schedule for all of a customer's subscriptions and ad-hoc invoices.
    ///
    /// # Arguments
    ///
    /// * `id` - Unique identifier for the customer
    /// * `currency` - Target currency for conversion. Uses the entity's default if not specified.
    /// * `interval` - Time interval for schedule entries
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
    ///         .revenue_schedules
    ///         .get_by_customer(
    ///             &"id".to_string(),
    ///             &GetByCustomerQueryRequest {
    ///                 currency: Some("USD".to_string()),
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn get_by_customer(
        &self,
        id: &str,
        request: &GetByCustomerQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<RevenueScheduleReportResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("revenue-schedules/customer/{}", id),
                None,
                QueryBuilder::new()
                    .string("currency", request.currency.clone())
                    .serialize("interval", request.interval.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Returns the aggregated revenue schedule for all subscriptions and invoices under a legal entity.
    ///
    /// # Arguments
    ///
    /// * `id` - Unique identifier for the legal-entity
    /// * `currency` - Target currency for conversion. Uses the entity's default if not specified.
    /// * `interval` - Time interval for schedule entries
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
    ///         .revenue_schedules
    ///         .get_by_legal_entity(
    ///             &"id".to_string(),
    ///             &GetByLegalEntityQueryRequest {
    ///                 currency: Some("USD".to_string()),
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn get_by_legal_entity(
        &self,
        id: &str,
        request: &GetByLegalEntityQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<RevenueScheduleReportResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("revenue-schedules/legal-entity/{}", id),
                None,
                QueryBuilder::new()
                    .string("currency", request.currency.clone())
                    .serialize("interval", request.interval.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Returns the revenue schedule for a single subscription, showing projected recognized, deferred, and metered revenue over time.
    ///
    /// # Arguments
    ///
    /// * `id` - Unique identifier for the subscription
    /// * `currency` - Target currency for conversion. Uses the entity's default if not specified.
    /// * `interval` - Time interval for schedule entries
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
    ///         .revenue_schedules
    ///         .get_by_subscription(
    ///             &"id".to_string(),
    ///             &GetBySubscriptionQueryRequest {
    ///                 currency: Some("USD".to_string()),
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn get_by_subscription(
        &self,
        id: &str,
        request: &GetBySubscriptionQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<RevenueScheduleReportResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("revenue-schedules/subscription/{}", id),
                None,
                QueryBuilder::new()
                    .string("currency", request.currency.clone())
                    .serialize("interval", request.interval.clone())
                    .build(),
                options,
            )
            .await
    }
}
