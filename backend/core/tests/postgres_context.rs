use std::{
    io::{Read, Write},
    net::{SocketAddr, TcpListener as StdTcpListener, TcpStream},
    process::{Child, Command, Stdio},
    str::FromStr,
    time::Duration,
};

use ensambria_core::{
    core_migrations::run_core_migrations_from_path,
    health::{
        LIVENESS_SERVICE, READINESS_SERVICE, check_database, initialize_health_status,
        monitor_database_readiness,
    },
    postgres::{PgPoolConfiguration, TenantContext, TenantId, create_pool},
};
use sqlx::{PgPool, postgres::PgConnectOptions, types::Uuid};
use tokio::{
    net::TcpListener,
    sync::{oneshot, watch},
    time::{sleep, timeout},
};
use tokio_stream::wrappers::TcpListenerStream;
use tonic::transport::{Endpoint, Server};
use tonic_health::pb::{
    HealthCheckRequest, health_check_response::ServingStatus as HealthServingStatus,
    health_client::HealthClient,
};
use tonic_health::{ServingStatus, server::health_reporter};

const TEST_TENANT_ID: &str = "018f4e3a-6b2c-7a1d-8f56-9a7b3c2d1e00";

struct CoreProcessGuard {
    child: Child,
}

impl CoreProcessGuard {
    fn child(&mut self) -> &mut Child {
        &mut self.child
    }

    fn read_logs(&mut self) -> (String, String) {
        let mut stdout = String::new();
        let mut stderr = String::new();
        self.child
            .stdout
            .take()
            .expect("CORE process stdout must be captured")
            .read_to_string(&mut stdout)
            .expect("CORE process stdout must be readable");
        self.child
            .stderr
            .take()
            .expect("CORE process stderr must be captured")
            .read_to_string(&mut stderr)
            .expect("CORE process stderr must be readable");
        (stdout, stderr)
    }
}

impl Drop for CoreProcessGuard {
    fn drop(&mut self) {
        match self.child.try_wait() {
            Ok(Some(status)) => {
                if !status.success() {
                    eprintln!("CORE test process exited unsuccessfully: {status}");
                }
            }
            Ok(None) => {
                if let Err(error) = self.child.kill() {
                    eprintln!("failed to stop CORE test process: {error}");
                }
                if let Err(error) = self.child.wait() {
                    eprintln!("failed to wait for CORE test process: {error}");
                }
            }
            Err(error) => eprintln!("failed to inspect CORE test process: {error}"),
        }
    }
}

fn reserve_loopback_address() -> SocketAddr {
    StdTcpListener::bind("127.0.0.1:0")
        .expect("a loopback port must be available")
        .local_addr()
        .expect("the reserved loopback listener must expose its address")
}

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
async fn migrations_use_restricted_identity_and_module_runtime_cannot_own_or_cross_access_schemas()
{
    let migration_database_url = std::env::var("CORE_TEST_MIGRATOR_DATABASE_URL").expect(
        "CORE_TEST_MIGRATOR_DATABASE_URL must point to an isolated PostgreSQL test database",
    );
    let migrations_path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("migrations");
    run_core_migrations_from_path(&migration_database_url, &migrations_path)
        .await
        .expect("CORE migrations must apply with the dedicated migration identity");
    run_core_migrations_from_path(&migration_database_url, &migrations_path)
        .await
        .expect("CORE migrations must be safe to run again");

    let pool = test_pool().await;
    let schema_owners: Vec<(String, String)> = sqlx::query_as(
        r#"SELECT namespace.nspname::text,
                  pg_catalog.pg_get_userbyid(namespace.nspowner)::text
           FROM pg_catalog.pg_namespace AS namespace
           WHERE namespace.nspname IN ('core_identity_auth', 'core_tenancy')
           ORDER BY namespace.nspname"#,
    )
    .fetch_all(&pool)
    .await
    .expect("CORE module schema ownership must be inspectable");

    assert_eq!(
        schema_owners,
        vec![
            (
                "core_identity_auth".to_owned(),
                "core_identity_auth_schema_owner".to_owned(),
            ),
            (
                "core_tenancy".to_owned(),
                "core_tenancy_schema_owner".to_owned(),
            ),
        ],
        "each CORE module schema must belong to its dedicated non-login owner",
    );

    let (
        can_use_identity_schema,
        can_create_in_identity_schema,
        can_read_migration_table,
        can_use_tenancy_schema,
    ): (bool, bool, bool, bool) = sqlx::query_as(
        r#"SELECT has_schema_privilege(current_user, 'core_identity_auth', 'USAGE'),
                  has_schema_privilege(current_user, 'core_identity_auth', 'CREATE'),
                  COALESCE(
                      (
                          SELECT has_table_privilege(current_user, relation.oid, 'SELECT')
                          FROM pg_catalog.pg_class AS relation
                          JOIN pg_catalog.pg_namespace AS namespace
                            ON namespace.oid = relation.relnamespace
                          WHERE namespace.nspname = 'core_tenancy'
                            AND relation.relname = '_sqlx_migrations'
                      ),
                      false
                  ),
                  has_schema_privilege(current_user, 'core_tenancy', 'USAGE')"#,
    )
    .fetch_one(&pool)
    .await
    .expect("CORE runtime module grants must be inspectable");

    assert!(
        can_use_identity_schema,
        "identity runtime needs its own schema"
    );
    assert!(
        !can_create_in_identity_schema,
        "identity runtime must not create schema objects"
    );
    assert!(
        !can_read_migration_table,
        "runtime must not inspect the migration ledger"
    );
    assert!(
        !can_use_tenancy_schema,
        "identity runtime must not access tenancy schema"
    );

    assert!(
        sqlx::query("CREATE TABLE core_identity_auth.runtime_must_not_create (id integer)")
            .execute(&pool)
            .await
            .is_err(),
        "identity runtime must not create objects in its module schema"
    );
    assert!(
        sqlx::query("SELECT * FROM core_tenancy.tenants LIMIT 1")
            .fetch_all(&pool)
            .await
            .is_err(),
        "identity runtime must not read tenancy-owned tables"
    );
    assert!(
        sqlx::query("SET ROLE core_identity_auth_schema_owner")
            .execute(&pool)
            .await
            .is_err(),
        "identity runtime must not assume its schema owner role"
    );
    pool.close().await;
}

