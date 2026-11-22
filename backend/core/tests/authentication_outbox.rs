use std::{future::Future, pin::Pin, sync::{Arc, Mutex}};

use ensambria_core::{
    core_migrations::run_core_migrations_from_path,
    identity_authentication::{
        crypto::secret_encryption::Aes256GcmSecretProtector,
        email_outbox::{EmailOutboxWorker, OutboxEmailDelivery},
        service::{AuthenticationService, EmailDelivery, EmailDeliveryError, EmailMessage, LoginOutcome},
    },
};
use sqlx::PgPool;
use std::{str::FromStr, time::Duration};
use sqlx::postgres::PgConnectOptions;
use uuid::Uuid;

#[derive(Clone, Default)]
struct CapturingDelivery {
    bodies: Arc<Mutex<Vec<String>>>,
}

impl EmailDelivery for CapturingDelivery {
    fn send<'a>(&'a self, message: EmailMessage)
        -> Pin<Box<dyn Future<Output = Result<(), EmailDeliveryError>> + Send + 'a>>
    {
        self.bodies.lock().expect("capture lock").push(message.text_body);
        Box::pin(std::future::ready(Ok(())))
    }
}

impl CapturingDelivery {
    fn take_token(&self, marker: &str) -> String {
        let bodies = self.bodies.lock().expect("capture lock");
        let body = bodies.iter().rev().find(|body| body.contains(marker))
            .expect("expected outbox email to be delivered");
        body.split(marker).nth(1)
            .and_then(|tail| tail.split_whitespace().next())
            .map(|value| value.trim_end_matches(|c: char| !c.is_ascii_hexdigit()))
            .filter(|value| value.len() == 64)
            .expect("email contains a 256-bit opaque token")
            .to_owned()
    }
}

async fn pools() -> (PgPool, PgPool) {
    let migration_url = std::env::var("CORE_TEST_MIGRATOR_DATABASE_URL").expect("migrator URL");
    let migrations = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("migrations");
    run_core_migrations_from_path(&migration_url, &migrations).await.expect("migrations apply");
    let runtime_url = std::env::var("CORE_TEST_DATABASE_URL").expect("runtime URL");
    let worker_url = std::env::var("CORE_EMAIL_WORKER_DATABASE_URL").expect("worker URL");
    let runtime = PgPool::connect_with(PgConnectOptions::from_str(&runtime_url).expect("runtime URL parses"))
        .await.expect("runtime connects");
    let worker = PgPool::connect_with(PgConnectOptions::from_str(&worker_url).expect("worker URL parses"))
        .await.expect("worker connects");
    (runtime, worker)
}

fn service(pool: PgPool) -> AuthenticationService<OutboxEmailDelivery> {
    let protector = Aes256GcmSecretProtector::new("integration-kms-key-v1".into(), &[0x52; 32])
        .expect("test encryption key");
    AuthenticationService::new(
        pool.clone(),
        OutboxEmailDelivery::new(pool),
        protector,
        &[0x53; 32],
        "https://app.example.test".into(),
    ).expect("authentication service")
}

#[tokio::test]
async fn registration_and_recovery_are_enqueued_then_delivered_by_the_worker() {
    let (runtime, worker_pool) = pools().await;
    let auth = service(runtime.clone());
    let delivery = CapturingDelivery::default();
    let worker = EmailOutboxWorker;
    let email = format!("{}@example.test", Uuid::now_v7());

    auth.register(&email, Some("Outbox Integration"), "correct horse battery staple")
        .await.expect("registration should enqueue verification");
    let queued: i64 = sqlx::query_scalar(
        "SELECT count(*) FROM core.email_outbox WHERE recipient = $1 AND status = 'pending'",
    ).bind(&email).fetch_one(&worker_pool).await.expect("inspect queued verification");
    assert_eq!(queued, 1);

    assert_eq!(worker.deliver_batch(&worker_pool, &delivery, 10).await.expect("deliver verification"), 1);
    let verification_token = delivery.take_token("verify-email?token=");
    auth.verify_email(&verification_token).await.expect("outbox-delivered verification token activates identity");

    auth.request_recovery(&email).await.expect("recovery should enqueue email");
    assert_eq!(worker.deliver_batch(&worker_pool, &delivery, 10).await.expect("deliver recovery"), 1);
    let recovery_token = delivery.take_token("reset-password?token=");
    auth.complete_recovery(&recovery_token, "battery staple correct horse")
        .await.expect("outbox-delivered recovery token resets credential");

    assert!(matches!(
        auth.login(&email, "battery staple correct horse").await,
        Ok(LoginOutcome::Authenticated(_))
    ));
    assert!(auth.complete_recovery(&recovery_token, "another password").await.is_err());

    runtime.close().await;
    worker_pool.close().await;
    let _ = Duration::from_secs(0);
}
