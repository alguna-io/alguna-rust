//! # Alguna Public API SDK
//!
//! The official Rust SDK for the Alguna Public API.
//!
//! ## Getting Started
//!
//! ```rust
//! use alguna::prelude::*;
//!
//! #[tokio::main]
//! async fn main() {
//!     let config = ClientConfig {
//!         api_key: Some("<token>".to_string()),
//!         ..Default::default()
//!     };
//!     let client = Alguna::new(config).expect("Failed to build client");
//!     client
//!         .events
//!         .ingest(
//!             &IngestEventsRequest {
//!                 events: vec![EventItem {
//!                     account: "cust_01H1VECT".to_string(),
//!                     event_name: "api_call".to_string(),
//!                     unique_id: "evt_abc123".to_string(),
//!                     ..Default::default()
//!                 }],
//!             },
//!             None,
//!         )
//!         .await;
//! }
//! ```
//!
//! ## Modules
//!
//! - [`api`] - Core API types and models
//! - [`client`] - Client implementations
//! - [`config`] - Configuration options
//! - [`core`] - Core utilities and infrastructure
//! - [`error`] - Error types and handling
//! - [`prelude`] - Common imports for convenience

pub mod api;
pub mod client;
pub mod config;
pub mod core;
pub mod environment;
pub mod error;
pub mod prelude;

pub use api::*;
pub use client::*;
pub use config::*;
pub use core::*;
pub use environment::*;
pub use error::{ApiError, BuildError};

mod ext;
