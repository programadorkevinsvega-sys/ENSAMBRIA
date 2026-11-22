use std::{
    error::Error,
    fmt,
    future::Future,
    pin::Pin,
    time::{Duration, SystemTime, UNIX_EPOCH},
};

use ring::{
    digest::{SHA256, digest},
    hmac,
};
use lettre::{
    AsyncSmtpTransport, AsyncTransport, Message, Tokio1Executor,
    message::Mailbox,
    transport::smtp::authentication::Credentials,
};
use sqlx::{PgPool, Row};
use uuid::Uuid;
use zeroize::Zeroizing;

use crate::{
    identity_authentication::crypto::{
        email::normalize_email,
        password::{PasswordHash, hash_password, verify_password},
        secret_encryption::{Aes256GcmSecretProtector, EncryptedSecret},
        session_token::{SessionToken, SessionTokenHash},
        totp::TotpSecret,
    },
    postgres::{ActorContext, ActorId},
};

const ACCESS_TOKEN_TTL: Duration = Duration::from_secs(15 * 60);
const REFRESH_TOKEN_TTL: Duration = Duration::from_secs(30 * 24 * 60 * 60);
const VERIFICATION_TOKEN_TTL: Duration = Duration::from_secs(24 * 60 * 60);
const RECOVERY_TOKEN_TTL: Duration = Duration::from_secs(60 * 60);
const MFA_CHALLENGE_TTL: Duration = Duration::from_secs(5 * 60);
const RATE_WINDOW_SECONDS: i32 = 60;
const RATE_MAX_ATTEMPTS: i32 = 10;
const RATE_BLOCK_SECONDS: i32 = 300;
const MFA_RECOVERY_CODE_COUNT: usize = 10;

pub struct EmailMessage {
    pub recipient: String,
    pub subject: String,
    pub text_body: String,
}

impl fmt::Debug for EmailMessage {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("EmailMessage")
            .field("recipient", &self.recipient)
            .field("subject", &self.subject)
            .field("text_body", &"[REDACTED]")
            .finish()
    }
}

pub trait EmailDelivery: Send + Sync {
    fn send<'a>(
        &'a self,
        message: EmailMessage,
    ) -> Pin<Box<dyn Future<Output = Result<(), EmailDeliveryError>> + Send + 'a>>;
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct EmailDeliveryError;

impl fmt::Display for EmailDeliveryError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("transactional email delivery failed")
    }
}
impl Error for EmailDeliveryError {}


pub struct SmtpEmailDelivery {
    transport: AsyncSmtpTransport<Tokio1Executor>,
    from: Mailbox,
}

impl SmtpEmailDelivery {
    pub fn new(
        host: &str,
        port: u16,
        username: String,
        password: String,
        from_address: &str,
    ) -> Result<Self, EmailDeliveryError> {
        if host.trim().is_empty()
            || host.contains("://")
            || host.bytes().any(|byte| byte.is_ascii_whitespace())
            || port == 0
            || username.is_empty()
            || password.is_empty()
        {
            return Err(EmailDeliveryError);
        }
        let from = from_address
            .parse::<Mailbox>()
            .map_err(|_| EmailDeliveryError)?;
        let transport = AsyncSmtpTransport::<Tokio1Executor>::starttls_relay(host.trim())
            .map_err(|_| EmailDeliveryError)?
            .port(port)
            .credentials(Credentials::new(username, password))
            .build();
        Ok(Self { transport, from })
    }
}

impl EmailDelivery for SmtpEmailDelivery {
    fn send<'a>(
        &'a self,
        message: EmailMessage,
    ) -> Pin<Box<dyn Future<Output = Result<(), EmailDeliveryError>> + Send + 'a>> {
        Box::pin(async move {
            let recipient = message
                .recipient
                .parse::<Mailbox>()
                .map_err(|_| EmailDeliveryError)?;
            let email = Message::builder()
                .from(self.from.clone())
                .to(recipient)
                .subject(message.subject)
                .body(message.text_body)
                .map_err(|_| EmailDeliveryError)?;
            self.transport
                .send(email)
                .await
                .map_err(|_| EmailDeliveryError)?;
            Ok(())
        })
    }
}

pub struct AuthenticationService<D: EmailDelivery> {
    pool: PgPool,
    email: D,
    mfa_protector: Aes256GcmSecretProtector,
    rate_limit_key: hmac::Key,
    public_base_url: String,
}

impl<D: EmailDelivery> AuthenticationService<D> {
    pub fn new(
        pool: PgPool,
        email: D,
        mfa_protector: Aes256GcmSecretProtector,
        rate_limit_secret: &[u8],
        public_base_url: String,
    ) -> Result<Self, AuthenticationError> {
        if rate_limit_secret.len() < 32
            || public_base_url.trim().is_empty()
            || !public_base_url.starts_with("https://")
        {
            return Err(AuthenticationError::InvalidConfiguration);
        }
        Ok(Self {
            pool,
            email,
            mfa_protector,
            rate_limit_key: hmac::Key::new(hmac::HMAC_SHA256, rate_limit_secret),
            public_base_url: public_base_url.trim_end_matches('/').to_owned(),
        })
    }

