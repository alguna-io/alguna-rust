use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;

pub struct IntegrationsClient {
    pub http_client: HttpClient,
}

impl IntegrationsClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Returns the organization's connected integrations (for example Stripe, HubSpot, or QuickBooks). Use the `id` of an integration when linking a connected account to a customer. Supports filtering by provider type.
    ///
    /// # Arguments
    ///
    /// * `provider` - Filter by integration provider
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
    ///         .integrations
    ///         .list(
    ///             &IntegrationsListQueryRequest {
    ///                 provider: Some("stripe".to_string()),
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn list(
        &self,
        request: &IntegrationsListQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<IntegrationListResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                "integrations",
                None,
                QueryBuilder::new()
                    .string("provider", request.provider.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Fetches a single integration by its ID.
    ///
    /// # Arguments
    ///
    /// * `id` - Unique identifier for the integration
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
    ///     client.integrations.get(&"id".to_string(), None).await;
    /// }
    /// ```
    pub async fn get(
        &self,
        id: &str,
        options: Option<RequestOptions>,
    ) -> Result<IntegrationResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("integrations/{}", id),
                None,
                None,
                options,
            )
            .await
    }
}
