use std::{str::FromStr, time::Duration};

use ensambria_core::{
    core_migrations::run_core_migrations_from_path,
    postgres::{ActorContext, ActorId, PgPoolConfiguration, TenantContext, TenantId, create_pool},
};
use sqlx::{PgPool, postgres::PgConnectOptions, types::Uuid};

async fn test_pool() -> PgPool {
    let migration_url = std::env::var("CORE_TEST_MIGRATOR_DATABASE_URL")
        .expect("isolated PostgreSQL migration URL must be configured");
    let migrations = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("migrations");
    run_core_migrations_from_path(&migration_url, &migrations)
        .await
        .expect("CORE migrations must apply with the restricted migration identity");

    let database_url = std::env::var("CORE_TEST_DATABASE_URL")
        .expect("isolated PostgreSQL runtime URL must be configured");
    let options =
        PgConnectOptions::from_str(&database_url).expect("isolated PostgreSQL URL must be valid");
    let config = PgPoolConfiguration::new(
        1,
        Duration::from_secs(3),
        Some(Duration::from_secs(30)),
        Some(Duration::from_secs(300)),
    )
    .expect("integration pool settings must be valid");
    create_pool(options, config)
        .await
        .expect("non-privileged PostgreSQL runtime must connect")
}

async fn tenancy_pool() -> PgPool {
    let migration_url = std::env::var("CORE_TEST_MIGRATOR_DATABASE_URL")
        .expect("isolated PostgreSQL migration URL must be configured");
    let migrations = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("migrations");
    run_core_migrations_from_path(&migration_url, &migrations)
        .await
        .expect("CORE migrations must apply with the restricted migration identity");

    let database_url = std::env::var("CORE_TENANT_DATABASE_URL")
        .expect("isolated PostgreSQL tenancy runtime URL must be configured");
    let options =
        PgConnectOptions::from_str(&database_url).expect("isolated PostgreSQL URL must be valid");
    let config = PgPoolConfiguration::new(
        1,
        Duration::from_secs(3),
        Some(Duration::from_secs(30)),
        Some(Duration::from_secs(300)),
    )
    .expect("integration pool settings must be valid");
    create_pool(options, config)
        .await
        .expect("non-privileged PostgreSQL tenancy runtime must connect")
}

async fn set_tenant(transaction: &mut sqlx::Transaction<'_, sqlx::Postgres>, tenant_id: Uuid) {
    TenantContext::new(TenantId::new(tenant_id))
        .establish(transaction)
        .await
        .expect("tenant context must be established transaction-locally");
}

async fn set_actor(transaction: &mut sqlx::Transaction<'_, sqlx::Postgres>, actor_id: Uuid) {
    ActorContext::new(ActorId::new(actor_id))
        .establish(transaction)
        .await
        .expect("actor context must be established transaction-locally");
}

#[tokio::test]
async fn rls_denies_cross_tenant_reads_writes_and_cross_tenant_relationships() {
    let pool = tenancy_pool().await;
    let tenant_a = Uuid::now_v7();
    let tenant_b = Uuid::now_v7();
    let org_a = Uuid::now_v7();
    let org_b = Uuid::now_v7();
    let mut transaction = pool.begin().await.expect("transaction must begin");

    set_tenant(&mut transaction, tenant_a).await;
    sqlx::query("INSERT INTO core_tenancy.tenants (id) VALUES ($1)")
        .bind(tenant_a)
        .execute(&mut *transaction)
        .await
        .expect("tenant A must be insertable within its own scope");
    sqlx::query(
        "INSERT INTO core_tenancy.organizations (id, tenant_id, display_name) VALUES ($1, $2, 'A')",
    )
    .bind(org_a)
    .bind(tenant_a)
    .execute(&mut *transaction)
    .await
    .expect("organization A must be insertable within its own scope");

    set_tenant(&mut transaction, tenant_b).await;
    sqlx::query("INSERT INTO core_tenancy.tenants (id) VALUES ($1)")
        .bind(tenant_b)
        .execute(&mut *transaction)
        .await
        .expect("tenant B must be insertable within its own scope");
    sqlx::query(
        "INSERT INTO core_tenancy.organizations (id, tenant_id, display_name) VALUES ($1, $2, 'B')",
    )
    .bind(org_b)
    .bind(tenant_b)
    .execute(&mut *transaction)
    .await
    .expect("organization B must be insertable within its own scope");

    set_tenant(&mut transaction, tenant_a).await;
    let visible: i64 =
        sqlx::query_scalar("SELECT count(*) FROM core_tenancy.tenants WHERE id = $1")
            .bind(tenant_b)
            .fetch_one(&mut *transaction)
            .await
            .expect("cross-tenant reads must return an empty result");
    assert_eq!(visible, 0);

    let updated = sqlx::query(
        "UPDATE core_tenancy.organizations SET display_name = 'tampered' WHERE id = $1",
    )
    .bind(org_b)
    .execute(&mut *transaction)
    .await
    .expect("cross-tenant updates must not affect rows");
    assert_eq!(updated.rows_affected(), 0);

    let deleted = sqlx::query("DELETE FROM core_tenancy.organizations WHERE id = $1")
        .bind(org_b)
        .execute(&mut *transaction)
        .await
        .expect("cross-tenant deletes must not affect rows");
    assert_eq!(deleted.rows_affected(), 0);

    let forged = sqlx::query(
        "INSERT INTO core_tenancy.organizations (id, tenant_id, display_name) VALUES ($1, $2, 'forged')",
    )
    .bind(Uuid::now_v7())
    .bind(tenant_b)
    .execute(&mut *transaction)
    .await;
    assert!(forged.is_err(), "forged tenant writes must be rejected");

    let cross_org = sqlx::query(
        "INSERT INTO core_tenancy.branches (id, tenant_id, organization_id, display_name) VALUES ($1, $2, $3, 'invalid')",
    )
    .bind(Uuid::now_v7())
    .bind(tenant_a)
    .bind(org_b)
    .execute(&mut *transaction)
    .await;
    assert!(
        cross_org.is_err(),
        "composite tenant/organization foreign keys must reject cross-tenant references"
    );

    transaction
        .rollback()
        .await
        .expect("test transaction must roll back");
    pool.close().await;
}

