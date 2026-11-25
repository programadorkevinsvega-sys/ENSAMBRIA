use std::{future::Future, pin::Pin, time::Duration};

use sqlx::PgPool;
use uuid::Uuid;

use super::service::{EmailDelivery, EmailDeliveryError, EmailMessage, SmtpEmailDelivery};

const MAX_ATTEMPTS: i32 = 10;
const LEASE_SECONDS: i64 = 120;

#[derive(Clone)]
pub struct OutboxEmailDelivery {
    pool: PgPool,
}

impl OutboxEmailDelivery {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

impl EmailDelivery for OutboxEmailDelivery {
    fn send<'a>(
        &'a self,
        message: EmailMessage,
    ) -> Pin<Box<dyn Future<Output = Result<(), EmailDeliveryError>> + Send + 'a>> {
        Box::pin(async move {
            sqlx::query(
                "INSERT INTO core_identity_auth.email_outbox (id, recipient, subject, text_body) VALUES ($1, $2, $3, $4)",
            )
            .bind(Uuid::now_v7())
            .bind(message.recipient)
            .bind(message.subject)
            .bind(message.text_body)
            .execute(&self.pool)
            .await
            .map_err(|_| EmailDeliveryError)?;
            Ok(())
        })
    }

    fn send_in_transaction<'a>(
        &'a self,
        connection: &'a mut sqlx::PgConnection,
        message: EmailMessage,
    ) -> Pin<Box<dyn Future<Output = Result<(), EmailDeliveryError>> + Send + 'a>> {
        Box::pin(async move {
            sqlx::query(
                "INSERT INTO core_identity_auth.email_outbox (id, recipient, subject, text_body) VALUES ($1, $2, $3, $4)",
            )
            .bind(Uuid::now_v7())
            .bind(message.recipient)
            .bind(message.subject)
            .bind(message.text_body)
            .execute(connection)
            .await
            .map_err(|_| EmailDeliveryError)?;
            Ok(())
        })
    }
}

/// Starts the production SMTP worker using CORE_SMTP_* deployment secrets.
pub async fn run_smtp_worker_from_env(
    pool: PgPool,
    poll_interval: Duration,
    batch_size: i64,
) -> Result<(), EmailOutboxError> {
    let smtp =
        SmtpEmailDelivery::from_env().map_err(|_| EmailOutboxError::InvalidSmtpConfiguration)?;
    EmailOutboxWorker
        .run(pool, smtp, poll_interval, batch_size)
        .await
}

#[derive(Debug, Clone, Copy, Default)]
pub struct EmailOutboxWorker;

impl EmailOutboxWorker {
    /// Claims and delivers up to `limit` messages. Expired leases are reclaimed after crashes.
    pub async fn deliver_batch<D: EmailDelivery>(
        &self,
        pool: &PgPool,
        delivery: &D,
        limit: i64,
    ) -> Result<usize, EmailOutboxError> {
        if !(1..=500).contains(&limit) {
            return Err(EmailOutboxError::InvalidBatchSize);
        }

        sqlx::query(
            "UPDATE core_identity_auth.email_outbox SET status = 'dead', locked_until = NULL, text_body = '[REDACTED]', last_error = 'delivery lease expired after maximum attempts' WHERE status = 'processing' AND locked_until <= clock_timestamp() AND attempts >= $1",
        )
        .bind(MAX_ATTEMPTS)
        .execute(pool)
        .await
        .map_err(|_| EmailOutboxError::Storage)?;

        let messages = sqlx::query_as::<_, (Uuid, String, String, String, i32)>(
            "WITH candidates AS (                SELECT id FROM core_identity_auth.email_outbox                 WHERE ((status = 'pending' AND available_at <= clock_timestamp())                     OR (status = 'processing' AND locked_until <= clock_timestamp()))                     AND attempts < $1                 ORDER BY available_at, created_at LIMIT $2 FOR UPDATE SKIP LOCKED             )              UPDATE core_identity_auth.email_outbox AS outbox              SET status = 'processing', attempts = attempts + 1,                  locked_until = clock_timestamp() + ($3 * interval '1 second')              FROM candidates WHERE outbox.id = candidates.id              RETURNING outbox.id, outbox.recipient, outbox.subject, outbox.text_body, outbox.attempts",
        )
        .bind(MAX_ATTEMPTS)
        .bind(limit)
        .bind(LEASE_SECONDS)
        .fetch_all(pool)
        .await
        .map_err(|_| EmailOutboxError::Storage)?;

        let mut delivered = 0;
        for (id, recipient, subject, text_body, attempts) in messages {
            let message = EmailMessage {
                recipient,
                subject,
                text_body,
            };
            match delivery.send(message).await {
                Ok(()) => {
                    sqlx::query(
                        "UPDATE core_identity_auth.email_outbox SET status = 'sent', sent_at = clock_timestamp(), locked_until = NULL, last_error = NULL, text_body = '[REDACTED]' WHERE id = $1 AND status = 'processing'",
                    )
                    .bind(id)
                    .execute(pool)
                    .await
                    .map_err(|_| EmailOutboxError::Storage)?;
                    delivered += 1;
                }
                Err(_) => {
                    let terminal = attempts >= MAX_ATTEMPTS;
                    let delay_seconds = 30_i64.saturating_mul(1_i64 << attempts.min(10));
                    sqlx::query(
                        "UPDATE core_identity_auth.email_outbox SET status = CASE WHEN $2 THEN 'dead' ELSE 'pending' END, text_body = CASE WHEN $2 THEN '[REDACTED]' ELSE text_body END, available_at = clock_timestamp() + ($3 * interval '1 second'), locked_until = NULL, last_error = 'smtp delivery failed' WHERE id = $1 AND status = 'processing'",
                    )
                    .bind(id)
                    .bind(terminal)
                    .bind(delay_seconds.min(86_400))
                    .execute(pool)
                    .await
                    .map_err(|_| EmailOutboxError::Storage)?;
                }
            }
        }
        Ok(delivered)
    }

    pub async fn run<D: EmailDelivery>(
        &self,
        pool: PgPool,
        delivery: D,
        poll_interval: Duration,
        batch_size: i64,
    ) -> Result<(), EmailOutboxError> {
        if poll_interval.is_zero() {
            return Err(EmailOutboxError::InvalidPollInterval);
        }
        loop {
            self.deliver_batch(&pool, &delivery, batch_size).await?;
            tokio::time::sleep(poll_interval).await;
        }
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum EmailOutboxError {
    InvalidBatchSize,
    InvalidPollInterval,
    InvalidSmtpConfiguration,
    Storage,
}

impl std::fmt::Display for EmailOutboxError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Self::InvalidBatchSize => "email outbox batch size must be between 1 and 500",
            Self::InvalidPollInterval => "email outbox poll interval must be positive",
            Self::InvalidSmtpConfiguration => "CORE_SMTP_* configuration is invalid or incomplete",
            Self::Storage => "email outbox persistence operation failed",
        })
    }
}
impl std::error::Error for EmailOutboxError {}