#[tokio::test]
async fn readiness_database_check_has_an_independent_timeout() {
    let pool = test_pool().await;
    let held_connection = pool.acquire().await.expect("connection must be acquired");

    let result = check_database(&pool, Duration::from_millis(20)).await;
    assert!(
        result
            .expect_err("readiness must time out while the pool connection is occupied")
            .to_string()
            .contains("timeout")
    );

    drop(held_connection);
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

#[tokio::test]
async fn grpc_liveness_stays_serving_while_postgres_readiness_recovers_and_fails() {
    let pool = test_pool().await;
    let (reporter, health_service) = health_reporter();
    initialize_health_status(&reporter).await;
    let listener = TcpListener::bind("127.0.0.1:0")
        .await
        .expect("health listener must bind to loopback");
    let address = listener
        .local_addr()
        .expect("health listener must expose its bound address");
    let (shutdown_sender, shutdown_receiver) = oneshot::channel();
    let server = Server::builder()
        .add_service(health_service)
        .serve_with_incoming_shutdown(TcpListenerStream::new(listener), async {
            let _ = shutdown_receiver.await;
        });
    let server_task = tokio::spawn(server);
    let (monitor_shutdown_sender, monitor_shutdown_receiver) = watch::channel(false);
    let monitor_task = tokio::spawn(monitor_database_readiness(
        pool.clone(),
        reporter.clone(),
        Duration::from_millis(20),
        Duration::from_secs(1),
        monitor_shutdown_receiver,
    ));
    let channel = Endpoint::from_shared(format!("http://{address}"))
        .expect("health endpoint must be a valid URI")
        .connect()
        .await
        .expect("health gRPC client must connect");
    let mut client = HealthClient::new(channel);

    let liveness = client
        .check(HealthCheckRequest {
            service: LIVENESS_SERVICE.to_owned(),
        })
        .await
        .expect("liveness check must succeed while readiness is pending")
        .into_inner();
    assert_eq!(
        HealthServingStatus::try_from(liveness.status)
            .expect("liveness must return a known gRPC health status"),
        HealthServingStatus::Serving
    );

    let readiness = timeout(Duration::from_secs(2), async {
        loop {
            let response = client
                .check(HealthCheckRequest {
                    service: READINESS_SERVICE.to_owned(),
                })
                .await
                .expect("readiness check must succeed");
            if HealthServingStatus::try_from(response.into_inner().status)
                .is_ok_and(|status| status == HealthServingStatus::Serving)
            {
                break;
            }
            sleep(Duration::from_millis(10)).await;
        }
    })
    .await;
    assert!(
        readiness.is_ok(),
        "PostgreSQL readiness must become serving"
    );

    pool.close().await;
    let readiness = timeout(Duration::from_secs(2), async {
        loop {
            let response = client
                .check(HealthCheckRequest {
                    service: READINESS_SERVICE.to_owned(),
                })
                .await;
            if response.is_ok_and(|response| {
                HealthServingStatus::try_from(response.into_inner().status)
                    .is_ok_and(|status| status == HealthServingStatus::NotServing)
            }) {
                break;
            }
            sleep(Duration::from_millis(10)).await;
        }
    })
    .await;
    assert!(
        readiness.is_ok(),
        "readiness must fail after PostgreSQL closes"
    );

    let liveness = client
        .check(HealthCheckRequest {
            service: LIVENESS_SERVICE.to_owned(),
        })
        .await
        .expect("liveness must remain available when PostgreSQL is unavailable")
        .into_inner();
    assert_eq!(
        HealthServingStatus::try_from(liveness.status)
            .expect("liveness must return a known gRPC health status"),
        HealthServingStatus::Serving
    );

    monitor_shutdown_sender
        .send(true)
        .expect("readiness monitor must accept shutdown");
    reporter
        .set_service_status(READINESS_SERVICE, ServingStatus::NotServing)
        .await;
    shutdown_sender
        .send(())
        .expect("health server must accept shutdown");
    monitor_task
        .await
        .expect("readiness monitor must stop cleanly");
    server_task
        .await
        .expect("health server task must complete")
        .expect("health server must stop cleanly");
}

#[tokio::test]
async fn core_process_serves_health_and_metrics_then_stops_cleanly() {
    let database_url = std::env::var("CORE_TEST_DATABASE_URL")
        .expect("CORE_TEST_DATABASE_URL must point to an isolated PostgreSQL test database");
    let health_address = reserve_loopback_address();
    let metrics_address = reserve_loopback_address();
    let mut process = CoreProcessGuard {
        child: Command::new(env!("CARGO_BIN_EXE_core"))
            .env("DATABASE_URL", &database_url)
            .env("CORE_DATABASE_MAX_CONNECTIONS", "1")
            .env("CORE_DATABASE_ACQUIRE_TIMEOUT_MS", "1000")
            .env("CORE_HEALTH_LISTEN_ADDRESS", health_address.to_string())
            .env("CORE_HEALTH_CHECK_INTERVAL_MS", "50")
            .env("CORE_HEALTH_CHECK_TIMEOUT_MS", "500")
            .env("CORE_METRICS_LISTEN_ADDRESS", metrics_address.to_string())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .expect("CORE process must start"),
    };

    let health_endpoint = format!("http://{health_address}");
    let channel = timeout(Duration::from_secs(5), async {
        loop {
            match Endpoint::from_shared(health_endpoint.clone())
                .expect("health endpoint must be a valid URI")
                .connect()
                .await
            {
                Ok(channel) => break channel,
                Err(_) => sleep(Duration::from_millis(20)).await,
            }
        }
    })
    .await
    .expect("CORE health listener must become available");
    let mut client = HealthClient::new(channel);

    let liveness = client
        .check(HealthCheckRequest {
            service: LIVENESS_SERVICE.to_owned(),
        })
        .await
        .expect("CORE liveness must respond")
        .into_inner();
    assert_eq!(
        HealthServingStatus::try_from(liveness.status)
            .expect("CORE liveness must return a known gRPC health status"),
        HealthServingStatus::Serving
    );

    let readiness = timeout(Duration::from_secs(5), async {
        loop {
            let response = client
                .check(HealthCheckRequest {
                    service: READINESS_SERVICE.to_owned(),
                })
                .await
                .expect("CORE readiness must respond");
            if HealthServingStatus::try_from(response.into_inner().status)
                .is_ok_and(|status| status == HealthServingStatus::Serving)
            {
                break;
            }
            sleep(Duration::from_millis(20)).await;
        }
    })
    .await;
    assert!(readiness.is_ok(), "CORE readiness must observe PostgreSQL");

    let mut metrics_stream =
        TcpStream::connect(metrics_address).expect("CORE metrics listener must accept requests");
    metrics_stream
        .set_read_timeout(Some(Duration::from_secs(3)))
        .expect("metrics response timeout must be configurable");
    metrics_stream
        .write_all(b"GET /metrics HTTP/1.1\r\nHost: localhost\r\nConnection: close\r\n\r\n")
        .expect("metrics request must be written");
    let mut metrics_response = String::new();
    metrics_stream
        .read_to_string(&mut metrics_response)
        .expect("metrics response must be readable");
    assert!(metrics_response.starts_with("HTTP/1.1 200"));
    assert!(metrics_response.contains("ensambria_core_database_health_checks_total"));

    let process_id = process.child().id().to_string();
    let signal_status = Command::new("kill")
        .arg("-TERM")
        .arg(process_id)
        .status()
        .expect("SIGTERM must be deliverable to the CORE test process");
    assert!(signal_status.success(), "CORE process must accept SIGTERM");

    let exit_status = timeout(Duration::from_secs(5), async {
        loop {
            if let Some(status) = process
                .child()
                .try_wait()
                .expect("CORE process status must be inspectable")
            {
                break status;
            }
            sleep(Duration::from_millis(20)).await;
        }
    })
    .await
    .expect("CORE process must stop within the shutdown timeout");
    assert!(exit_status.success(), "CORE process must stop successfully");

    let (stdout, stderr) = process.read_logs();
    assert!(stdout.contains("service.started"));
    assert!(stdout.contains("service.stopped"));
    let readiness_event = stdout
        .lines()
        .find(|line| line.contains("service.readiness.changed"))
        .expect("CORE must log a readiness transition");
    assert!(readiness_event.contains("correlation_id"));
    assert!(readiness_event.contains("trace_id"));
    assert!(!stdout.contains(&database_url));
    assert!(!stderr.contains(&database_url));
}

#[tokio::test]
async fn core_process_correlates_readiness_failure_when_postgres_is_unavailable() {
    let database_listener =
        StdTcpListener::bind("127.0.0.1:0").expect("a PostgreSQL test port must be available");
    let database_address = database_listener
        .local_addr()
        .expect("the PostgreSQL test listener must expose its address");
    let health_listener =
        StdTcpListener::bind("127.0.0.1:0").expect("a health test port must be available");
    let health_address = health_listener
        .local_addr()
        .expect("the health test listener must expose its address");
    let metrics_listener =
        StdTcpListener::bind("127.0.0.1:0").expect("a metrics test port must be available");
    let metrics_address = metrics_listener
        .local_addr()
        .expect("the metrics test listener must expose its address");
    let database_url = format!("postgres://core_runtime@{database_address}/ensambria_core_test");
    drop(database_listener);
    drop(health_listener);
    drop(metrics_listener);
    let mut process = CoreProcessGuard {
        child: Command::new(env!("CARGO_BIN_EXE_core"))
            .env("DATABASE_URL", &database_url)
            .env("CORE_DATABASE_MAX_CONNECTIONS", "1")
            .env("CORE_DATABASE_ACQUIRE_TIMEOUT_MS", "1000")
            .env("CORE_HEALTH_LISTEN_ADDRESS", health_address.to_string())
            .env("CORE_HEALTH_CHECK_INTERVAL_MS", "50")
            .env("CORE_HEALTH_CHECK_TIMEOUT_MS", "500")
            .env("CORE_METRICS_LISTEN_ADDRESS", metrics_address.to_string())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .expect("CORE process must start while PostgreSQL is unavailable"),
    };

    let health_endpoint = format!("http://{health_address}");
    let channel = timeout(Duration::from_secs(5), async {
        loop {
            match Endpoint::from_shared(health_endpoint.clone())
                .expect("health endpoint must be a valid URI")
                .connect()
                .await
            {
                Ok(channel) => break channel,
                Err(_) => sleep(Duration::from_millis(20)).await,
            }
        }
    })
    .await
    .expect("CORE health listener must remain available");
    let mut client = HealthClient::new(channel);
    let readiness = client
        .check(HealthCheckRequest {
            service: READINESS_SERVICE.to_owned(),
        })
        .await
        .expect("CORE readiness must respond when PostgreSQL is unavailable")
        .into_inner();
    assert_eq!(
        HealthServingStatus::try_from(readiness.status)
            .expect("CORE readiness must return a known gRPC health status"),
        HealthServingStatus::NotServing
    );
    sleep(Duration::from_millis(100)).await;

    let process_id = process.child().id().to_string();
    let signal_status = Command::new("kill")
        .arg("-TERM")
        .arg(process_id)
        .status()
        .expect("SIGTERM must be deliverable to the CORE test process");
    assert!(signal_status.success(), "CORE process must accept SIGTERM");

    let exit_status = timeout(Duration::from_secs(5), async {
        loop {
            if let Some(status) = process
                .child()
                .try_wait()
                .expect("CORE process status must be inspectable")
            {
                break status;
            }
            sleep(Duration::from_millis(20)).await;
        }
    })
    .await
    .expect("CORE process must stop within the shutdown timeout");
    assert!(exit_status.success(), "CORE process must stop successfully");

    let (stdout, stderr) = process.read_logs();
    let failure_event = stdout
        .lines()
        .find(|line| line.contains("service.readiness.failed"))
        .expect("CORE must log PostgreSQL readiness failures");
    assert!(failure_event.contains("correlation_id"));
    assert!(failure_event.contains("trace_id"));
    assert!(
        failure_event.contains("\"failure_class\":\"dependency\"")
            || failure_event.contains("\"failure_class\":\"timeout\""),
        "readiness failure must have a stable classified error: {failure_event}"
    );
    assert!(!stdout.contains(&database_url));
    assert!(!stderr.contains(&database_url));
}