#[tokio::test]
async fn actor_rls_and_tenant_context_do_not_leak_after_commit_or_rollback() {
    let pool = test_pool().await;
    let tenancy_pool = tenancy_pool().await;
    let actor_b = Uuid::now_v7();
    let tenant_a = Uuid::now_v7();
    let identity_id = Uuid::now_v7();
    let mut transaction = pool.begin().await.expect("identity transaction must begin");

    set_actor(&mut transaction, identity_id).await;
    sqlx::query(
        "INSERT INTO core_identity_auth.identities (id, email_normalized, email_display) VALUES ($1, 'a@example.test', 'a@example.test')",
    )
    .bind(identity_id)
    .execute(&mut *transaction)
    .await
    .expect("actor A may create its own identity context");
    transaction
        .commit()
        .await
        .expect("identity transaction must commit");

    let mut tenancy_transaction = tenancy_pool
        .begin()
        .await
        .expect("tenancy transaction must begin");
    set_tenant(&mut tenancy_transaction, tenant_a).await;
    sqlx::query("INSERT INTO core_tenancy.tenants (id) VALUES ($1)")
        .bind(tenant_a)
        .execute(&mut *tenancy_transaction)
        .await
        .expect("tenant must be created by the tenancy runtime in its own scope");
    tenancy_transaction
        .commit()
        .await
        .expect("tenancy transaction must commit");

    let mut transaction = pool
        .begin()
        .await
        .expect("second identity transaction must begin");
    set_actor(&mut transaction, actor_b).await;
    let visible: i64 =
        sqlx::query_scalar("SELECT count(*) FROM core_identity_auth.identities WHERE id = $1")
            .bind(identity_id)
            .fetch_one(&mut *transaction)
            .await
            .expect("another actor must not read this identity");
    assert_eq!(visible, 0);
    let forged = sqlx::query(
        "INSERT INTO core_identity_auth.identities (id, email_normalized, email_display) VALUES ($1, 'b@example.test', 'b@example.test')",
    )
    .bind(identity_id)
    .execute(&mut *transaction)
    .await;
    assert!(
        forged.is_err(),
        "an actor cannot create another actor's identity"
    );
    transaction
        .rollback()
        .await
        .expect("second transaction must roll back");

    let actor_context: Option<String> =
        sqlx::query_scalar("SELECT NULLIF(current_setting('app.actor_id', true), '')")
            .fetch_one(&pool)
            .await
            .expect("actor context setting must be readable");
    let tenant_context: Option<String> =
        sqlx::query_scalar("SELECT NULLIF(current_setting('app.tenant_id', true), '')")
            .fetch_one(&tenancy_pool)
            .await
            .expect("tenancy runtime context setting must be readable");
    assert_eq!(actor_context, None);
    assert_eq!(tenant_context, None);
    pool.close().await;
    tenancy_pool.close().await;
}
