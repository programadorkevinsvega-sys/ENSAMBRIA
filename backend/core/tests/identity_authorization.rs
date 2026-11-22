use std::{str::FromStr, time::Duration};

use ensambria_core::{
    core_migrations::run_core_migrations_from_path,
    postgres::{
        ActorContext, ActorId, PgPoolConfiguration, TenantContext, TenantId, create_pool,
    },
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

#[tokio::test]
async fn authorization_and_regional_resolution_are_explicit_and_scope_aware() {
    let pool = test_pool().await;
    let actor = Uuid::now_v7();
    let tenant = Uuid::now_v7();
    let organization = Uuid::now_v7();
    let branch = Uuid::now_v7();
    let membership = Uuid::now_v7();
    let role = Uuid::now_v7();
    let email = format!("{actor}@example.test");
    let mut tx = pool.begin().await.expect("transaction must begin");

    ActorContext::new(ActorId::new(actor))
        .establish(&mut tx)
        .await
        .expect("actor context must be established");
    TenantContext::new(TenantId::new(tenant))
        .establish(&mut tx)
        .await
        .expect("tenant context must be established");

    sqlx::query(
        "INSERT INTO core.identities (id, email_normalized, email_display, status, email_verified_at) VALUES ($1, $2, $2, 'active', clock_timestamp())",
    )
    .bind(actor)
    .bind(&email)
    .execute(&mut *tx)
    .await
    .expect("identity must be created");
    sqlx::query(
        "INSERT INTO core.identity_credentials (identity_id, password_hash) VALUES ($1, '$argon2id$v=19$m=19456,t=2,p=1$c2FsdA$aGFzaA')",
    )
    .bind(actor)
    .execute(&mut *tx)
    .await
    .expect("Argon2id verifier must be persisted");

    sqlx::query(
        "INSERT INTO core.tenants (id, status, country_code, currency_code, locale, time_zone) VALUES ($1, 'active', 'CO', 'COP', 'es-CO', 'America/Bogota')",
    )
    .bind(tenant)
    .execute(&mut *tx)
    .await
    .expect("tenant must be created");
    sqlx::query(
        "INSERT INTO core.organizations (id, tenant_id, display_name, country_code, currency_code, locale, time_zone) VALUES ($1, $2, 'Organization', 'MX', 'MXN', 'es-MX', 'America/Mexico_City')",
    )
    .bind(organization)
    .bind(tenant)
    .execute(&mut *tx)
    .await
    .expect("organization must be created");
    sqlx::query(
        "INSERT INTO core.branches (id, tenant_id, organization_id, display_name, country_code, currency_code, locale, time_zone) VALUES ($1, $2, $3, 'Branch', 'ES', 'EUR', 'es-ES', 'Europe/Madrid')",
    )
    .bind(branch)
    .bind(tenant)
    .bind(organization)
    .execute(&mut *tx)
    .await
    .expect("branch must be created");
    sqlx::query(
        "INSERT INTO core.tenant_memberships (id, tenant_id, identity_id, status) VALUES ($1, $2, $3, 'active')",
    )
    .bind(membership)
    .bind(tenant)
    .bind(actor)
    .execute(&mut *tx)
    .await
    .expect("membership must be created");
    sqlx::query(
        "INSERT INTO core.tenant_roles (id, tenant_id, code, display_name) VALUES ($1, $2, 'branch_operator', 'Branch operator')",
    )
    .bind(role)
    .bind(tenant)
    .execute(&mut *tx)
    .await
    .expect("role must be created");
    sqlx::query(
        "INSERT INTO core.tenant_role_permissions (tenant_id, role_id, permission_code) VALUES ($1, $2, 'branch.manage')",
    )
    .bind(tenant)
    .bind(role)
    .execute(&mut *tx)
    .await
    .expect("permission must be assigned to role");
    sqlx::query(
        "INSERT INTO core.membership_role_assignments (id, tenant_id, membership_id, role_id, scope_type, organization_id, branch_id) VALUES ($1, $2, $3, $4, 'branch', $5, $6)",
    )
    .bind(Uuid::now_v7())
    .bind(tenant)
    .bind(membership)
    .bind(role)
    .bind(organization)
    .bind(branch)
    .execute(&mut *tx)
    .await
    .expect("role must be assigned at branch scope");

    let allowed: bool = sqlx::query_scalar(
        "SELECT core.has_permission($1, $2, 'branch.manage', $3, $4)",
    )
    .bind(actor)
    .bind(tenant)
    .bind(organization)
    .bind(branch)
    .fetch_one(&mut *tx)
    .await
    .expect("authorization evaluator must execute");
    let denied_wrong_branch: bool = sqlx::query_scalar(
        "SELECT core.has_permission($1, $2, 'branch.manage', $3, $4)",
    )
    .bind(actor)
    .bind(tenant)
    .bind(organization)
    .bind(Uuid::now_v7())
    .fetch_one(&mut *tx)
    .await
    .expect("authorization evaluator must execute for another branch");
    let denied_organization_scope: bool = sqlx::query_scalar(
        "SELECT core.has_permission($1, $2, 'branch.manage', $3, NULL)",
    )
    .bind(actor)
    .bind(tenant)
    .bind(organization)
    .fetch_one(&mut *tx)
    .await
    .expect("branch-scoped permission must not imply organization scope");
    let denied_tenant_scope: bool = sqlx::query_scalar(
        "SELECT core.has_permission($1, $2, 'branch.manage', NULL, NULL)",
    )
    .bind(actor)
    .bind(tenant)
    .fetch_one(&mut *tx)
    .await
    .expect("branch-scoped permission must not imply tenant scope");
    assert!(allowed);
    assert!(!denied_wrong_branch);
    assert!(!denied_organization_scope);
    assert!(!denied_tenant_scope);

    let organization_regional: (Option<String>, Option<String>, Option<String>, Option<String>) =
        sqlx::query_as(
            "SELECT country_code::text, currency_code::text, locale, time_zone FROM core.resolve_regional_context($1, $2, NULL)",
        )
        .bind(tenant)
        .bind(organization)
        .fetch_one(&mut *tx)
        .await
        .expect("organization regional context must resolve");
    assert_eq!(
        organization_regional,
        (
            Some("MX".to_owned()),
            Some("MXN".to_owned()),
            Some("es-MX".to_owned()),
            Some("America/Mexico_City".to_owned())
        )
    );
    let tenant_regional: (Option<String>, Option<String>, Option<String>, Option<String>) =
        sqlx::query_as(
            "SELECT country_code::text, currency_code::text, locale, time_zone FROM core.resolve_regional_context($1, NULL, NULL)",
        )
        .bind(tenant)
        .fetch_one(&mut *tx)
        .await
        .expect("tenant regional context must resolve");
    assert_eq!(
        tenant_regional,
        (
            Some("CO".to_owned()),
            Some("COP".to_owned()),
            Some("es-CO".to_owned()),
            Some("America/Bogota".to_owned())
        )
    );

    let regional: (Option<String>, Option<String>, Option<String>, Option<String>) =
        sqlx::query_as(
            "SELECT country_code::text, currency_code::text, locale, time_zone FROM core.resolve_regional_context($1, $2, $3)",
        )
        .bind(tenant)
        .bind(organization)
        .bind(branch)
        .fetch_one(&mut *tx)
        .await
        .expect("regional context must resolve");
    assert_eq!(
        regional,
        (
            Some("ES".to_owned()),
            Some("EUR".to_owned()),
            Some("es-ES".to_owned()),
            Some("Europe/Madrid".to_owned())
        )
    );

    tx.commit().await.expect("authorization fixture must commit");

    let lookup: Option<(Uuid, String)> = sqlx::query_as(
        "SELECT identity_id, password_hash FROM core.auth_lookup_identity($1)",
    )
    .bind(&email)
    .fetch_optional(&pool)
    .await
    .expect("pre-auth identity lookup must be available through the definer function");
    assert_eq!(
        lookup,
        Some((
            actor,
            "$argon2id$v=19$m=19456,t=2,p=1$c2FsdA$aGFzaA".to_owned()
        ))
    );

    let denied_without_context: bool = sqlx::query_scalar(
        "SELECT core.has_permission($1, $2, 'branch.manage', $3, $4)",
    )
    .bind(actor)
    .bind(tenant)
    .bind(organization)
    .bind(branch)
    .fetch_one(&pool)
    .await
    .expect("authorization must fail closed without transaction context");
    assert!(!denied_without_context);
    let regional_without_context: Option<(Option<String>, Option<String>, Option<String>, Option<String>)> =
        sqlx::query_as(
            "SELECT country_code::text, currency_code::text, locale, time_zone FROM core.resolve_regional_context($1, $2, $3)",
        )
        .bind(tenant)
        .bind(organization)
        .bind(branch)
        .fetch_optional(&pool)
        .await
        .expect("regional resolution must fail closed without tenant context");
    assert_eq!(regional_without_context, None);

    let direct_credentials: i64 =
        sqlx::query_scalar("SELECT count(*) FROM core.identity_credentials")
            .fetch_one(&pool)
            .await
            .expect("pre-auth direct credential reads must remain blocked by RLS");
    assert_eq!(direct_credentials, 0);
    pool.close().await;
}
