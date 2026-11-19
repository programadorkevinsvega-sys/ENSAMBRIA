use std::{str::FromStr, time::Duration};

use ensambria_core::postgres::{PgPoolConfiguration, TenantContext, TenantId, create_pool};
use sqlx::{PgPool, postgres::PgConnectOptions, types::Uuid};

const TEST_TENANT_ID: &str = "018f4e3a-6b2c-7a1d-8f56-9a7b3c2d1e00";

async fn test_pool() -> PgPool {
    test_pool_with_timeout(Duration::from_secs(3)).await
}

async fn test_pool_with_timeout(acquire_timeout: Duration) -> PgPool {
    let database_url = std::env::var("CORE_TEST_DATABASE_URL")
        .expect("CORE_TEST_DATABASE_URL must point to an isolated PostgreSQL test database");
    let connect_options = PgConnectOptions::from_str(&database_url)
        .unwrap_or_else(|_| panic!("CORE_TEST_DATABASE_URL must be a valid PostgreSQL URL"));
    let configuration = PgPoolConfiguration::new(
        1,
        acquire_timeout,
        Some(Duration::from_secs(30)),
        Some(Duration::from_secs(300)),
    )
    .expect("integration-test pool settings must be valid");

    create_pool(connect_options, configuration)
        .await
        .expect("the isolated PostgreSQL test database must be available")
}

async fn current_tenant(pool: &PgPool) -> Option<String> {
    sqlx::query_scalar("SELECT NULLIF(current_setting('app.tenant_id', true), '')")
        .fetch_one(pool)
        .await
        .expect("the transaction-local tenant setting must be readable")
}

#[tokio::test]
async fn pool_acquisition_times_out_when_its_only_connection_is_checked_out() {
    let pool = test_pool_with_timeout(Duration::from_millis(100)).await;
    let held_connection = pool.acquire().await.expect("connection must be acquired");

    let result = pool.acquire().await;
    assert!(matches!(result, Err(sqlx::Error::PoolTimedOut)));

    drop(held_connection);
    pool.close().await;
}

#[tokio::test]
async fn integration_connection_uses_a_non_privileged_database_role() {
    let pool = test_pool().await;
    let (superuser, can_create_database, can_create_roles): (bool, bool, bool) = sqlx::query_as(
        "SELECT rolsuper, rolcreatedb, rolcreaterole \
             FROM pg_catalog.pg_roles \
             WHERE rolname = current_user",
    )
    .fetch_one(&pool)
    .await
    .expect("the connected PostgreSQL role must exist");

    assert!(!superuser);
    assert!(!can_create_database);
    assert!(!can_create_roles);
    pool.close().await;
}

#[tokio::test]
async fn tenant_context_is_cleared_after_commit_when_a_connection_is_reused() {
    let pool = test_pool().await;
    let tenant_id = TenantId::new(
        Uuid::parse_str(TEST_TENANT_ID).expect("the integration tenant ID must be valid UUIDv7"),
    );
    let mut transaction = pool.begin().await.expect("transaction must begin");

    TenantContext::new(tenant_id)
        .establish(&mut transaction)
        .await
        .expect("tenant context must be set within the transaction");

    let in_transaction: Option<String> =
        sqlx::query_scalar("SELECT current_setting('app.tenant_id', true)")
            .fetch_one(&mut *transaction)
            .await
            .expect("tenant context must be visible within its transaction");
    assert_eq!(in_transaction, Some(tenant_id.as_uuid().to_string()));

    transaction.commit().await.expect("transaction must commit");
    assert_eq!(current_tenant(&pool).await, None);
    pool.close().await;
}

#[tokio::test]
async fn tenant_context_is_cleared_after_rollback_when_a_connection_is_reused() {
    let pool = test_pool().await;
    let tenant_id = TenantId::new(
        Uuid::parse_str(TEST_TENANT_ID).expect("the integration tenant ID must be valid UUIDv7"),
    );
    let mut transaction = pool.begin().await.expect("transaction must begin");

    TenantContext::new(tenant_id)
        .establish(&mut transaction)
        .await
        .expect("tenant context must be set within the transaction");

    transaction
        .rollback()
        .await
        .expect("transaction must roll back");
    assert_eq!(current_tenant(&pool).await, None);
    pool.close().await;
}
