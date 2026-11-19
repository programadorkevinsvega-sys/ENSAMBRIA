use std::{process::Command, time::Duration};

use ensambria_core::health::{LIVENESS_SERVICE, READINESS_SERVICE};
use tokio::{
    net::TcpListener,
    sync::oneshot,
    time::{sleep, timeout},
};
use tokio_stream::wrappers::TcpListenerStream;
use tonic::transport::Server;
use tonic_health::{ServingStatus, server::health_reporter};

#[tokio::test]
async fn healthcheck_cli_requires_liveness_and_readiness_without_database_configuration() {
    let (reporter, health_service) = health_reporter();
    reporter
        .set_service_status(LIVENESS_SERVICE, ServingStatus::Serving)
        .await;
    reporter
        .set_service_status(READINESS_SERVICE, ServingStatus::Serving)
        .await;
    let listener = TcpListener::bind("127.0.0.1:0")
        .await
        .expect("health listener must bind to loopback");
    let address = listener
        .local_addr()
        .expect("health listener must expose its address");
    let (shutdown_sender, shutdown_receiver) = oneshot::channel();
    let server = Server::builder()
        .add_service(health_service)
        .serve_with_incoming_shutdown(TcpListenerStream::new(listener), async {
            shutdown_receiver
                .await
                .expect("health server shutdown signal must be sent");
        });
    let server_task = tokio::spawn(server);

    let healthy_probe = timeout(Duration::from_secs(5), async {
        loop {
            let output = run_healthcheck(address.to_string()).await;
            if output.status.success() {
                break output;
            }
            sleep(Duration::from_millis(25)).await;
        }
    })
    .await
    .expect("healthcheck CLI must connect to the gRPC service");
    assert!(healthy_probe.stdout.is_empty());
    assert!(healthy_probe.stderr.is_empty());

    reporter
        .set_service_status(READINESS_SERVICE, ServingStatus::NotServing)
        .await;
    let unhealthy_probe = run_healthcheck(address.to_string()).await;
    assert!(!unhealthy_probe.status.success());
    assert!(
        String::from_utf8_lossy(&unhealthy_probe.stderr).contains("Readiness is not serving")
            || String::from_utf8_lossy(&unhealthy_probe.stderr).contains(READINESS_SERVICE)
    );

    assert!(shutdown_sender.send(()).is_ok());
    server_task
        .await
        .expect("health server task must complete")
        .expect("health server must stop cleanly");
}

async fn run_healthcheck(address: String) -> std::process::Output {
    tokio::task::spawn_blocking(move || {
        Command::new(env!("CARGO_BIN_EXE_core"))
            .arg("--healthcheck")
            .arg(address)
            .env_remove("DATABASE_URL")
            .env_remove("CORE_DATABASE_MAX_CONNECTIONS")
            .env_remove("CORE_DATABASE_ACQUIRE_TIMEOUT_MS")
            .env_remove("CORE_HEALTH_LISTEN_ADDRESS")
            .env_remove("CORE_HEALTH_CHECK_INTERVAL_MS")
            .env_remove("CORE_HEALTH_CHECK_TIMEOUT_MS")
            .env_remove("CORE_METRICS_LISTEN_ADDRESS")
            .output()
            .expect("healthcheck CLI process must start")
    })
    .await
    .expect("healthcheck subprocess task must complete")
}
