//! Service clients and API endpoints
//!
//! This module contains client implementations for:
//!
//! - **Events**
//! - **Metrics**
//! - **Checkout Sessions**
//! - **Credit Notes**
//! - **Credits**
//! - **Portal**
//! - **Customers**
//! - **Insights**
//! - **Integrations**
//! - **Invoices**
//! - **Merchants**
//! - **Payments**
//! - **Plans**
//! - **Bundles**
//! - **Products**
//! - **Quotes**
//! - **Refunds**
//! - **Revenue Schedules**
//! - **Subscription Changes**
//! - **Subscription Versions**
//! - **Subscriptions**
//! - **Tags**
//! - **Tax**
//! - **Wallet Grants**
//! - **Wallets**
//! - **Webhooks**

use crate::{ApiError, ClientConfig};

pub mod bundles;
pub mod checkout_sessions;
pub mod credit_notes;
pub mod credits;
pub mod customers;
pub mod events;
pub mod insights;
pub mod integrations;
pub mod invoices;
pub mod merchants;
pub mod metrics;
pub mod payments;
pub mod plans;
pub mod portal;
pub mod products;
pub mod quotes;
pub mod refunds;
pub mod revenue_schedules;
pub mod subscription_changes;
pub mod subscription_versions;
pub mod subscriptions;
pub mod tags;
pub mod tax;
pub mod wallet_grants;
pub mod wallets;
pub mod webhooks;
pub struct Alguna {
    pub config: ClientConfig,
    pub events: EventsClient,
    pub metrics: MetricsClient,
    pub checkout_sessions: CheckoutSessionsClient,
    pub credit_notes: CreditNotesClient,
    pub credits: CreditsClient,
    pub portal: PortalClient,
    pub customers: CustomersClient,
    pub insights: InsightsClient,
    pub integrations: IntegrationsClient,
    pub invoices: InvoicesClient,
    pub merchants: MerchantsClient,
    pub payments: PaymentsClient,
    pub plans: PlansClient,
    pub bundles: BundlesClient,
    pub products: ProductsClient,
    pub quotes: QuotesClient,
    pub refunds: RefundsClient,
    pub revenue_schedules: RevenueSchedulesClient,
    pub subscription_changes: SubscriptionChangesClient,
    pub subscription_versions: SubscriptionVersionsClient,
    pub subscriptions: SubscriptionsClient,
    pub tags: TagsClient,
    pub tax: TaxClient,
    pub wallet_grants: WalletGrantsClient,
    pub wallets: WalletsClient,
}

impl Alguna {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        crate::ext::require_api_key(&config)?;
        Ok(Self {
            config: config.clone(),
            events: EventsClient::new(config.clone())?,
            metrics: MetricsClient::new(config.clone())?,
            checkout_sessions: CheckoutSessionsClient::new(config.clone())?,
            credit_notes: CreditNotesClient::new(config.clone())?,
            credits: CreditsClient::new(config.clone())?,
            portal: PortalClient::new(config.clone())?,
            customers: CustomersClient::new(config.clone())?,
            insights: InsightsClient::new(config.clone())?,
            integrations: IntegrationsClient::new(config.clone())?,
            invoices: InvoicesClient::new(config.clone())?,
            merchants: MerchantsClient::new(config.clone())?,
            payments: PaymentsClient::new(config.clone())?,
            plans: PlansClient::new(config.clone())?,
            bundles: BundlesClient::new(config.clone())?,
            products: ProductsClient::new(config.clone())?,
            quotes: QuotesClient::new(config.clone())?,
            refunds: RefundsClient::new(config.clone())?,
            revenue_schedules: RevenueSchedulesClient::new(config.clone())?,
            subscription_changes: SubscriptionChangesClient::new(config.clone())?,
            subscription_versions: SubscriptionVersionsClient::new(config.clone())?,
            subscriptions: SubscriptionsClient::new(config.clone())?,
            tags: TagsClient::new(config.clone())?,
            tax: TaxClient::new(config.clone())?,
            wallet_grants: WalletGrantsClient::new(config.clone())?,
            wallets: WalletsClient::new(config.clone())?,
        })
    }
}

pub use bundles::BundlesClient;
pub use checkout_sessions::CheckoutSessionsClient;
pub use credit_notes::CreditNotesClient;
pub use credits::CreditsClient;
pub use customers::CustomersClient;
pub use events::EventsClient;
pub use insights::InsightsClient;
pub use integrations::IntegrationsClient;
pub use invoices::InvoicesClient;
pub use merchants::MerchantsClient;
pub use metrics::MetricsClient;
pub use payments::PaymentsClient;
pub use plans::PlansClient;
pub use portal::PortalClient;
pub use products::ProductsClient;
pub use quotes::QuotesClient;
pub use refunds::RefundsClient;
pub use revenue_schedules::RevenueSchedulesClient;
pub use subscription_changes::SubscriptionChangesClient;
pub use subscription_versions::SubscriptionVersionsClient;
pub use subscriptions::SubscriptionsClient;
pub use tags::TagsClient;
pub use tax::TaxClient;
pub use wallet_grants::WalletGrantsClient;
pub use wallets::WalletsClient;
pub use webhooks::WebhooksClient;