    pub async fn register(
        &self,
        email: &str,
        display_name: Option<&str>,
        password: &str,
    ) -> Result<(), AuthenticationError> {
        let email = normalize_email(email).map_err(|_| AuthenticationError::InvalidInput)?;
        let verifier = hash_password(password).map_err(|_| AuthenticationError::InvalidInput)?;
        let identity_id = Uuid::now_v7();
        let (token, token_hash) =
            SessionToken::generate().map_err(|_| AuthenticationError::Cryptography)?;
        let expires_at = SystemTime::now() + VERIFICATION_TOKEN_TTL;

        let mut transaction = self
            .pool
            .begin()
            .await
            .map_err(|_| AuthenticationError::Storage)?;
        ActorContext::new(ActorId::new(identity_id))
            .establish(&mut transaction)
            .await
            .map_err(|_| AuthenticationError::Storage)?;

        let inserted = sqlx::query(
            "INSERT INTO core.identities (id, email_normalized, email_display, display_name, status) VALUES ($1, $2, $2, $3, 'pending_verification') ON CONFLICT (email_normalized) DO NOTHING",
        )
        .bind(identity_id)
        .bind(&email)
        .bind(display_name)
        .execute(&mut *transaction)
        .await
        .map_err(|_| AuthenticationError::Storage)?
        .rows_affected();

        if inserted == 0 {
            transaction
                .rollback()
                .await
                .map_err(|_| AuthenticationError::Storage)?;
            return Ok(());
        }

        sqlx::query(
            "INSERT INTO core.identity_credentials (identity_id, password_hash) VALUES ($1, $2)",
        )
        .bind(identity_id)
        .bind(verifier.as_str())
        .execute(&mut *transaction)
        .await
        .map_err(|_| AuthenticationError::Storage)?;

        let issued: bool =
            sqlx::query_scalar("SELECT core.auth_issue_verification_token($1, $2, $3, to_timestamp($4::double precision))")
                .bind(identity_id)
                .bind(Uuid::now_v7())
                .bind(token_hash.as_bytes())
                .bind(unix_timestamp(expires_at)?)
                .fetch_one(&mut *transaction)
                .await
                .map_err(|_| AuthenticationError::Storage)?;
        if !issued {
            return Err(AuthenticationError::Storage);
        }
        transaction
            .commit()
            .await
            .map_err(|_| AuthenticationError::Storage)?;

        let link = format!(
            "{}/verify-email?token={}",
            self.public_base_url,
            token.as_str()
        );
        self.record_audit(Some(identity_id), "auth.identity.registration_requested", "success", Some("identity"), Some(identity_id))
            .await?;
        self.email
            .send(EmailMessage {
                recipient: email,
                subject: "Verifica tu cuenta ENSAMBRIA".into(),
                text_body: format!(
                    "Confirma tu correo abriendo este enlace de un solo uso, válido por 24 horas:\n{link}\nSi no solicitaste esta cuenta, ignora este mensaje."
                ),
            })
            .await
            .map_err(|_| {
                tracing::warn!(event = "auth.verification_email_delivery_failed");
            });
        Ok(())
    }

    pub async fn resend_verification(&self, email: &str) -> Result<(), AuthenticationError> {
        let email = normalize_email(email).map_err(|_| AuthenticationError::InvalidInput)?;
        self.enforce_rate_limit("recovery", &email).await?;
        let pending: Option<Uuid> =
            sqlx::query_scalar("SELECT identity_id FROM core.auth_lookup_pending_identity($1)")
                .bind(&email)
                .fetch_optional(&self.pool)
                .await
                .map_err(|_| AuthenticationError::Storage)?;
        let Some(identity_id) = pending else {
            return Ok(());
        };
        let (token, token_hash) =
            SessionToken::generate().map_err(|_| AuthenticationError::Cryptography)?;
        let issued: bool =
            sqlx::query_scalar("SELECT core.auth_issue_verification_token($1, $2, $3, to_timestamp($4::double precision))")
                .bind(identity_id)
                .bind(Uuid::now_v7())
                .bind(token_hash.as_bytes())
                .bind(unix_timestamp(SystemTime::now() + VERIFICATION_TOKEN_TTL)?)
                .fetch_one(&self.pool)
                .await
                .map_err(|_| AuthenticationError::Storage)?;
        if !issued {
            return Ok(());
        }
        let link = format!(
            "{}/verify-email?token={}",
            self.public_base_url,
            token.as_str()
        );
        self.email.send(EmailMessage {
            recipient: email,
            subject: "Verifica tu cuenta ENSAMBRIA".into(),
            text_body: format!("Confirma tu correo con este enlace de un solo uso, válido por 24 horas:\\n{link}"),
        }).await.map_err(|_| {
            tracing::warn!(event = "auth.verification_email_delivery_failed");
        });
        Ok(())
    }

