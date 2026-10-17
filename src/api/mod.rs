//! API client and types for the Alguna Public API
//!
//! This module contains all the API definitions including request/response types
//! and client implementations for interacting with the API.
//!
//! ## Modules
//!
//! - [`resources`] - Service clients and endpoints
//! - [`types`] - Request, response, and model types

pub mod resources;
pub mod types;

pub use resources::{
    Alguna, BundlesClient, CheckoutSessionsClient, CreditNotesClient, CreditsClient,
    CustomersClient, EventsClient, InsightsClient, IntegrationsClient, InvoicesClient,
    MerchantsClient, MetricsClient, PaymentsClient, PlansClient, PortalClient, ProductsClient,
    QuotesClient, RefundsClient, RevenueSchedulesClient, SubscriptionChangesClient,
    SubscriptionVersionsClient, SubscriptionsClient, TagsClient, TaxClient, WalletGrantsClient,
    WalletsClient, WebhooksClient,
};
pub use types::*;
