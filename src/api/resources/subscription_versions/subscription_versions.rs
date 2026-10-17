use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, RequestOptions};
use reqwest::Method;

pub struct SubscriptionVersionsClient {
    pub http_client: HttpClient,
}

impl SubscriptionVersionsClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Returns every version attached to a subscription: drafts, the currently active published version, and any future pending versions.
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
    ///         .subscription_versions
    ///         .list(&"id".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn list(
        &self,
        id: &str,
        options: Option<RequestOptions>,
    ) -> Result<SubscriptionPhaseListResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("subscriptions/{}/versions", id),
                None,
                None,
                options,
            )
            .await
    }

    /// Creates a new version from a complete item set. Returns a draft by default that can be edited and published later. This is the snapshot-based mutation path (Path B) for full control over the version contents. For the delta-based alternative, use the Apply Changes to a Subscription endpoint. See the [Updating a Subscription](https://alguna.com/docs/api-reference/v2/examples/updating-a-subscription) examples.
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
    ///         .subscription_versions
    ///         .create(
    ///             &"id".to_string(),
    ///             &CreateVersionRequest {
    ///                 items: vec![VersionItemInput {
    ///                     ..Default::default()
    ///                 }],
    ///                 description: None,
    ///                 draft: None,
    ///                 effective_at: None,
    ///                 entitlements: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn create(
        &self,
        id: &str,
        request: &CreateVersionRequest,
        options: Option<RequestOptions>,
    ) -> Result<SubscriptionPhaseDetailResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!("subscriptions/{}/versions", id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Returns the subscription version that is currently in effect. This is the published version whose start date has passed and whose end date is either in the future or null. Returns 404 if the subscription has no active published version.
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
    ///         .subscription_versions
    ///         .get_current(&"id".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn get_current(
        &self,
        id: &str,
        options: Option<RequestOptions>,
    ) -> Result<SubscriptionPhaseDetailResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("subscriptions/{}/versions/current", id),
                None,
                None,
                options,
            )
            .await
    }

    /// Fetches a single version by ID, including its priced items and entitlements.
    ///
    /// # Arguments
    ///
    /// * `id` - Unique identifier for the subscription
    /// * `vid` - Unique identifier for the version
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
    ///         .subscription_versions
    ///         .get(&"id".to_string(), &"vid".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn get(
        &self,
        id: &str,
        vid: &str,
        options: Option<RequestOptions>,
    ) -> Result<SubscriptionPhaseDetailResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("subscriptions/{}/versions/{}", id, vid),
                None,
                None,
                options,
            )
            .await
    }

    /// Replaces the contents of a draft version in place. Snapshot-based: send the full intended item set. Items you omit are removed. Only draft versions can be replaced, published versions are immutable. See the [Updating a Subscription](https://alguna.com/docs/api-reference/v2/examples/updating-a-subscription) examples.
    ///
    /// # Arguments
    ///
    /// * `id` - Unique identifier for the subscription
    /// * `vid` - Unique identifier for the version
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
    ///         .subscription_versions
    ///         .replace(
    ///             &"id".to_string(),
    ///             &"vid".to_string(),
    ///             &ReplaceVersionRequest {
    ///                 items: vec![VersionItemInput {
    ///                     ..Default::default()
    ///                 }],
    ///                 description: None,
    ///                 effective_at: None,
    ///                 entitlements: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn replace(
        &self,
        id: &str,
        vid: &str,
        request: &ReplaceVersionRequest,
        options: Option<RequestOptions>,
    ) -> Result<SubscriptionPhaseDetailResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::PUT,
                &format!("subscriptions/{}/versions/{}", id, vid),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Discards an unwanted draft version. Published versions are immutable historical records and cannot be deleted, they must be superseded by creating a new version instead. See the [Updating a Subscription](https://alguna.com/docs/api-reference/v2/examples/updating-a-subscription) examples.
    ///
    /// # Arguments
    ///
    /// * `id` - Unique identifier for the subscription
    /// * `vid` - Unique identifier for the version
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
    ///         .subscription_versions
    ///         .delete(&"id".to_string(), &"vid".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn delete(
        &self,
        id: &str,
        vid: &str,
        options: Option<RequestOptions>,
    ) -> Result<(), ApiError> {
        self.http_client
            .execute_request(
                Method::DELETE,
                &format!("subscriptions/{}/versions/{}", id, vid),
                None,
                None,
                options,
            )
            .await
    }

    /// Fetches a single priced item from a version, addressed by either its product ID or its bundle ID. Bundle items return every child with its price.
    ///
    /// # Arguments
    ///
    /// * `id` - Unique identifier for the subscription
    /// * `vid` - Unique identifier for the version
    /// * `ref_` - Unique identifier for the item
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
    ///         .subscription_versions
    ///         .get_item(
    ///             &"id".to_string(),
    ///             &"vid".to_string(),
    ///             &"ref".to_string(),
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn get_item(
        &self,
        id: &str,
        vid: &str,
        ref_: &str,
        options: Option<RequestOptions>,
    ) -> Result<SubscriptionPhaseItemResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("subscriptions/{}/versions/{}/items/{}", id, vid, ref_),
                None,
                None,
                options,
            )
            .await
    }

    /// Publishes a draft version. If `effective_at` is in the past or today the version becomes the current active version immediately. Otherwise it sits as a pending change on the parent subscription until the effective date. See the [Updating a Subscription](https://alguna.com/docs/api-reference/v2/examples/updating-a-subscription) examples.
    ///
    /// # Arguments
    ///
    /// * `id` - Unique identifier for the subscription
    /// * `vid` - Unique identifier for the version
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
    ///         .subscription_versions
    ///         .publish(&"id".to_string(), &"vid".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn publish(
        &self,
        id: &str,
        vid: &str,
        options: Option<RequestOptions>,
    ) -> Result<SubscriptionPhaseDetailResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!("subscriptions/{}/versions/{}/publish", id, vid),
                None,
                None,
                options,
            )
            .await
    }

    /// Returns the monthly recurring revenue (MRR), annual recurring revenue (ARR), and annual contract value (ACV) computed from the pricing on the specified subscription version. This is the same calculation used in the Alguna dashboard subscription builder, so results match what is shown there one-to-one.
    ///
    /// # Arguments
    ///
    /// * `id` - Unique identifier for the subscription
    /// * `vid` - Unique identifier for the version
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
    ///         .subscription_versions
    ///         .get_revenue(&"id".to_string(), &"vid".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn get_revenue(
        &self,
        id: &str,
        vid: &str,
        options: Option<RequestOptions>,
    ) -> Result<SubscriptionRevenueResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("subscriptions/{}/versions/{}/revenue", id, vid),
                None,
                None,
                options,
            )
            .await
    }
}