    pub async fn login(
        &self,
        email: &str,
        password: &str,
    ) -> Result<LoginOutcome, AuthenticationError> {
        let email = normalize_email(email).map_err(|_| AuthenticationError::InvalidCredentials)?;
        self.enforce_rate_limit("login", &email).await?;

        let record: Option<(Uuid, String, String, bool, bool)> = sqlx::query_as(
            "SELECT identity_id, password_hash, identity_status, (email_verified_at IS NOT NULL), COALESCE(locked_until > clock_timestamp(), false) FROM core.auth_lookup_identity($1)",
        )
        .bind(&email)
        .fetch_optional(&self.pool)
        .await
        .map_err(|_| AuthenticationError::Storage)?;

        let Some((identity_id, encoded, status, verified, locked)) = record else {
            self.record_audit(None, "auth.login.failed", "failure", None, None)
                .await?;
            return Err(AuthenticationError::InvalidCredentials);
        };
        if status != "active" || !verified {
            self.record_audit(Some(identity_id), "auth.login.denied", "denied", Some("identity"), Some(identity_id))
                .await?;
            return Err(AuthenticationError::InvalidCredentials);
        }
        if locked {
            self.record_audit(Some(identity_id), "auth.login.locked", "denied", Some("identity"), Some(identity_id))
                .await?;
            return Err(AuthenticationError::TemporarilyLocked);
        }
        let verifier =
            PasswordHash::from_phc(&encoded).map_err(|_| AuthenticationError::Storage)?;
        if !verify_password(password, &verifier).map_err(|_| AuthenticationError::Cryptography)? {
            let _: bool =
                sqlx::query_scalar("SELECT core.auth_record_login_failure($1, 5, 900) IS NOT NULL")
                    .bind(identity_id)
                    .fetch_one(&self.pool)
                    .await
                    .map_err(|_| AuthenticationError::Storage)?;
            self.record_audit(Some(identity_id), "auth.login.failed", "failure", Some("identity"), Some(identity_id))
                .await?;
            return Err(AuthenticationError::InvalidCredentials);
        }

        let authenticated: Option<bool> = sqlx::query_scalar("SELECT core.auth_record_login_success($1)")
            .bind(identity_id)
            .fetch_optional(&self.pool)
            .await
            .map_err(|_| AuthenticationError::Storage)?;
        if authenticated != Some(true) {
            return Err(AuthenticationError::TemporarilyLocked);
        }

        let mfa_enabled: bool = sqlx::query_scalar("SELECT core.auth_mfa_is_enabled($1)")
            .bind(identity_id)
            .fetch_one(&self.pool)
            .await
            .map_err(|_| AuthenticationError::Storage)?;
        if mfa_enabled {
            let (challenge, challenge_hash) =
                SessionToken::generate().map_err(|_| AuthenticationError::Cryptography)?;
            let expires_at = SystemTime::now() + MFA_CHALLENGE_TTL;
            let issued: bool =
                sqlx::query_scalar("SELECT core.auth_issue_mfa_challenge($1, $2, $3, to_timestamp($4::double precision))")
                    .bind(Uuid::now_v7())
                    .bind(identity_id)
                    .bind(challenge_hash.as_bytes())
                   .bind(unix_timestamp(expires_at)?)
                    .fetch_one(&self.pool)
                    .await
                    .map_err(|_| AuthenticationError::Storage)?;
            if !issued {
                return Err(AuthenticationError::Storage);
            }
            self.record_audit(Some(identity_id), "auth.login.mfa_challenge", "success", Some("identity"), Some(identity_id))
                .await?;
            return Ok(LoginOutcome::MfaRequired {
                challenge_token: challenge.as_str().to_owned(),
            });
        }

        Ok(LoginOutcome::Authenticated(
            self.create_session(identity_id).await?,
        ))
    }

