use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, RequestOptions};
use reqwest::Method;

pub struct CheckoutSessionsClient {
    pub http_client: HttpClient,
}

impl CheckoutSessionsClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Creates a checkout session and returns a browser-safe session token plus a hosted checkout URL. Embed the URL with the Alguna Checkout SDK, or redirect the customer to it directly. The session charges a plan's prices, ad-hoc line items, or a bare amount, and expires after `expires_in_seconds`.
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
    ///         .checkout_sessions
    ///         .create(
    ///             &CheckoutSessionCreateRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn create(
        &self,
        request: &CheckoutSessionCreateRequest,
        options: Option<RequestOptions>,
    ) -> Result<CheckoutSessionResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "checkout-sessions",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Returns the current state of a checkout session, including the payment and subscription it created once the customer completes checkout. Poll this endpoint or subscribe to the `checkout.session.completed` webhook event.
    ///
    /// # Arguments
    ///
    /// * `id` - Unique identifier for the checkout-session
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
    ///     client.checkout_sessions.get(&"id".to_string(), None).await;
    /// }
    /// ```
    pub async fn get(
        &self,
        id: &str,
        options: Option<RequestOptions>,
    ) -> Result<CheckoutSessionResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("checkout-sessions/{}", id),
                None,
                None,
                options,
            )
            .await
    }
}
