use ensambria_event_bus::connect_jetstream;
use ensambria_platform::{
    product_catalog::ProductCatalog, tenant_onboarding::PlatformTenantOnboarding,
    tenant_onboarding_consumer,
};
use ensambria_platform_product_catalog_contracts::ProductCatalog as ProductCatalogPort;
use ensambria_platform_tenant_onboarding_contracts::TenantOnboarding;
use sqlx::postgres::PgPoolOptions;
use std::{env, error::Error, sync::Arc, time::Duration};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    tracing_subscriber::fmt()
        .json()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info")),
        )
        .init();
    let product_pool = PgPoolOptions::new()
        .max_connections(2)
        .acquire_timeout(Duration::from_secs(5))
        .connect(&env::var("PLATFORM_PRODUCT_CATALOG_DATABASE_URL")?)
        .await?;
    let onboarding_pool = PgPoolOptions::new()
        .max_connections(4)
        .acquire_timeout(Duration::from_secs(5))
        .connect(&env::var("PLATFORM_TENANT_ONBOARDING_DATABASE_URL")?)
        .await?;
    let product_catalog: Arc<dyn ProductCatalogPort> =
        Arc::new(ProductCatalog::new(product_pool));
    let onboarding: Arc<dyn TenantOnboarding> =
        Arc::new(PlatformTenantOnboarding::new(onboarding_pool, product_catalog));
    let jetstream = connect_jetstream(&env::var("ENSAMBRIA_NATS_URL")?).await?;
    tenant_onboarding_consumer::run(onboarding, jetstream).await
}
