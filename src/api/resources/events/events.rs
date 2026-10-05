use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, RequestOptions};
use reqwest::Method;

pub struct EventsClient {
    pub http_client: HttpClient,
}

impl EventsClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Ingests a batch of usage events for metering. Accepts up to 100 events per request. Each event must carry a unique `unique_id` for idempotency and may include arbitrary properties used by metric filters and aggregations.
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
    ///         .events
    ///         .ingest(
    ///             &IngestEventsRequest {
    ///                 events: vec![EventItem {
    ///                     account: "cust_01H1VECT".to_string(),
    ///                     event_name: "api_call".to_string(),
    ///                     unique_id: "evt_abc123".to_string(),
    ///                     ..Default::default()
    ///                 }],
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn ingest(
        &self,
        request: &IngestEventsRequest,
        options: Option<RequestOptions>,
    ) -> Result<IngestEventsResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "events",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }
}