    pub async fn verify_mfa_challenge(
        &self,
        challenge_token: &str,
        factor: &str,
        unix_seconds: u64,
    ) -> Result<AuthSession, AuthenticationError> {
        let challenge_hash = SessionTokenHash::from_presented(challenge_token)
            .map_err(|_| AuthenticationError::InvalidChallenge)?;
        self.enforce_rate_limit("mfa", challenge_token).await?;
        let challenge: Option<(Uuid, bool, bool)> = sqlx::query_as(
            "SELECT identity_id, (expires_at > clock_timestamp()), (consumed_at IS NULL) FROM core.auth_find_mfa_challenge($1)",
        )
        .bind(challenge_hash.as_bytes())
        .fetch_optional(&self.pool)
        .await
        .map_err(|_| AuthenticationError::Storage)?;
        let Some((identity_id, not_expired, not_consumed)) = challenge else {
            return Err(AuthenticationError::InvalidChallenge);
        };
        if !not_expired || !not_consumed {
            return Err(AuthenticationError::InvalidChallenge);
        }

        let mut transaction = self
            .pool
            .begin()
            .await
            .map_err(|_| AuthenticationError::Storage)?;
        ActorContext::new(ActorId::new(identity_id))
            .establish(&mut transaction)
            .await
            .map_err(|_| AuthenticationError::Storage)?;

        let mut factor_valid = false;
        if factor.len() == 6 && factor.bytes().all(|byte| byte.is_ascii_digit()) {
            let row = sqlx::query(
                "SELECT encrypted_secret, encryption_key_version FROM core.identity_mfa_totp WHERE identity_id = $1 AND confirmed_at IS NOT NULL",
            )
            .bind(identity_id)
            .fetch_optional(&mut *transaction)
            .await
            .map_err(|_| AuthenticationError::Storage)?
            .ok_or(AuthenticationError::MfaNotEnabled)?;
            let encrypted = EncryptedSecret {
                ciphertext: row
                    .try_get("encrypted_secret")
                    .map_err(|_| AuthenticationError::Storage)?,
                key_version: row
                    .try_get("encryption_key_version")
                    .map_err(|_| AuthenticationError::Storage)?,
            };
            let secret_bytes = Zeroizing::new(
                self.mfa_protector
                    .open(identity_id, &encrypted)
                    .map_err(|_| AuthenticationError::Cryptography)?,
            );
            let secret_array: [u8; 20] = secret_bytes
                .as_slice()
                .try_into()
                .map_err(|_| AuthenticationError::Cryptography)?;
            let secret = TotpSecret::from_bytes(secret_array);
            if let Some(step) = secret.verify_at(factor, unix_seconds) {
                factor_valid = sqlx::query_scalar("SELECT core.auth_accept_totp_step($1, $2)")
                    .bind(identity_id)
                    .bind(step as i64)
                    .fetch_one(&mut *transaction)
                    .await
                    .map_err(|_| AuthenticationError::Storage)?;
            }
        } else {
            let code_hash = digest(&SHA256, factor.as_bytes());
            factor_valid = sqlx::query_scalar("SELECT core.auth_consume_mfa_recovery_code($1, $2)")
                .bind(identity_id)
                .bind(code_hash.as_ref())
                .fetch_one(&mut *transaction)
                .await
                .map_err(|_| AuthenticationError::Storage)?;
        }
        if !factor_valid {
            transaction
                .rollback()
                .await
                .map_err(|_| AuthenticationError::Storage)?;
            return Err(AuthenticationError::InvalidMfaFactor);
        }

        let consumed: Option<Uuid> =
            sqlx::query_scalar("SELECT core.auth_consume_mfa_challenge($1)")
                .bind(challenge_hash.as_bytes())
                .fetch_one(&mut *transaction)
                .await
                .map_err(|_| AuthenticationError::Storage)?;
        if consumed != Some(identity_id) {
            transaction
                .rollback()
                .await
                .map_err(|_| AuthenticationError::Storage)?;
            return Err(AuthenticationError::InvalidChallenge);
        }

        let session = self
            .create_session_in_transaction(identity_id, &mut transaction)
            .await?;
        transaction
            .commit()
            .await
            .map_err(|_| AuthenticationError::Storage)?;
        self.record_audit(Some(identity_id), "auth.session.created", "success", Some("identity"), Some(identity_id))
            .await?;
        Ok(session)
    }

    pub async fn authenticate_access_token(
        &self,
        presented: &str,
    ) -> Result<Uuid, AuthenticationError> {
        let hash = SessionTokenHash::from_presented(presented)
            .map_err(|_| AuthenticationError::InvalidAccessToken)?;
        let record: Option<(Uuid, bool, bool, String)> = sqlx::query_as(
            "SELECT identity_id, (access_expires_at > clock_timestamp()), (revoked_at IS NULL AND refresh_expires_at > clock_timestamp()), identity_status FROM core.auth_find_access_session($1)",
        )
        .bind(hash.as_bytes())
        .fetch_optional(&self.pool)
        .await
        .map_err(|_| AuthenticationError::Storage)?;
        match record {
            Some((identity_id, access_valid, session_valid, status))
                if access_valid && session_valid && status == "active" =>
            {
                Ok(identity_id)
            }
            _ => Err(AuthenticationError::InvalidAccessToken),
        }
    }

    pub async fn refresh_session(
        &self,
        refresh_token: &str,
    ) -> Result<AuthSession, AuthenticationError> {
        let old_hash = SessionTokenHash::from_presented(refresh_token)
            .map_err(|_| AuthenticationError::InvalidRefreshToken)?;
        let (access, access_hash) =
            SessionToken::generate().map_err(|_| AuthenticationError::Cryptography)?;
        let (refresh, refresh_hash) =
            SessionToken::generate().map_err(|_| AuthenticationError::Cryptography)?;
        let session_id = Uuid::now_v7();
        let now = SystemTime::now();
        let access_expiry = now + ACCESS_TOKEN_TTL;
        let refresh_expiry = now + REFRESH_TOKEN_TTL;

        let result: Option<(Option<Uuid>, bool, bool)> = sqlx::query_as(
            "SELECT identity_id, rotation_succeeded, reuse_detected FROM core.auth_rotate_refresh_token($1, $2, $3, to_timestamp($4::double precision), $5, to_timestamp($6::double precision))",
        )
        .bind(old_hash.as_bytes())
        .bind(session_id)
        .bind(access_hash.as_bytes())
        .bind(unix_timestamp(access_expiry)?)
        .bind(refresh_hash.as_bytes())
        .bind(unix_timestamp(refresh_expiry)?)
        .fetch_optional(&self.pool)
        .await
        .map_err(|_| AuthenticationError::Storage)?;
        match result {
            Some((Some(identity_id), true, false)) => {
                self.record_audit(Some(identity_id), "auth.session.refreshed", "success", Some("identity"), Some(identity_id))
                    .await?;
                Ok(AuthSession {
                    identity_id,
                    session_id,
                    access_token: access.as_str().to_owned(),
                    refresh_token: refresh.as_str().to_owned(),
                    access_expires_at: access_expiry,
                    refresh_expires_at: refresh_expiry,
                })
            }
            Some((Some(identity_id), false, true)) => {
                self.record_audit(Some(identity_id), "auth.refresh.reuse_detected", "failure", Some("identity"), Some(identity_id))
                    .await?;
                Err(AuthenticationError::RefreshReuseDetected)
            }
            _ => Err(AuthenticationError::InvalidRefreshToken),
        }
    }

