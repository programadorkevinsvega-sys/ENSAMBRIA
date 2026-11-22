use std::{env, error::Error, io, net::SocketAddr};

use ensambria_platform::{
    product_catalog::ProductCatalog,
    product_catalog_rpc::ProductCatalogRpc,
};
use ensambria_platform_contract::platform::product_catalog::v1::ProductCatalogServer;
use ensambria_shared_cache::TtlCache;
use sqlx::PgPool;
use tokio::net::TcpListener;
use tokio_stream::wrappers::TcpListenerStream;
use tonic::transport::Server;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let database_url = env::var("PLATFORM_DATABASE_URL")
        .map_err(|_| io::Error::other("PLATFORM_DATABASE_URL is required"))?;
    let listen_address = env::var("PLATFORM_GRPC_LISTEN_ADDRESS")
        .map_err(|_| io::Error::other("PLATFORM_GRPC_LISTEN_ADDRESS is required"))?
        .parse::<SocketAddr>()?;

    let pool = PgPool::connect(&database_url).await?;
    let catalog = if env::var("PLATFORM_REDIS_URL").is_ok() {
        match TtlCache::connect_from_env("platform").await {
            Ok(cache) => ProductCatalog::with_cache(pool, cache),
            Err(_) => {
                eprintln!("event=cache.redis.startup_failed result=postgres_fallback");
                ProductCatalog::new(pool)
            }
        }
    } else {
        ProductCatalog::new(pool)
    };

    let listener = TcpListener::bind(listen_address).await?;
    let bound_address = listener.local_addr()?;
    eprintln!("event=platform.product_catalog.started address={bound_address}");
    Server::builder()
        .add_service(ProductCatalogServer::new(ProductCatalogRpc::new(catalog)))
        .serve_with_incoming_shutdown(TcpListenerStream::new(listener), shutdown_signal())
        .await?;
    Ok(())
}

async fn shutdown_signal() {
    #[cfg(unix)]
    {
        let mut terminate = tokio::signal::unix::signal(
            tokio::signal::unix::SignalKind::terminate(),
        )
        .expect("SIGTERM handler must be installed");
        tokio::select! {
            _ = tokio::signal::ctrl_c() => {}
            _ = terminate.recv() => {}
        }
    }
    #[cfg(not(unix))]
    {
        let _ = tokio::signal::ctrl_c().await;
    }
}
