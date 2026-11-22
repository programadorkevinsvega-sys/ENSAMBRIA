use ensambria_core::{
    core_migrations::run_core_migrations_from_path,
    platform_product_policy::{PlatformContractError, PlatformProductPolicy},
};
use ensambria_platform::product_catalog::ProductCatalog;
use sqlx::PgPool;
use uuid::Uuid;

const PRODUCT_ID: &str = "018f0f2a-7b3c-7abc-8def-0123456789ab";

#[tokio::test]
async fn core_enforces_platform_published_product_and_capability_contract() {
    let database_url = std::env::var("PLATFORM_TEST_DATABASE_URL")
        .expect("PLATFORM_TEST_DATABASE_URL must be configured by repository CI");
    let platform_pool = PgPool::connect(&database_url).await.expect("PLATFORM runtime must connect");
    let core_migrator_url = std::env::var("CORE_TEST_MIGRATOR_DATABASE_URL")
        .expect("CORE_TEST_MIGRATOR_DATABASE_URL must be configured");
    let migrations = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("migrations");
    run_core_migrations_from_path(&core_migrator_url, &migrations)
        .await.expect("CORE migrations must apply");
    let core_url = std::env::var("CORE_TEST_DATABASE_URL")
        .expect("CORE_TEST_DATABASE_URL must be configured");
    let core_pool = PgPool::connect(&core_url).await.expect("CORE runtime must connect");
    let policy = PlatformProductPolicy::new(ProductCatalog::new(platform_pool.clone()));
    let product_id = Uuid::parse_str(PRODUCT_ID).expect("fixture product UUID must be valid");

    let product = policy.require_capability(product_id, "identity.auth", "v1", 1).await
        .expect("published identity capability must satisfy CORE contract");
    assert_eq!(product.product_key, "ensambria-core");

    let tenant_id = Uuid::now_v7();
    let mut transaction = core_pool.begin().await.expect("tenant fixture transaction must begin");
    sqlx::query("SELECT set_config('app.tenant_id', $1, true)")
        .bind(tenant_id.to_string())
        .execute(&mut *transaction).await.expect("tenant RLS context must be set");
    sqlx::query("INSERT INTO core.tenants (id, status) VALUES ($1, 'active')")
        .bind(tenant_id).execute(&mut *transaction).await.expect("tenant fixture must be inserted");
    sqlx::query(
        "INSERT INTO core.tenant_products (tenant_id, product_id, product_config_version, association_status, validated_at) VALUES ($1, $2, 1, 'active', clock_timestamp())",
    )
    .bind(tenant_id).bind(product_id).execute(&mut *transaction).await
        .expect("tenant product association must be inserted");
    transaction.commit().await.expect("tenant fixture must commit");

    let tenant_product = policy.require_tenant_capability(
        &core_pool, tenant_id, product_id, "identity.auth", "v1",
    ).await.expect("CORE tenant association and PLATFORM contract must both be active");
    assert_eq!(tenant_product.product_key, "ensambria-core");

    assert_eq!(
        policy.require_tenant_capability(
            &core_pool, tenant_id, product_id, "identity.auth", "v2",
        ).await.unwrap_err(),
        PlatformContractError::ContractVersionMismatch
    );

    assert_eq!(
        policy.require_capability(product_id, "identity.auth", "v2", 1).await.unwrap_err(),
        PlatformContractError::ContractVersionMismatch
    );
    assert_eq!(
        policy.require_capability(product_id, "identity.auth", "v1", 2).await.unwrap_err(),
        PlatformContractError::ConfigurationVersionMismatch
    );
    assert_eq!(
        policy.require_capability(product_id, "identity.billing", "v1", 1).await.unwrap_err(),
        PlatformContractError::CapabilityUnavailable
    );

    let resolved = policy.resolve_domain(" SHOP.EXAMPLE.TEST. ").await
        .expect("PLATFORM domain resolver must be reachable")
        .expect("published verified domain must resolve");
    assert_eq!(resolved.product_id, product_id);
    core_pool.close().await;
    platform_pool.close().await;
}
