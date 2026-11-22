use std::{
    future::Future,
    pin::Pin,
    str::FromStr,
    sync::{Arc, Mutex},
    time::Duration,
};

use ensambria_core::{
    core_migrations::run_core_migrations_from_path,
    identity_authentication::{
        crypto::{
            password::{PasswordHash, hash_password, verify_password},
            secret_encryption::Aes256GcmSecretProtector,
            totp::TotpSecret,
        },
        service::{
            AuthenticationService, EmailDelivery, EmailDeliveryError, EmailMessage,
            LoginOutcome, unix_time_now,
        },
    },
    postgres::{ActorContext, ActorId, PgPoolConfiguration, create_pool},
};
use sqlx::{PgPool, postgres::PgConnectOptions, types::Uuid};

async fn test_pool() -> PgPool {
    let migration_url = std::env::var("CORE_TEST_MIGRATOR_DATABASE_URL")
        .expect("isolated PostgreSQL migration URL must be configured");
    let migrations = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("migrations");
    if let Err(error) = run_core_migrations_from_path(&migration_url, &migrations).await {
        let source = std::error::Error::source(&error).map(|source| format!("{source:?}"));
        panic!("CORE migrations must apply with the restricted migration identity: {error:?}; source={source:?}");
    }
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
    let token_hash = vec![0x77_u8; 32];
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
    sqlx::query(
        "INSERT INTO core.identity_credentials (identity_id, password_hash) VALUES ($1, $2)",
    )
    .bind(identity_id)
    .bind(stored_hash.as_str())
    .execute(&mut *transaction)
    .await
    .expect("Argon2id credential must be inserted under its actor context");
    sqlx::query(
        "INSERT INTO core.auth_sessions (id, identity_id, refresh_token_hash, expires_at) VALUES ($1, $2, $3, clock_timestamp() + interval '1 day')",
    )
    .bind(session_id)
    .bind(identity_id)
    .bind(&token_hash)
    .execute(&mut *transaction)
    .await
    .expect("opaque refresh session must be inserted under its actor context");
    transaction
        .commit()
        .await
        .expect("identity transaction must commit");

    let lookup: Option<(Uuid, String)> =
        sqlx::query_as("SELECT identity_id, password_hash FROM core.auth_lookup_identity($1)")
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
        sqlx::query_scalar("SELECT count(*) FROM core.auth_sessions WHERE identity_id = $1")
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


#[derive(Clone, Default)]
struct CapturingEmailDelivery {
    messages: Arc<Mutex<Vec<EmailMessage>>>,
}

impl CapturingEmailDelivery {
    fn token_from_latest_message(&self, marker: &str) -> String {
        let messages = self.messages.lock().expect("email capture lock must be healthy");
        let body = messages
            .iter()
            .rev()
            .find(|message| message.text_body.contains(marker))
            .expect("expected transactional email must be delivered")
            .text_body
            .clone();
        body.split(marker)
            .nth(1)
            .and_then(|value| value.split_whitespace().next())
            .map(|value| value.trim_end_matches(|character: char| !character.is_ascii_hexdigit()))
            .filter(|value| value.len() == 64)
            .expect("email must contain a 256-bit opaque token")
            .to_owned()
    }
}

impl EmailDelivery for CapturingEmailDelivery {
    fn send<'a>(
        &'a self,
        message: EmailMessage,
    ) -> Pin<Box<dyn Future<Output = Result<(), EmailDeliveryError>> + Send + 'a>> {
        self.messages
            .lock()
            .expect("email capture lock must be healthy")
            .push(message);
        Box::pin(std::future::ready(Ok(())))
    }
}

fn authentication_service(
    pool: PgPool,
    delivery: CapturingEmailDelivery,
) -> AuthenticationService<CapturingEmailDelivery> {
    let protector = Aes256GcmSecretProtector::new("test-kms-v1".into(), &[0x42; 32])
        .expect("test MFA key must be valid");
    AuthenticationService::new(
        pool,
        delivery,
        protector,
        &[0x43; 32],
        "https://app.example.test".into(),
    )
    .expect("test authentication service configuration must be valid")
}

