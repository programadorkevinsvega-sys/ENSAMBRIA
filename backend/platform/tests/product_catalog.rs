use ensambria_platform::product_catalog::ProductCatalog;
use sqlx::PgPool;
use uuid::Uuid;

const PRODUCT_ID: &str = "018f0f2a-7b3c-7abc-8def-0123456789ab";

#[tokio::test]
async fn published_product_catalog_contract_is_readable_by_runtime_role() {
    let database_url = std::env::var("PLATFORM_TEST_DATABASE_URL")
        .expect("PLATFORM_TEST_DATABASE_URL must be configured by repository CI");
    let pool = PgPool::connect(&database_url)
        .await
        .expect("restricted PLATFORM runtime role must connect");
    let catalog = ProductCatalog::new(pool.clone());
    let product_id = Uuid::parse_str(PRODUCT_ID).expect("fixture product UUID must be valid");

    let product = catalog
        .get_active_product(product_id)
        .await
        .expect("published catalogue query must succeed")
        .expect("active published product must be visible");
    assert_eq!(product.product_key, "ensambria-core");
    assert_eq!(product.configuration_schema_version, "product-config.v1");
    assert_eq!(product.configuration["branding"]["displayName"], "ENSAMBRIA Core");
    assert_eq!(product.domains.len(), 1);
    assert_eq!(product.domains[0].domain_name, "shop.example.test");
    assert_eq!(product.markets.len(), 1);
    assert_eq!(product.markets[0].country_code, "CO");
    assert_eq!(product.markets[0].currency_code, "COP");
    assert_eq!(product.surfaces.len(), 1);
    assert_eq!(product.surfaces[0].surface_code, "operator.web");
    assert_eq!(product.capabilities.len(), 1);
    assert_eq!(product.capabilities[0].capability_code, "identity.auth");
    assert_eq!(product.capabilities[0].contract_version, "v1");

    let by_domain = catalog
        .resolve_active_domain(" SHOP.EXAMPLE.TEST. ")
        .await
        .expect("domain resolution must succeed")
        .expect("active verified domain must resolve");
    assert_eq!(by_domain.product_id, product_id);

    let configuration = catalog
        .get_published_configuration(product_id, 1)
        .await
        .expect("published configuration query must succeed")
        .expect("published configuration version must exist");
    assert_eq!(configuration.schema_version, "product-config.v1");
    assert_eq!(configuration.canonical_json["branding"]["displayName"], "ENSAMBRIA Core");

    assert!(
        catalog
            .resolve_active_domain("unknown.example.test")
            .await
            .expect("unknown domain lookup must succeed")
            .is_none()
    );
    pool.close().await;
}
