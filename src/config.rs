// Replaces the generated config.rs. The generator's `ClientConfig` also has `token`,
// `username`, `password` and OAuth fields that no Alguna auth scheme reads, and its
// defaults send no `Alguna-Version`, ignore `ALGUNA_API_KEY` and add X-Fern-* headers.
// A field the generated code adds or reads differently fails to compile here.

use crate::Environment;
use std::collections::HashMap;
use std::time::Duration;

#[derive(Debug, Clone)]
pub struct ClientConfig {
    pub base_url: String,
    /// Sent as `Authorization: Bearer <key>`. Defaults to the `ALGUNA_API_KEY` environment
    /// variable. `Alguna::new` and `ApiClientBuilder::build` fail without one.
    pub api_key: Option<String>,
    /// The limit for each attempt.
    pub timeout: Duration,
    /// Retries after a network error, a 408, 429, 502, 503 or 504, or a 409 with
    /// `Retry-After`. 0 turns retries off.
    pub max_retries: u32,
    /// Sent on every request. Holds `Alguna-Version`, the API version this SDK was generated from.
    pub custom_headers: HashMap<String, String>,
    pub user_agent: String,
    /// Optional custom `reqwest` client, used as-is for every request.
    /// When set, it owns all transport-level configuration (TLS, proxies, timeout,
    /// user agent); when `None` the SDK builds its own client from `timeout` and
    /// `user_agent`.
    pub reqwest_client: Option<reqwest::Client>,
}

impl Default for ClientConfig {
    fn default() -> Self {
        Self {
            base_url: Environment::default().url().to_string(),
            api_key: std::env::var("ALGUNA_API_KEY").ok(),
            timeout: Duration::from_secs(60),
            max_retries: 2,
            custom_headers: HashMap::from([(
                crate::ext::VERSION_HEADER.to_string(),
                "2026-04-01".to_string(),
            )]),
            user_agent: concat!("alguna-rust/", env!("CARGO_PKG_VERSION")).to_string(),
            reqwest_client: None,
        }
    }
}
