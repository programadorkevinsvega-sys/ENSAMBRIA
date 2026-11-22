use std::{
    future::Future,
    pin::Pin,
    sync::{
        Arc,
        atomic::{AtomicUsize, Ordering},
    },
};

use ensambria_core::{
    core_migrations::run_core_migrations_from_path,
    identity_authentication::{
        email_outbox::{EmailOutboxWorker, OutboxEmailDelivery},
        service::{EmailDelivery, EmailDeliveryError, EmailMessage},
    },
};
use sqlx::PgPool;
use uuid::Uuid;

async fn test_pools() -> (PgPool, PgPool) {
    let migration_url = std::env::var("CORE_TEST_MIGRATOR_DATABASE_URL")
        .expect("CORE_TEST_MIGRATOR_DATABASE_URL must be configured");
    let migrations = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("migrations");
    run_core_migrations_from_path(&migration_url, &migrations)
        .await
        .expect("CORE migrations must apply");
    let runtime = PgPool::connect(
        &std::env::var("CORE_TEST_DATABASE_URL")
            .expect("CORE_TEST_DATABASE_URL must be configured"),
    )
    .await
    .expect("CORE runtime must connect");
    let worker = PgPool::connect(
        &std::env::var("CORE_EMAIL_WORKER_DATABASE_URL")
            .expect("CORE_EMAIL_WORKER_DATABASE_URL must be configured"),
    )
    .await
    .expect("restricted CORE email worker must connect");
    (runtime, worker)
}

#[derive(Clone)]
struct CountingDelivery {
    sent: Arc<AtomicUsize>,
    fail: bool,
}

impl EmailDelivery for CountingDelivery {
    fn send<'a>(
        &'a self,
        _message: EmailMessage,
    ) -> Pin<Box<dyn Future<Output = Result<(), EmailDeliveryError>> + Send + 'a>> {
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
    let (runtime_pool, worker_pool) = test_pools().await;
    let outbox = OutboxEmailDelivery::new(runtime_pool.clone());
    let worker = EmailOutboxWorker;
    let id = Uuid::now_v7();

    outbox
        .send(EmailMessage {
            recipient: format!("{id}@example.test"),
            subject: "ENSAMBRIA test".into(),
            text_body: "one-time token [REDACTED]".into(),
        })
        .await
        .expect("message must be durably queued");

    let failed = CountingDelivery {
        sent: Arc::new(AtomicUsize::new(0)),
        fail: true,
    };
    assert_eq!(
        worker
            .deliver_batch(&worker_pool, &failed, 10)
            .await
            .expect("worker must claim message"),
        0
    );
    let state: (String, i32) =
        sqlx::query_as("SELECT status, attempts FROM core.email_outbox WHERE recipient = $1")
            .bind(format!("{id}@example.test"))
            .fetch_one(&worker_pool)
            .await
            .expect("retry state must persist");
    assert_eq!(state.0, "pending");
    assert_eq!(state.1, 1);

    sqlx::query(
        "UPDATE core.email_outbox SET available_at = clock_timestamp() WHERE recipient = $1",
    )
    .bind(format!("{id}@example.test"))
    .execute(&worker_pool)
    .await
    .expect("test retry must be made due");

    let successful = CountingDelivery {
        sent: Arc::new(AtomicUsize::new(0)),
        fail: false,
    };
    assert_eq!(
        worker
            .deliver_batch(&worker_pool, &successful, 10)
            .await
            .expect("worker must retry"),
        1
    );
    assert_eq!(successful.sent.load(Ordering::SeqCst), 1);
    let final_state: String =
        sqlx::query_scalar("SELECT status FROM core.email_outbox WHERE recipient = $1")
            .bind(format!("{id}@example.test"))
            .fetch_one(&worker_pool)
            .await
            .expect("sent state must persist");
    assert_eq!(final_state, "sent");
    let retained_body: String =
        sqlx::query_scalar("SELECT text_body FROM core.email_outbox WHERE recipient = $1")
            .bind(format!("{id}@example.test"))
            .fetch_one(&worker_pool)
            .await
            .expect("sent payload metadata must remain");
    assert_eq!(retained_body, "[REDACTED]");

    assert!(
        worker
            .deliver_batch(&worker_pool, &successful, 0)
            .await
            .is_err()
    );
    runtime_pool.close().await;
    worker_pool.close().await;
}

#[tokio::test]
async fn outbox_enqueue_obeys_the_callers_transaction_boundary() {
    let (runtime_pool, worker_pool) = test_pools().await;
    let outbox = OutboxEmailDelivery::new(runtime_pool.clone());
    let recipient = format!("{}@example.test", Uuid::now_v7());

    let mut rolled_back = runtime_pool.begin().await.expect("transaction must begin");
    outbox
        .send_in_transaction(
            &mut *rolled_back,
            EmailMessage {
                recipient: recipient.clone(),
                subject: "rollback test".into(),
                text_body: "one-time token [REDACTED]".into(),
            },
        )
        .await
        .expect("outbox insert must join the caller transaction");
    rolled_back.rollback().await.expect("transaction must roll back");

    let after_rollback: i64 =
        sqlx::query_scalar("SELECT count(*) FROM core.email_outbox WHERE recipient = $1")
            .bind(&recipient)
            .fetch_one(&worker_pool)
            .await
            .expect("inspect rolled-back outbox row");
    assert_eq!(after_rollback, 0);

    let mut committed = runtime_pool.begin().await.expect("second transaction must begin");
    outbox
        .send_in_transaction(
            &mut *committed,
            EmailMessage {
                recipient: recipient.clone(),
                subject: "commit test".into(),
                text_body: "one-time token [REDACTED]".into(),
            },
        )
        .await
        .expect("outbox insert must join the caller transaction");
    committed.commit().await.expect("transaction must commit");

    let after_commit: i64 =
        sqlx::query_scalar("SELECT count(*) FROM core.email_outbox WHERE recipient = $1")
            .bind(&recipient)
            .fetch_one(&worker_pool)
            .await
            .expect("inspect committed outbox row");
    assert_eq!(after_commit, 1);

    runtime_pool.close().await;
    worker_pool.close().await;
}