    pub async fn request_recovery(&self, email: &str) -> Result<(), AuthenticationError> {
        let email = normalize_email(email).map_err(|_| AuthenticationError::InvalidInput)?;
        self.enforce_rate_limit("recovery", &email).await?;
        let identity: Option<Uuid> =
            sqlx::query_scalar("SELECT identity_id FROM core.auth_lookup_identity($1)")
                .bind(&email)
                .fetch_optional(&self.pool)
                .await
                .map_err(|_| AuthenticationError::Storage)?;
        let Some(identity_id) = identity else {
            self.record_audit(None, "auth.recovery.requested", "success", None, None)
                .await?;
            return Ok(());
        };

        let (token, token_hash) =
            SessionToken::generate().map_err(|_| AuthenticationError::Cryptography)?;
        let issued: bool =
            sqlx::query_scalar("SELECT core.auth_issue_recovery_token($1, $2, $3, to_timestamp($4::double precision))")
                .bind(identity_id)
                .bind(Uuid::now_v7())
                .bind(token_hash.as_bytes())
                .bind(unix_timestamp(SystemTime::now() + RECOVERY_TOKEN_TTL)?)
                .fetch_one(&self.pool)
                .await
                .map_err(|_| AuthenticationError::Storage)?;
        if !issued {
            return Ok(());
        }
        self.record_audit(Some(identity_id), "auth.recovery.requested", "success", Some("identity"), Some(identity_id))
            .await?;
        let link = format!(
            "{}/reset-password?token={}",
            self.public_base_url,
            token.as_str()
        );
        self.email
            .send(EmailMessage {
                recipient: email,
                subject: "Recupera tu cuenta ENSAMBRIA".into(),
                text_body: format!(
                    "Solicitaste restablecer tu contraseña. Usa este enlace de un solo uso, válido por una hora:\n{link}\nSi no solicitaste el cambio, ignora este mensaje."
                ),
            })
            .await
            .map_err(|_| {
                tracing::warn!(event = "auth.recovery_email_delivery_failed");
            });
        Ok(())
    }

    pub async fn complete_recovery(
        &self,
        token: &str,
        new_password: &str,
    ) -> Result<Uuid, AuthenticationError> {
        let verifier =
            hash_password(new_password).map_err(|_| AuthenticationError::InvalidInput)?;
        let token_hash = SessionTokenHash::from_presented(token)
            .map_err(|_| AuthenticationError::InvalidRecoveryToken)?;
        let identity: Option<Uuid> =
            sqlx::query_scalar("SELECT core.auth_complete_recovery($1, $2)")
                .bind(token_hash.as_bytes())
                .bind(verifier.as_str())
                .fetch_one(&self.pool)
                .await
                .map_err(|_| AuthenticationError::Storage)?;
        let identity = identity.ok_or(AuthenticationError::InvalidRecoveryToken)?;
        self.record_audit(Some(identity), "auth.recovery.completed", "success", Some("identity"), Some(identity))
            .await?;
        Ok(identity)
    }

    pub async fn verify_email(&self, token: &str) -> Result<Uuid, AuthenticationError> {
        let token_hash = SessionTokenHash::from_presented(token)
            .map_err(|_| AuthenticationError::InvalidVerificationToken)?;
        let identity: Option<Uuid> =
            sqlx::query_scalar("SELECT core.auth_consume_verification_token($1)")
                .bind(token_hash.as_bytes())
                .fetch_one(&self.pool)
                .await
                .map_err(|_| AuthenticationError::Storage)?;
        let identity = identity.ok_or(AuthenticationError::InvalidVerificationToken)?;
        self.record_audit(Some(identity), "auth.email.verified", "success", Some("identity"), Some(identity))
            .await?;
        Ok(identity)
    }

    pub async fn begin_totp_enrollment(
        &self,
        identity_id: Uuid,
    ) -> Result<String, AuthenticationError> {
        let secret = TotpSecret::generate().map_err(|_| AuthenticationError::Cryptography)?;
        let encrypted = self
            .mfa_protector
            .seal(identity_id, secret.as_bytes())
            .map_err(|_| AuthenticationError::Cryptography)?;
        let mut transaction = self
            .pool
            .begin()
            .await
            .map_err(|_| AuthenticationError::Storage)?;
        ActorContext::new(ActorId::new(identity_id))
            .establish(&mut transaction)
            .await
            .map_err(|_| AuthenticationError::Storage)?;
        let row = sqlx::query(
            "INSERT INTO core.identity_mfa_totp (identity_id, encrypted_secret, encryption_key_version) VALUES ($1, $2, $3) ON CONFLICT (identity_id) DO UPDATE SET encrypted_secret = EXCLUDED.encrypted_secret, encryption_key_version = EXCLUDED.encryption_key_version, confirmed_at = NULL, last_accepted_step = NULL, updated_at = clock_timestamp() WHERE core.identity_mfa_totp.confirmed_at IS NULL RETURNING identity_id",
        )
        .bind(identity_id)
        .bind(&encrypted.ciphertext)
        .bind(&encrypted.key_version)
        .fetch_optional(&mut *transaction)
        .await
        .map_err(|_| AuthenticationError::Storage)?;
        if row.is_none() {
            transaction
                .rollback()
                .await
                .map_err(|_| AuthenticationError::Storage)?;
            return Err(AuthenticationError::MfaAlreadyEnabled);
        }
        transaction
            .commit()
            .await
            .map_err(|_| AuthenticationError::Storage)?;
        self.record_audit(Some(identity_id), "auth.mfa.enrollment_started", "success", Some("identity"), Some(identity_id))
            .await?;
        Ok(format!(
            "otpauth://totp/ENSAMBRIA?secret={}&issuer=ENSAMBRIA&algorithm=SHA1&digits=6&period=30",
            secret.base32()
        ))
    }

