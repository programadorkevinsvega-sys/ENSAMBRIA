use ensambria_core::core_migrations::run_core_migrations_from_path;

#[tokio::test]
async fn concurrent_migration_invocations_are_serialized_and_idempotent() {
    let database_url = std::env::var("CORE_TEST_MIGRATOR_DATABASE_URL")
        .expect("isolated PostgreSQL migration URL must be configured");
    let migrations = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("migrations");
    let (first, second) = tokio::join!(
        run_core_migrations_from_path(&database_url, &migrations),
        run_core_migrations_from_path(&database_url, &migrations)
    );
    first.expect("first concurrent migration must succeed");
    second.expect("second concurrent migration must observe the completed migration set");
}
