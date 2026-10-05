use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, RequestOptions};
use reqwest::Method;

pub struct SubscriptionChangesClient {
    pub http_client: HttpClient,
}

impl SubscriptionChangesClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Applies a delta (`add`, `remove`, `update`) on top of a source version. The API computes the new version, creates it, and publishes it unless `draft: true` is set. The `effective` field controls when the change takes effect (keyword or specific date). This is the high-level mutation endpoint. See the [Updating a Subscription](https://alguna.com/docs/api-reference/v2/examples/updating-a-subscription) examples for concrete bodies covering price adjustments, plan swaps, bundle operations, and tier replacements.
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
    ///         .subscription_changes
    ///         .create(
    ///             &"id".to_string(),
    ///             &CreateChangeRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn create(
        &self,
        id: &str,
        request: &CreateChangeRequest,
        options: Option<RequestOptions>,
    ) -> Result<ChangeResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!("subscriptions/{}/changes", id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Computes the same delta as Apply Changes to a Subscription without writing a new version. Use this to drive approval flows or show the customer what is about to change before committing. See the [Updating a Subscription](https://alguna.com/docs/api-reference/v2/examples/updating-a-subscription) examples.
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
    ///         .subscription_changes
    ///         .preview(
    ///             &"id".to_string(),
    ///             &CreateChangeRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn preview(
        &self,
        id: &str,
        request: &CreateChangeRequest,
        options: Option<RequestOptions>,
    ) -> Result<ChangePreviewResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!("subscriptions/{}/changes/preview", id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }
}
