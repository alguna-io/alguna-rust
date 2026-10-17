use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, RequestOptions};
use reqwest::Method;

pub struct PortalClient {
    pub http_client: HttpClient,
}

impl PortalClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Creates a short-lived customer portal session and returns a signed URL the customer can use to access their billing portal. Sessions are single-use per token and expire after a short TTL.
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
    ///         .portal
    ///         .create_session(
    ///             &CreateCustomerPortalSessionRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn create_session(
        &self,
        request: &CreateCustomerPortalSessionRequest,
        options: Option<RequestOptions>,
    ) -> Result<CustomerPortalSessionResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "customer-portal-sessions",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }
}