    pub async fn confirm_totp_enrollment(
        &self,
        identity_id: Uuid,
        code: &str,
        unix_seconds: u64,
    ) -> Result<Vec<String>, AuthenticationError> {
        let mut transaction = self
            .pool
            .begin()
            .await
            .map_err(|_| AuthenticationError::Storage)?;
        ActorContext::new(ActorId::new(identity_id))
            .establish(&mut transaction)
            .await
            .map_err(|_| AuthenticationError::Storage)?;
        let row = sqlx::query(
            "SELECT encrypted_secret, encryption_key_version FROM core.identity_mfa_totp WHERE identity_id = $1 AND confirmed_at IS NULL",
        )
        .bind(identity_id)
        .fetch_optional(&mut *transaction)
        .await
        .map_err(|_| AuthenticationError::Storage)?
        .ok_or(AuthenticationError::MfaEnrollmentNotFound)?;
        let encrypted = EncryptedSecret {
            ciphertext: row
                .try_get("encrypted_secret")
                .map_err(|_| AuthenticationError::Storage)?,
            key_version: row
                .try_get("encryption_key_version")
                .map_err(|_| AuthenticationError::Storage)?,
        };
        let secret_bytes = Zeroizing::new(
            self.mfa_protector
                .open(identity_id, &encrypted)
                .map_err(|_| AuthenticationError::Cryptography)?,
        );
        let secret_array: [u8; 20] = secret_bytes
            .as_slice()
            .try_into()
            .map_err(|_| AuthenticationError::Cryptography)?;
        let secret = TotpSecret::from_bytes(secret_array);
        let step = secret
            .verify_at(code, unix_seconds)
            .ok_or(AuthenticationError::InvalidMfaFactor)?;
        let confirmed: bool = sqlx::query_scalar("SELECT core.auth_confirm_totp_step($1, $2)")
            .bind(identity_id)
            .bind(step as i64)
            .fetch_one(&mut *transaction)
            .await
            .map_err(|_| AuthenticationError::Storage)?;
        if !confirmed {
            return Err(AuthenticationError::InvalidMfaFactor);
        }

        let mut recovery_codes = Vec::with_capacity(MFA_RECOVERY_CODE_COUNT);
        for _ in 0..MFA_RECOVERY_CODE_COUNT {
            let (code, hash) = generate_recovery_code()?;
            sqlx::query(
                "INSERT INTO core.identity_mfa_recovery_codes (identity_id, code_hash) VALUES ($1, $2)",
            )
            .bind(identity_id)
            .bind(hash.as_ref())
            .execute(&mut *transaction)
            .await
            .map_err(|_| AuthenticationError::Storage)?;
            recovery_codes.push(code);
        }
        transaction
            .commit()
            .await
            .map_err(|_| AuthenticationError::Storage)?;
        self.record_audit(Some(identity_id), "auth.mfa.enabled", "success", Some("identity"), Some(identity_id))
            .await?;
        Ok(recovery_codes)
    }

    pub async fn disable_totp(
        &self,
        identity_id: Uuid,
        current_password: &str,
    ) -> Result<(), AuthenticationError> {
        let mut transaction = self
            .pool
            .begin()
            .await
            .map_err(|_| AuthenticationError::Storage)?;
        ActorContext::new(ActorId::new(identity_id))
            .establish(&mut transaction)
            .await
            .map_err(|_| AuthenticationError::Storage)?;
        let encoded: String = sqlx::query_scalar(
            "SELECT password_hash FROM core.identity_credentials WHERE identity_id = $1",
        )
        .bind(identity_id)
        .fetch_one(&mut *transaction)
        .await
        .map_err(|_| AuthenticationError::Storage)?;
        let verifier =
            PasswordHash::from_phc(&encoded).map_err(|_| AuthenticationError::Storage)?;
        if !verify_password(current_password, &verifier)
            .map_err(|_| AuthenticationError::Cryptography)?
        {
            return Err(AuthenticationError::InvalidCredentials);
        }
        sqlx::query("DELETE FROM core.identity_mfa_recovery_codes WHERE identity_id = $1")
            .bind(identity_id)
            .execute(&mut *transaction)
            .await
            .map_err(|_| AuthenticationError::Storage)?;
        sqlx::query("DELETE FROM core.identity_mfa_totp WHERE identity_id = $1")
            .bind(identity_id)
            .execute(&mut *transaction)
            .await
            .map_err(|_| AuthenticationError::Storage)?;
        transaction
            .commit()
            .await
            .map_err(|_| AuthenticationError::Storage)?;
        self.record_audit(Some(identity_id), "auth.mfa.disabled", "success", Some("identity"), Some(identity_id))
            .await?;
        Ok(())
    }