#[tokio::test]
async fn registration_login_refresh_reuse_and_recovery_are_single_use() {
    let pool = test_pool().await;
    let delivery = CapturingEmailDelivery::default();
    let service = authentication_service(pool.clone(), delivery.clone());
    let email = format!("{}@example.test", Uuid::now_v7());
    let initial_password = "correct horse battery staple";
    let replacement_password = "battery staple correct horse";

    service
        .register(&email, Some("Integration Test"), initial_password)
        .await
        .expect("registration must complete");
    let verification_token = delivery.token_from_latest_message("verify-email?token=");
    service
        .verify_email(&verification_token)
        .await
        .expect("email verification token must be consumable once");
    assert!(
        service.verify_email(&verification_token).await.is_err(),
        "email verification token must not be reusable"
    );

    let first_session = match service.login(&email, initial_password).await {
        Ok(LoginOutcome::Authenticated(session)) => session,
        _ => panic!("verified identity must authenticate without MFA"),
    };
    assert_eq!(
        service
            .authenticate_access_token(&first_session.access_token)
            .await
            .expect("issued access token must authenticate"),
        first_session.identity_id
    );

    let rotated_session = service
        .refresh_session(&first_session.refresh_token)
        .await
        .expect("refresh token must rotate");
    assert!(
        service
            .authenticate_access_token(&first_session.access_token)
            .await
            .is_err(),
        "rotated access token must no longer authenticate"
    );
    assert_eq!(
        service.refresh_session(&first_session.refresh_token).await.unwrap_err(),
        ensambria_core::identity_authentication::service::AuthenticationError::RefreshReuseDetected,
        "reusing a rotated refresh token must be detected"
    );
    assert!(
        service
            .authenticate_access_token(&rotated_session.access_token)
            .await
            .is_err(),
        "refresh reuse must revoke the affected session family"
    );

    service
        .request_recovery(&email)
        .await
        .expect("recovery request must not disclose account existence");
    let recovery_token = delivery.token_from_latest_message("reset-password?token=");
    service
        .complete_recovery(&recovery_token, replacement_password)
        .await
        .expect("recovery token must replace the credential");
    assert!(
        service
            .complete_recovery(&recovery_token, initial_password)
            .await
            .is_err(),
        "recovery token must be single use"
    );
    assert!(
        service.login(&email, initial_password).await.is_err(),
        "old password must stop working after recovery"
    );
    let recovered_session = match service.login(&email, replacement_password).await {
        Ok(LoginOutcome::Authenticated(session)) => session,
        _ => panic!("recovered identity must authenticate with the replacement password"),
    };
    service
        .logout_all(&recovered_session.access_token)
        .await
        .expect("authenticated actor must revoke all of its sessions");
    assert!(
        service
            .authenticate_access_token(&recovered_session.access_token)
            .await
            .is_err(),
        "global logout must revoke the current session"
    );
    pool.close().await;
}

#[tokio::test]
async fn totp_enrollment_mfa_challenge_recovery_code_and_disable_complete() {
    let pool = test_pool().await;
    let delivery = CapturingEmailDelivery::default();
    let service = authentication_service(pool.clone(), delivery.clone());
    let email = format!("{}@example.test", Uuid::now_v7());
    let password = "correct horse battery staple";

    service
        .register(&email, Some("MFA Integration Test"), password)
        .await
        .expect("registration must complete");
    let verification_token = delivery.token_from_latest_message("verify-email?token=");
    let identity_id = service
        .verify_email(&verification_token)
        .await
        .expect("email verification must complete");
    let initial_session = match service.login(&email, password).await {
        Ok(LoginOutcome::Authenticated(session)) => session,
        _ => panic!("identity must authenticate before MFA enrollment"),
    };

    let enrollment_uri = service
        .begin_totp_enrollment(&initial_session.access_token, password)
        .await
        .expect("authenticated identity must begin TOTP enrollment");
    let encoded_secret = enrollment_uri
        .split("secret=")
        .nth(1)
        .and_then(|value| value.split('&').next())
        .expect("enrollment URI must contain a secret");
    let secret = TotpSecret::from_base32(encoded_secret)
        .expect("enrollment URI secret must be valid base32");
    let current_step = unix_time_now().expect("server clock must be available") / 30;
    let current_code = secret.code_at_step(current_step);
    let recovery_codes = service
        .confirm_totp_enrollment(&initial_session.access_token, password, &current_code)
        .await
        .expect("valid TOTP code must confirm enrollment");
    assert_eq!(recovery_codes.len(), 10);

    let challenge_token = match service.login(&email, password).await {
        Ok(LoginOutcome::MfaRequired { challenge_token }) => challenge_token,
        _ => panic!("enabled MFA must require a second factor"),
    };
    let mfa_session = service
        .verify_mfa_challenge(&challenge_token, &recovery_codes[0])
        .await
        .expect("one-time recovery code must complete the MFA challenge");
    assert_eq!(mfa_session.identity_id, identity_id);

    let next_challenge = match service.login(&email, password).await {
        Ok(LoginOutcome::MfaRequired { challenge_token }) => challenge_token,
        _ => panic!("subsequent login must still require MFA"),
    };
    assert_eq!(
        service
            .verify_mfa_challenge(&next_challenge, &recovery_codes[0])
            .await
            .unwrap_err(),
        ensambria_core::identity_authentication::service::AuthenticationError::InvalidMfaFactor,
        "consumed MFA recovery code must not be reusable"
    );

    service
        .disable_totp(&mfa_session.access_token, password)
        .await
        .expect("current password and authenticated session must disable TOTP");
    assert!(matches!(
        service.login(&email, password).await,
        Ok(LoginOutcome::Authenticated(_))
    ));
    pool.close().await;
}
