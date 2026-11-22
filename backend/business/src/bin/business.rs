use std::{error::Error, net::SocketAddr, sync::Arc};

use ensambria_business::{http::{router, ApiState}, service::BusinessService};
use ensambria_core::{
    identity_authentication::{email_outbox::OutboxEmailDelivery, production::authentication_service_from_env},
    postgres::PgDatabaseConfiguration,
};
use ensambria_shared_cache::TtlCache;
use sqlx::postgres::PgPoolOptions;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let core_config = PgDatabaseConfiguration::from_env()?;
    let core_pool = core_config.connect_lazy();
    let business_url = std::env::var("BUSINESS_DATABASE_URL")?;
    let business_pool = PgPoolOptions::new()
        .max_connections(std::env::var("BUSINESS_DATABASE_MAX_CONNECTIONS").ok().and_then(|v| v.parse().ok()).unwrap_or(32))
        .connect(&business_url).await?;

    let auth_cache = TtlCache::connect_from_env("auth").await.ok();
    let auth_service = authentication_service_from_env(core_pool.clone())?;
    let auth_service = match auth_cache {
        Some(cache) => auth_service.with_rate_limit_cache(cache),
        None => {
            tracing::warn!(event="auth.rate_limit.redis_unavailable", fallback="postgres");
            auth_service
        }
    };
    let auth = Arc::new(auth_service);
    let cache = match TtlCache::connect_from_env("business").await {
        Ok(cache) => Some(cache),
        Err(error) => {
            tracing::warn!(event="cache.redis.business_unavailable", error=%error, fallback="postgres");
            None
        }
    };
    let business = BusinessService::new(business_pool.clone(), cache);
    let state = Arc::new(ApiState { auth, business });
    let app = router::<OutboxEmailDelivery>(state);

    let address: SocketAddr = std::env::var("BUSINESS_HTTP_LISTEN_ADDRESS")
        .unwrap_or_else(|_| "0.0.0.0:8080".to_owned()).parse()?;
    let listener = TcpListener::bind(address).await?;
    tracing::info!(event="business.http.started", address=%listener.local_addr()?);
    axum::serve(listener, app).await?;
    business_pool.close().await;
    Ok(())
}
