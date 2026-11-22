use std::{future::Future, pin::Pin, sync::{Arc, atomic::{AtomicUsize, Ordering}}, time::Duration};

use ensambria_core::{
    core_migrations::run_core_migrations_from_path,
    identity_authentication::{
        email_outbox::{EmailOutboxWorker, OutboxEmailDelivery},
        service::{EmailDelivery, EmailDeliveryError, EmailMessage},
    },
};
use sqlx::PgPool;
use uuid::Uuid;

async fn test_pool() -> PgPool {
    let migration_url = std::env::var("CORE_TEST_MIGRATOR_DATABASE_URL")
        .expect("CORE_TEST_MIGRATOR_DATABASE_URL must be configured");
    let migrations = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("migrations");
    run_core_migrations_from_path(&migration_url, &migrations)
        .await.expect("CORE migrations must apply");
    PgPool::connect(&std::env::var("CORE_TEST_DATABASE_URL").expect("CORE_TEST_DATABASE_URL must be configured"))
        .await.expect("CORE runtime must connect")
}

#[derive(Clone)]
struct CountingDelivery {
    sent: Arc<AtomicUsize>,
    fail: bool,
}

impl EmailDelivery for CountingDelivery {
    fn send<'a>(&'a self, _message: EmailMessage)
        -> Pin<Box<dyn Future<Output = Result<(), EmailDeliveryError>> + Send + 'a>>
    {
        let sent = self.sent.clone();
        let fail = self.fail;
        Box::pin(async move {
            if fail {
                Err(EmailDeliveryError)
            } else {
                sent.fetch_add(1, Ordering::SeqCst);
                Ok(())
            }
        })
    }
}

#[tokio::test]
async fn email_outbox_persists_messages_and_worker_marks_success_or_schedules_retry() {
    let pool = test_pool().await;
    let outbox = OutboxEmailDelivery::new(pool.clone());
    let worker = EmailOutboxWorker;
    let id = Uuid::now_v7();

    outbox.send(EmailMessage {
        recipient: format!("{id}@example.test"),
        subject: "ENSAMBRIA test".into(),
        text_body: "one-time token [REDACTED]".into(),
    }).await.expect("message must be durably queued");

    let failed = CountingDelivery { sent: Arc::new(AtomicUsize::new(0)), fail: true };
    assert_eq!(worker.deliver_batch(&pool, &failed, 10).await.expect("worker must claim message"), 0);
    let state: (String, i32) = sqlx::query_as(
        "SELECT status, attempts FROM core.email_outbox WHERE recipient = $1",
    )
    .bind(format!("{id}@example.test"))
    .fetch_one(&pool).await.expect("retry state must persist");
    assert_eq!(state.0, "pending");
    assert_eq!(state.1, 1);

    sqlx::query("UPDATE core.email_outbox SET available_at = clock_timestamp() WHERE recipient = $1")
        .bind(format!("{id}@example.test"))
        .execute(&pool).await.expect("test retry must be made due");

    let successful = CountingDelivery { sent: Arc::new(AtomicUsize::new(0)), fail: false };
    assert_eq!(worker.deliver_batch(&pool, &successful, 10).await.expect("worker must retry"), 1);
    assert_eq!(successful.sent.load(Ordering::SeqCst), 1);
    let final_state: String = sqlx::query_scalar(
        "SELECT status FROM core.email_outbox WHERE recipient = $1",
    )
    .bind(format!("{id}@example.test"))
    .fetch_one(&pool).await.expect("sent state must persist");
    assert_eq!(final_state, "sent");

    assert!(worker.deliver_batch(&pool, &successful, 0).await.is_err());
    pool.close().await;
}
