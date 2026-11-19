use std::{error::Error, io};

use ensambria_core::{
    health::{
        LIVENESS_SERVICE, READINESS_SERVICE, initialize_health_status, monitor_database_readiness,
    },
    observability::{initialize_tracing, install_metrics_exporter},
    postgres::PgDatabaseConfiguration,
    runtime_config::CoreRuntimeConfiguration,
};
use tokio::{
    net::TcpListener,
    sync::{oneshot, watch},
};
use tokio_stream::wrappers::TcpListenerStream;
use tonic::transport::Server;
use tonic_health::{ServingStatus, server::health_reporter};
use uuid::Uuid;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    initialize_tracing()?;
    if let Err(error) = run().await {
        tracing::error!(
            event = "service.startup_or_runtime.failed",
            component = "backend-core",
            operation = "process.run",
            result = "failure",
            failure_class = "startup_or_runtime"
        );
        return Err(error);
    }
    Ok(())
}

async fn run() -> Result<(), Box<dyn Error>> {
    let database_configuration = PgDatabaseConfiguration::from_env()?;
    let runtime_configuration = CoreRuntimeConfiguration::from_env()?;
    let pool = database_configuration.connect_lazy();
    install_metrics_exporter(runtime_configuration.metrics_listen_address)?;

    let (reporter, health_service) = health_reporter();
    initialize_health_status(&reporter).await;

    let listener = TcpListener::bind(runtime_configuration.health_listen_address).await?;
    let health_address = listener.local_addr()?;
    let (shutdown_sender, shutdown_receiver) = oneshot::channel();
    let (monitor_shutdown_sender, monitor_shutdown_receiver) = watch::channel(false);
    let monitor = tokio::spawn(monitor_database_readiness(
        pool.clone(),
        reporter.clone(),
        runtime_configuration.health_check_interval,
        runtime_configuration.health_check_timeout,
        monitor_shutdown_receiver,
    ));

    tracing::info!(
        event = "service.started",
        component = "backend-core",
        operation = "process.start",
        service = "backend-core",
        health_address = %health_address,
        liveness_service = LIVENESS_SERVICE,
        readiness_service = READINESS_SERVICE,
        result = "success"
    );

    let incoming = TcpListenerStream::new(listener);
    let server = Server::builder()
        .trace_fn(health_request_span)
        .add_service(health_service)
        .serve_with_incoming_shutdown(incoming, async {
            let _ = shutdown_receiver.await;
        });
    tokio::pin!(server);

    tokio::select! {
        result = &mut server => result?,
        signal_result = shutdown_signal() => {
            signal_result?;
            reporter.set_service_status(LIVENESS_SERVICE, ServingStatus::NotServing).await;
            reporter.set_service_status(READINESS_SERVICE, ServingStatus::NotServing).await;
            reporter.set_service_status("", ServingStatus::NotServing).await;
            monitor_shutdown_sender.send(true)?;
            shutdown_sender.send(()).map_err(|_| io::Error::other("health server shutdown channel closed"))?;
            server.await?;
        }
    }

    if let Err(error) = monitor.await {
        tracing::error!(
            event = "service.readiness_monitor.join_failed",
            component = "backend-core",
            operation = "readiness.monitor.stop",
            result = "failure",
            failure_class = "task_join"
        );
        return Err(Box::new(error));
    }

    pool.close().await;
    tracing::info!(
        event = "service.stopped",
        component = "backend-core",
        operation = "process.stop",
        service = "backend-core",
        result = "success"
    );
    Ok(())
}

fn health_request_span(request: &http::Request<()>) -> tracing::Span {
    let correlation_id = request
        .headers()
        .get("x-correlation-id")
        .and_then(|value| value.to_str().ok())
        .and_then(|value| Uuid::parse_str(value).ok())
        .unwrap_or_else(Uuid::now_v7);
    let trace_id = request
        .headers()
        .get("traceparent")
        .and_then(|value| value.to_str().ok())
        .and_then(parse_trace_id)
        .map(str::to_owned)
        .unwrap_or_else(|| Uuid::now_v7().simple().to_string());

    tracing::info_span!(
        "grpc.health.request",
        component = "backend-core",
        operation = %request.uri().path(),
        correlation_id = %correlation_id,
        trace_id = %trace_id
    )
}

fn parse_trace_id(traceparent: &str) -> Option<&str> {
    let bytes = traceparent.as_bytes();
    if bytes.len() != 55 || bytes[2] != b'-' || bytes[35] != b'-' || bytes[52] != b'-' {
        return None;
    }
    if &bytes[..2] != b"00" {
        return None;
    }

    let trace_id = &traceparent[3..35];
    let span_id = &traceparent[36..52];
    let flags = &traceparent[53..55];
    let is_hex = |value: &str| value.bytes().all(|byte| byte.is_ascii_hexdigit());
    if !is_hex(trace_id)
        || !is_hex(span_id)
        || !is_hex(flags)
        || trace_id.bytes().all(|byte| byte == b'0')
        || span_id.bytes().all(|byte| byte == b'0')
    {
        return None;
    }

    Some(trace_id)
}

async fn shutdown_signal() -> io::Result<()> {
    #[cfg(unix)]
    {
        let mut terminate =
            tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())?;
        tokio::select! {
            result = tokio::signal::ctrl_c() => result,
            _ = terminate.recv() => Ok(()),
        }
    }
    #[cfg(not(unix))]
    {
        tokio::signal::ctrl_c().await
    }
}

#[cfg(test)]
mod tests {
    use super::parse_trace_id;

    #[test]
    fn accepts_valid_w3c_traceparent_and_rejects_malformed_context() {
        assert_eq!(
            parse_trace_id("00-4bf92f3577b34da6a3ce929d0e0e4736-00f067aa0ba902b7-01"),
            Some("4bf92f3577b34da6a3ce929d0e0e4736")
        );
        assert_eq!(
            parse_trace_id("00-00000000000000000000000000000000-00f067aa0ba902b7-01"),
            None
        );
        assert_eq!(parse_trace_id("invalid"), None);
    }
}
