use std::{str::FromStr, time::Duration};

use ensambria_core::{
    core_migrations::run_core_migrations_from_path,
    identity_authentication::crypto::password::{hash_password, verify_password, PasswordHash},
    postgres::{ActorContext, ActorId, PgPoolConfiguration, create_pool},
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

async fn set_actor(transaction: &mut sqlx::Transaction<'_, sqlx::Postgres>, actor: Uuid) {
    ActorContext::new(ActorId::new(actor))
        .establish(transaction)
        .await
        .expect("authenticated actor context must be established");
}

#[tokio::test]
async fn credentials_and_sessions_are_actor_scoped_and_pre_auth_lookup_is_narrow() {
    let pool = test_pool().await;
    let actor = Uuid::now_v7();
    let other_actor = Uuid::now_v7();
    let identity_id = actor;
    let session_id = Uuid::now_v7();
    let token_hash = vec![0x77; 32];
    let email = format!("{identity_id}@example.test");
    let stored_hash = hash_password("correct horse battery staple").expect("Argon2id hash");
    let mut transaction = pool.begin().await.expect("transaction must begin");
    set_actor(&mut transaction, actor).await;

    sqlx::query(
        "INSERT INTO core.identities (id, email_normalized, email_display, status, email_verified_at) VALUES ($1, $2, $2, 'active', clock_timestamp())",
    )
    .bind(identity_id)
    .bind(&email)
    .execute(&mut *transaction)
    .await
    .expect("identity must be inserted under its actor context");
    sqlx::query("INSERT INTO core.identity_credentials (identity_id, password_hash) VALUES ($1, $2)")
        .bind(identity_id)
        .bind(stored_hash.as_str())
        .execute(&mut *transaction)
        .await
        .expect("Argon2id credential must be inserted under its actor context");
    sqlx::query(
        "INSERT INTO core.sessions (id, identity_id, expires_at) VALUES ($1, $2, clock_timestamp() + interval '1 day')",
    )
    .bind(session_id)
    .bind(identity_id)
    .execute(&mut *transaction)
    .await
    .expect("session must be inserted under its actor context");
    sqlx::query(
        "INSERT INTO core.session_token_lookup (token_hash, session_id, identity_id) VALUES ($1, $2, $3)",
    )
    .bind(&token_hash)
    .bind(session_id)
    .bind(identity_id)
    .execute(&mut *transaction)
    .await
    .expect("opaque session lookup must be insertable");
    transaction
        .commit()
        .await
        .expect("identity transaction must commit");

    let lookup: Option<(Uuid, String)> = sqlx::query_as(
        "SELECT identity_id, password_hash FROM core.auth_lookup_identity($1)",
    )
    .bind(&email)
    .fetch_optional(&pool)
    .await
    .expect("narrow pre-auth lookup must be executable");
    let (found_identity, phc) = lookup.expect("active verified identity must be found");
    assert_eq!(found_identity, identity_id);
    let restored = PasswordHash::from_phc(&phc).expect("PHC hash must be valid");
    assert!(verify_password("correct horse battery staple", &restored).expect("verify password"));

    let mut transaction = pool.begin().await.expect("second transaction must begin");
    set_actor(&mut transaction, other_actor).await;
    let visible_credentials: i64 =
        sqlx::query_scalar("SELECT count(*) FROM core.identity_credentials WHERE identity_id = $1")
            .bind(identity_id)
            .fetch_one(&mut *transaction)
            .await
            .expect("credential lookup must be scoped");
    let visible_sessions: i64 =
        sqlx::query_scalar("SELECT count(*) FROM core.sessions WHERE identity_id = $1")
            .bind(identity_id)
            .fetch_one(&mut *transaction)
            .await
            .expect("session lookup must be scoped");
    assert_eq!(visible_credentials, 0);
    assert_eq!(visible_sessions, 0);
    transaction
        .rollback()
        .await
        .expect("test transaction must roll back");

    let direct_credentials: i64 =
        sqlx::query_scalar("SELECT count(*) FROM core.identity_credentials")
            .fetch_one(&pool)
            .await
            .expect("pre-auth direct credential reads must remain blocked by RLS");
    assert_eq!(direct_credentials, 0);
    pool.close().await;
}