    pub async fn logout(&self, access_token: &str) -> Result<(), AuthenticationError> {
        let identity_id = self.authenticate_access_token(access_token).await?;
        let hash = SessionTokenHash::from_presented(access_token)
            .map_err(|_| AuthenticationError::InvalidAccessToken)?;
        let mut transaction = self
            .pool
            .begin()
            .await
            .map_err(|_| AuthenticationError::Storage)?;
        ActorContext::new(ActorId::new(identity_id))
            .establish(&mut transaction)
            .await
            .map_err(|_| AuthenticationError::Storage)?;
        sqlx::query(
            "UPDATE core.auth_sessions SET revoked_at = COALESCE(revoked_at, clock_timestamp()) WHERE access_token_hash = $1 AND identity_id = $2",
        )
        .bind(hash.as_bytes())
        .bind(identity_id)
        .execute(&mut *transaction)
        .await
        .map_err(|_| AuthenticationError::Storage)?;
        transaction
            .commit()
            .await
            .map_err(|_| AuthenticationError::Storage)?;
        self.record_audit(Some(identity_id), "auth.session.revoked", "success", Some("identity"), Some(identity_id))
            .await?;
        Ok(())
    }

    pub async fn logout_all(&self, identity_id: Uuid) -> Result<(), AuthenticationError> {
        let mut transaction = self
            .pool
            .begin()
            .await
            .map_err(|_| AuthenticationError::Storage)?;
        ActorContext::new(ActorId::new(identity_id))
            .establish(&mut transaction)
            .await
            .map_err(|_| AuthenticationError::Storage)?;
        sqlx::query(
            "UPDATE core.auth_sessions SET revoked_at = COALESCE(revoked_at, clock_timestamp()) WHERE identity_id = $1",
        )
        .bind(identity_id)
        .execute(&mut *transaction)
        .await
        .map_err(|_| AuthenticationError::Storage)?;
        transaction
            .commit()
            .await
            .map_err(|_| AuthenticationError::Storage)?;
        self.record_audit(Some(identity_id), "auth.sessions.revoked_all", "success", Some("identity"), Some(identity_id))
            .await?;
        Ok(())
    }

    async fn create_session(&self, identity_id: Uuid) -> Result<AuthSession, AuthenticationError> {
        let mut transaction = self
            .pool
            .begin()
            .await
            .map_err(|_| AuthenticationError::Storage)?;
        let session = self
            .create_session_in_transaction(identity_id, &mut transaction)
            .await?;
        transaction
            .commit()
            .await
            .map_err(|_| AuthenticationError::Storage)?;
        self.record_audit(Some(identity_id), "auth.session.created", "success", Some("identity"), Some(identity_id))
            .await?;
        Ok(session)
    }

    async fn create_session_in_transaction(
        &self,
        identity_id: Uuid,
        transaction: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    ) -> Result<AuthSession, AuthenticationError> {
        let (access, access_hash) =
            SessionToken::generate().map_err(|_| AuthenticationError::Cryptography)?;
        let (refresh, refresh_hash) =
            SessionToken::generate().map_err(|_| AuthenticationError::Cryptography)?;
        let session_id = Uuid::now_v7();
        let now = SystemTime::now();
        let access_expiry = now + ACCESS_TOKEN_TTL;
        let refresh_expiry = now + REFRESH_TOKEN_TTL;
        let created: bool =
            sqlx::query_scalar("SELECT core.auth_create_session($1, $2, $3, to_timestamp($4::double precision), $5, to_timestamp($6::double precision))")
                .bind(session_id)
                .bind(identity_id)
                .bind(access_hash.as_bytes())
               .bind(unix_timestamp(access_expiry)?)
                .bind(refresh_hash.as_bytes())
               .bind(unix_timestamp(refresh_expiry)?)
                .fetch_one(&mut **transaction)
                .await
                .map_err(|_| AuthenticationError::Storage)?;
        if !created {
            return Err(AuthenticationError::InvalidCredentials);
        }
        Ok(AuthSession {
            identity_id,
            session_id,
            access_token: access.as_str().to_owned(),
            refresh_token: refresh.as_str().to_owned(),
            access_expires_at: access_expiry,
            refresh_expires_at: refresh_expiry,
        })
    }

    async fn enforce_rate_limit(
        &self,
        action: &str,
        subject: &str,
    ) -> Result<(), AuthenticationError> {
        let input = format!("{action}:{subject}");
        let digest = hmac::sign(&self.rate_limit_key, input.as_bytes());
        let allowed: bool =
            sqlx::query_scalar("SELECT core.auth_consume_rate_limit($1, $2, $3, $4, $5)")
                .bind(digest.as_ref())
                .bind(action)
                .bind(RATE_WINDOW_SECONDS)
                .bind(RATE_MAX_ATTEMPTS)
                .bind(RATE_BLOCK_SECONDS)
                .fetch_one(&self.pool)
                .await
                .map_err(|_| AuthenticationError::Storage)?;
        if allowed {
            Ok(())
        } else {
            Err(AuthenticationError::RateLimited)
        }
    }
    async fn record_audit(
        &self,
        actor_id: Option<Uuid>,
        action: &str,
        outcome: &str,
        target_type: Option<&str>,
        target_id: Option<Uuid>,
    ) -> Result<(), AuthenticationError> {
        sqlx::query(
            "SELECT core.auth_record_security_event($1, $2, $3, $4, $5, $6, $7, $8::jsonb)",
        )
        .bind(Uuid::now_v7())
        .bind(actor_id)
        .bind(action)
        .bind(outcome)
        .bind(Uuid::now_v7())
        .bind(target_type)
        .bind(target_id)
        .bind("{}")
        .fetch_one(&self.pool)
        .await
        .map_err(|_| AuthenticationError::Storage)?;
        Ok(())
    }

}

