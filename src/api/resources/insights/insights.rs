use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;

pub struct InsightsClient {
    pub http_client: HttpClient,
}

impl InsightsClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Returns the organization-wide receivables position as of now: total open balance, the past-due portion, invoice counts, and a breakdown by days past due. Unlike the other insights endpoints this is a point-in-time snapshot rather than a period aggregate, so it takes no date range. Amounts from every currency are converted into the organization's base currency.
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
    ///     client.insights.ar_aging(None).await;
    /// }
    /// ```
    pub async fn ar_aging(
        &self,
        options: Option<RequestOptions>,
    ) -> Result<ArAgingSummaryResponse, ApiError> {
        self.http_client
            .execute_request(Method::GET, "insights/ar-aging", None, None, options)
            .await
    }

    /// Returns one entry per customer and currency, each with the open balance, invoice counts, and a breakdown by days past due. A customer billed in more than one currency produces one entry per currency, so entries for a single customer can be summed by invoice count but never by amount. Like the aging summary, this is a point-in-time snapshot and takes no date range.
    ///
    /// # Arguments
    ///
    /// * `customer_alias` - Filter by an alternative identifier (external ID) for the customer. Used when customer_id is not provided.
    /// * `customer_id` - Filter by customer ID. Provide at most one of customer_id or customer_alias; omit both to report on every customer.
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
    ///         .insights
    ///         .ar_aging_by_customer(
    ///             &ArAgingByCustomerQueryRequest {
    ///                 customer_alias: Some("ext_39003209203023902".to_string()),
    ///                 customer_id: Some("cust_abc123".to_string()),
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn ar_aging_by_customer(
        &self,
        request: &ArAgingByCustomerQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<ArAgingByCustomerResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                "insights/ar-aging/by-customer",
                None,
                QueryBuilder::new()
                    .string("customer_alias", request.customer_alias.clone())
                    .string("customer_id", request.customer_id.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Returns top customers ranked by net billings for the selected period, with a comparison to the prior period. Remaining customers are aggregated into an 'other' total.
    ///
    /// # Arguments
    ///
    /// * `date_range` - Time period for the query. Determines the primary period and automatically calculates a comparison period.
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
    ///         .insights
    ///         .billings_by_customer(
    ///             &BillingsByCustomerQueryRequest {
    ///                 date_range: BillingsByCustomerInsightsRequestDateRange::ThisMonth,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn billings_by_customer(
        &self,
        request: &BillingsByCustomerQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<BillingsByCustomerResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                "insights/billings/by-customer",
                None,
                QueryBuilder::new()
                    .serialize("date_range", Some(request.date_range.clone()))
                    .build(),
                options,
            )
            .await
    }

    /// Returns billings broken down by product for the selected period, with a comparison to the prior period.
    ///
    /// # Arguments
    ///
    /// * `customer_id` - Optional customer ID to filter by
    /// * `date_range` - Time period for the query.
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
    ///         .insights
    ///         .billings_by_product(
    ///             &BillingsByProductQueryRequest {
    ///                 customer_id: Some("cust_abc123".to_string()),
    ///                 date_range: BillingsByProductInsightsRequestDateRange::ThisMonth,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn billings_by_product(
        &self,
        request: &BillingsByProductQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<BillingsByProductResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                "insights/billings/by-product",
                None,
                QueryBuilder::new()
                    .string("customer_id", request.customer_id.clone())
                    .serialize("date_range", Some(request.date_range.clone()))
                    .build(),
                options,
            )
            .await
    }

    /// Returns a time series of net billings data points split into fixed and metered components. Based on issued invoices.
    ///
    /// # Arguments
    ///
    /// * `date_range` - Time period for the query. Determines the primary period and automatically calculates a comparison period.
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
    ///         .insights
    ///         .billings_over_time(
    ///             &BillingsOverTimeQueryRequest {
    ///                 date_range: BillingsOverTimeInsightsRequestDateRange::ThisMonth,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn billings_over_time(
        &self,
        request: &BillingsOverTimeQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<AmountsOverTimeResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                "insights/billings/over-time",
                None,
                QueryBuilder::new()
                    .serialize("date_range", Some(request.date_range.clone()))
                    .build(),
                options,
            )
            .await
    }

    /// Returns customer lifecycle metrics: active, churned, new, and reactivated customer counts with comparison to the prior period.
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
    ///     client.insights.customers(None).await;
    /// }
    /// ```
    pub async fn customers(
        &self,
        options: Option<RequestOptions>,
    ) -> Result<CustomerInsightsResponse, ApiError> {
        self.http_client
            .execute_request(Method::GET, "insights/customers", None, None, options)
            .await
    }

    /// Returns a time series of customer lifecycle events: new, reactivated, existing, and churned customer counts per period.
    ///
    /// # Arguments
    ///
    /// * `date_range` - Time period for the query. Determines the primary period and automatically calculates a comparison period.
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
    ///         .insights
    ///         .customers_over_time(
    ///             &CustomersOverTimeQueryRequest {
    ///                 date_range: CustomersOverTimeInsightsRequestDateRange::ThisMonth,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn customers_over_time(
        &self,
        request: &CustomersOverTimeQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<CustomerInsightsOverTimeResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                "insights/customers/over-time",
                None,
                QueryBuilder::new()
                    .serialize("date_range", Some(request.date_range.clone()))
                    .build(),
                options,
            )
            .await
    }

    /// Returns a net revenue summary including current month, last month, and annualized figures broken down by fixed (recurring) and metered (usage-based) revenue.
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
    ///     client.insights.revenue(None).await;
    /// }
    /// ```
    pub async fn revenue(
        &self,
        options: Option<RequestOptions>,
    ) -> Result<NetRevenueResponse, ApiError> {
        self.http_client
            .execute_request(Method::GET, "insights/revenue", None, None, options)
            .await
    }

    /// Returns a time series of revenue classified by movement type: existing, new, expansion, contraction, and churn.
    ///
    /// # Arguments
    ///
    /// * `date_range` - Time period for the query. Determines the primary period and automatically calculates a comparison period.
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
    ///         .insights
    ///         .revenue_breakdown_over_time(
    ///             &RevenueBreakdownOverTimeQueryRequest {
    ///                 date_range: RevenueBreakdownOverTimeInsightsRequestDateRange::ThisMonth,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn revenue_breakdown_over_time(
        &self,
        request: &RevenueBreakdownOverTimeQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<RevenueBreakdownOverTimeResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                "insights/revenue/breakdown-over-time",
                None,
                QueryBuilder::new()
                    .serialize("date_range", Some(request.date_range.clone()))
                    .build(),
                options,
            )
            .await
    }

    /// Returns top customers ranked by net revenue for the selected period, with a comparison to the prior period. Remaining customers are aggregated into an 'other' total.
    ///
    /// # Arguments
    ///
    /// * `date_range` - Time period for the query. Determines the primary period and automatically calculates a comparison period.
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
    ///         .insights
    ///         .revenue_by_customer(
    ///             &RevenueByCustomerQueryRequest {
    ///                 date_range: RevenueByCustomerInsightsRequestDateRange::ThisMonth,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn revenue_by_customer(
        &self,
        request: &RevenueByCustomerQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<RevenueByCustomerResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                "insights/revenue/by-customer",
                None,
                QueryBuilder::new()
                    .serialize("date_range", Some(request.date_range.clone()))
                    .build(),
                options,
            )
            .await
    }

    /// Returns revenue broken down by product for the selected period, with a comparison to the prior period.
    ///
    /// # Arguments
    ///
    /// * `customer_id` - Optional customer ID to filter by
    /// * `date_range` - Time period for the query.
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
    ///         .insights
    ///         .revenue_by_product(
    ///             &RevenueByProductQueryRequest {
    ///                 customer_id: Some("cust_abc123".to_string()),
    ///                 date_range: RevenueByProductInsightsRequestDateRange::ThisMonth,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn revenue_by_product(
        &self,
        request: &RevenueByProductQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<RevenueByProductResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                "insights/revenue/by-product",
                None,
                QueryBuilder::new()
                    .string("customer_id", request.customer_id.clone())
                    .serialize("date_range", Some(request.date_range.clone()))
                    .build(),
                options,
            )
            .await
    }

    /// Returns a time series of net revenue data points split into fixed and metered components. Resolution (daily or monthly) is automatically determined by the date range.
    ///
    /// # Arguments
    ///
    /// * `date_range` - Time period for the query. Determines the primary period and automatically calculates a comparison period.
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
    ///         .insights
    ///         .revenue_over_time(
    ///             &RevenueOverTimeQueryRequest {
    ///                 date_range: RevenueOverTimeInsightsRequestDateRange::ThisMonth,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn revenue_over_time(
        &self,
        request: &RevenueOverTimeQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<AmountsOverTimeResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                "insights/revenue/over-time",
                None,
                QueryBuilder::new()
                    .serialize("date_range", Some(request.date_range.clone()))
                    .build(),
                options,
            )
            .await
    }

    /// Returns the monthly recurring revenue (MRR), annual recurring revenue (ARR), and annual contract value (ACV) for a single subscription.
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
    ///         .insights
    ///         .revenue_by_subscription(&"id".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn revenue_by_subscription(
        &self,
        id: &str,
        options: Option<RequestOptions>,
    ) -> Result<RevenueBySubscriptionResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("insights/revenue/subscription/{}", id),
                None,
                None,
                options,
            )
            .await
    }
}
