use ensambria_core::platform_product_policy::{PlatformContractError, PlatformProductPolicy};
use ensambria_platform::product_catalog::ProductCatalog;
use sqlx::PgPool;
use uuid::Uuid;

const PRODUCT_ID: &str = "018f0f2a-7b3c-7abc-8def-0123456789ab";

#[tokio::test]
async fn core_enforces_platform_published_product_and_capability_contract() {
    let database_url = std::env::var("PLATFORM_TEST_DATABASE_URL")
        .expect("PLATFORM_TEST_DATABASE_URL must be configured by repository CI");
    let pool = PgPool::connect(&database_url).await.expect("PLATFORM runtime must connect");
    let policy = PlatformProductPolicy::new(ProductCatalog::new(pool.clone()));
    let product_id = Uuid::parse_str(PRODUCT_ID).expect("fixture product UUID must be valid");

    let product = policy.require_capability(product_id, "identity.auth", "v1", 1).await
        .expect("published identity capability must satisfy CORE contract");
    assert_eq!(product.product_key, "ensambria-core");

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
    pool.close().await;
}
