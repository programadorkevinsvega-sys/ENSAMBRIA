use ensambria_core::{
    core_migrations::run_core_migrations_from_path,
    platform_product_policy::{PlatformContractError, PlatformProductPolicy},
    postgres::{ActorContext, ActorId, TenantContext, TenantId},
};
use ensambria_platform::{product_catalog::ProductCatalog, product_catalog_rpc::ProductCatalogRpc};
use ensambria_platform_contract::platform::product_catalog::v1::{
    GetPublishedConfigurationRequest, ProductCatalogClient, ProductCatalogServer,
};
use sqlx::PgPool;
use tokio::{net::TcpListener, sync::oneshot};
use tokio_stream::wrappers::TcpListenerStream;
use tonic::transport::Server;
use uuid::Uuid;

const PRODUCT_ID: &str = "018f0f2a-7b3c-7abc-8def-0123456789ab";

#[tokio::test]
async fn core_enforces_platform_published_product_and_capability_over_real_grpc() {
    let database_url = std::env::var("PLATFORM_TEST_DATABASE_URL")
        .expect("PLATFORM_TEST_DATABASE_URL must be configured by repository CI");
    let platform_pool = PgPool::connect(&database_url)
        .await
        .expect("PLATFORM runtime must connect");

    let listener = TcpListener::bind("127.0.0.1:0")
        .await
        .expect("PLATFORM gRPC listener must bind");
    let address = listener
        .local_addr()
        .expect("PLATFORM gRPC listener address must be available");
    let (shutdown_sender, shutdown_receiver) = oneshot::channel();
    let platform_service = ProductCatalogRpc::new(ProductCatalog::new(platform_pool.clone()));
    let server = tokio::spawn(async move {
        Server::builder()
            .add_service(ProductCatalogServer::new(platform_service))
            .serve_with_incoming_shutdown(
                TcpListenerStream::new(listener),
                async {
                    let _ = shutdown_receiver.await;
                },
            )
            .await
            .expect("PLATFORM ProductCatalog gRPC server must run");
    });

    let client = ProductCatalogClient::connect(format!("http://{address}"))
        .await
        .expect("CORE must connect to the real PLATFORM gRPC contract");
    let mut configuration_client = client.clone();
    let product_id = Uuid::parse_str(PRODUCT_ID).expect("fixture product UUID must be valid");
    let configuration = configuration_client
        .get_published_configuration(GetPublishedConfigurationRequest {
            product_id: product_id.to_string(),
            configuration_version: 1,
        })
        .await
        .expect("PLATFORM must return its published configuration")
        .into_inner();
    assert_eq!(configuration.version, 1);
    assert!(!configuration.canonical_json.is_empty());

    let policy = PlatformProductPolicy::new(client);
    let product = policy
        .require_capability(product_id, "identity.auth", "v1", 1)
        .await
        .expect("published identity capability must satisfy CORE contract");
    assert_eq!(product.product_key, "ensambria-core");

    let core_migrator_url = std::env::var("CORE_TEST_MIGRATOR_DATABASE_URL")
        .expect("CORE_TEST_MIGRATOR_DATABASE_URL must be configured");
    let migrations = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("migrations");
    run_core_migrations_from_path(&core_migrator_url, &migrations)
        .await
        .expect("CORE migrations must apply");
    let core_url =
        std::env::var("CORE_TEST_DATABASE_URL").expect("CORE_TEST_DATABASE_URL must be configured");
    let core_pool = PgPool::connect(&core_url)
        .await
        .expect("CORE runtime must connect");

    let tenant_id = Uuid::now_v7();
    let actor_id = Uuid::now_v7();
    let mut transaction = core_pool
        .begin()
        .await
        .expect("tenant fixture transaction must begin");
    ActorContext::new(ActorId::new(actor_id))
        .establish(&mut transaction)
        .await
        .expect("actor context must be established");
    TenantContext::new(TenantId::new(tenant_id))
        .establish(&mut transaction)
        .await
        .expect("tenant RLS context must be established");
    sqlx::query("INSERT INTO core.tenants (id, status) VALUES ($1, 'active')")
        .bind(tenant_id)
        .execute(&mut *transaction)
        .await
        .expect("tenant fixture must be inserted");
    sqlx::query(
        "INSERT INTO core.tenant_products (tenant_id, product_id, product_config_version, association_status, validated_at) VALUES ($1, $2, 1, 'active', clock_timestamp())",
    )
    .bind(tenant_id)
    .bind(product_id)
    .execute(&mut *transaction)
    .await
    .expect("CORE tenant-product association fixture must be inserted");
    let association_version: Option<i32> = sqlx::query_scalar(
        "SELECT product_config_version FROM core.tenant_products WHERE tenant_id = $1 AND product_id = $2 AND association_status = 'active'",
    )
    .bind(tenant_id)
    .bind(product_id)
    .fetch_optional(&mut *transaction)
    .await
    .expect("tenant association must be read through CORE RLS");
    transaction
        .commit()
        .await
        .expect("tenant association fixture must commit");
    assert_eq!(association_version, Some(1));

    let tenant_product = policy
        .require_capability(
            product_id,
            "identity.auth",
            "v1",
            association_version.expect("active tenant association must exist"),
        )
        .await
        .expect("CORE association version and PLATFORM contract must both be active");
    assert_eq!(tenant_product.product_key, "ensambria-core");

    assert_eq!(
        policy
            .require_capability(product_id, "identity.auth", "v2", 1)
            .await
            .unwrap_err(),
        PlatformContractError::ContractVersionMismatch
    );
    assert_eq!(
        policy
            .require_capability(product_id, "identity.auth", "v1", 2)
            .await
            .unwrap_err(),
        PlatformContractError::ConfigurationVersionMismatch
    );
    assert_eq!(
        policy
            .require_capability(product_id, "identity.billing", "v1", 1)
            .await
            .unwrap_err(),
        PlatformContractError::CapabilityUnavailable
    );
    assert_eq!(
        policy
            .require_capability(Uuid::now_v7(), "identity.auth", "v1", 1)
            .await
            .unwrap_err(),
        PlatformContractError::ProductUnavailable
    );

    let resolved = policy
        .resolve_domain(" SHOP.EXAMPLE.TEST. ")
        .await
        .expect("PLATFORM domain resolver must be reachable")
        .expect("published verified domain must resolve");
    assert_eq!(resolved.product_id, product_id);

    let _ = shutdown_sender.send(());
    server
        .await
        .expect("PLATFORM gRPC server task must finish cleanly");
    core_pool.close().await;
    platform_pool.close().await;
}