pub enum LoginOutcome {
    Authenticated(AuthSession),
    MfaRequired { challenge_token: String },
}

pub struct AuthSession {
    pub identity_id: Uuid,
    pub session_id: Uuid,
    pub access_token: String,
    pub refresh_token: String,
    pub access_expires_at: SystemTime,
    pub refresh_expires_at: SystemTime,
}

impl fmt::Debug for AuthSession {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("AuthSession")
            .field("identity_id", &self.identity_id)
            .field("session_id", &self.session_id)
            .field("access_token", &"[REDACTED]")
            .field("refresh_token", &"[REDACTED]")
            .field("access_expires_at", &self.access_expires_at)
            .field("refresh_expires_at", &self.refresh_expires_at)
            .finish()
    }
}


fn unix_timestamp(value: SystemTime) -> Result<i64, AuthenticationError> {
    let seconds = value
        .duration_since(UNIX_EPOCH)
        .map_err(|_| AuthenticationError::Cryptography)?
        .as_secs();
    i64::try_from(seconds).map_err(|_| AuthenticationError::Cryptography)
}

fn generate_recovery_code() -> Result<(String, [u8; 32]), AuthenticationError> {
    use ring::rand::{SecureRandom, SystemRandom};
    let mut random = [0; 16];
    SystemRandom::new()
        .fill(&mut random)
        .map_err(|_| AuthenticationError::Cryptography)?;
    let mut code = String::with_capacity(32);
    for byte in random {
        use fmt::Write;
        write!(&mut code, "{byte:02X}").map_err(|_| AuthenticationError::Cryptography)?;
    }
    let hash = digest(&SHA256, code.as_bytes());
    let mut stored = [0; 32];
    stored.copy_from_slice(hash.as_ref());
    Ok((code, stored))
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum AuthenticationError {
    InvalidConfiguration,
    InvalidInput,
    InvalidCredentials,
    TemporarilyLocked,
    InvalidAccessToken,
    InvalidRefreshToken,
    RefreshReuseDetected,
    InvalidRecoveryToken,
    InvalidVerificationToken,
    InvalidChallenge,
    InvalidMfaFactor,
    MfaNotEnabled,
    MfaAlreadyEnabled,
    MfaEnrollmentNotFound,
    RateLimited,
    Cryptography,
    Storage,
}

impl fmt::Display for AuthenticationError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(match self {
            Self::InvalidConfiguration => "authentication service configuration is invalid",
            Self::InvalidInput => "authentication input is invalid",
            Self::InvalidCredentials => "email or password is invalid",
            Self::TemporarilyLocked => "authentication is temporarily unavailable",
            Self::InvalidAccessToken => "access token is invalid or expired",
            Self::InvalidRefreshToken => "refresh token is invalid or expired",
            Self::RefreshReuseDetected => "refresh token reuse detected; sessions were revoked",
            Self::InvalidRecoveryToken => "recovery token is invalid or expired",
            Self::InvalidVerificationToken => "verification token is invalid or expired",
            Self::InvalidChallenge => "MFA challenge is invalid or expired",
            Self::InvalidMfaFactor => "MFA factor is invalid",
            Self::MfaNotEnabled => "MFA is not enabled for this identity",
            Self::MfaAlreadyEnabled => "MFA is already enabled",
            Self::MfaEnrollmentNotFound => "MFA enrollment was not found",
            Self::RateLimited => "too many authentication attempts",
            Self::Cryptography => "authentication cryptographic operation failed",
            Self::Storage => "authentication persistence operation failed",
        })
    }
}
impl Error for AuthenticationError {}

pub fn unix_time_now() -> Result<u64, AuthenticationError> {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|duration| duration.as_secs())
        .map_err(|_| AuthenticationError::Cryptography)
}

#[cfg(test)]
mod smtp_tests {
    use super::SmtpEmailDelivery;

    #[test]
    fn smtp_transport_requires_authenticated_starttls_configuration() {
        assert!(
            SmtpEmailDelivery::new(
                "smtp.example.test",
                587,
                "mailer".into(),
                "secret".into(),
                "no-reply@example.test",
            )
            .is_ok()
        );
        assert!(
            SmtpEmailDelivery::new(
                "https://smtp.example.test",
                587,
                "mailer".into(),
                "secret".into(),
                "no-reply@example.test",
            )
            .is_err()
        );
        assert!(
            SmtpEmailDelivery::new(
                "smtp.example.test",
                0,
                "mailer".into(),
                "secret".into(),
                "no-reply@example.test",
            )
            .is_err()
        );
    }
}
