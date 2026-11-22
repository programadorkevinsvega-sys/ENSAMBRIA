use std::{env, error::Error, fmt};

use sqlx::PgPool;
use zeroize::Zeroizing;

use super::{
    email_outbox::OutboxEmailDelivery,
    service::AuthenticationService,
};
use crate::identity_authentication::crypto::secret_encryption::Aes256GcmSecretProtector;

/// Builds the production authentication service with durable email enqueueing.
/// Secrets are injected by the deployment secret manager as hexadecimal values.
pub fn authentication_service_from_env(
    pool: PgPool,
) -> Result<AuthenticationService<OutboxEmailDelivery>, ProductionAuthenticationConfigurationError> {
    let mfa_key_version = required("CORE_MFA_ENCRYPTION_KEY_VERSION")?;
    let mfa_key = decode_hex_secret("CORE_MFA_ENCRYPTION_KEY_HEX", 32)?;
    let rate_limit_secret = decode_hex_secret("CORE_AUTH_RATE_LIMIT_SECRET_HEX", 32)?;
    let public_base_url = required("CORE_PUBLIC_BASE_URL")?;

    let protector = Aes256GcmSecretProtector::new(mfa_key_version, mfa_key.as_slice())
        .map_err(|_| ProductionAuthenticationConfigurationError::InvalidSecret)?;
    AuthenticationService::new(
        pool.clone(),
        OutboxEmailDelivery::new(pool),
        protector,
        rate_limit_secret.as_slice(),
        public_base_url,
    )
    .map_err(|_| ProductionAuthenticationConfigurationError::InvalidPublicConfiguration)
}

fn required(name: &'static str) -> Result<String, ProductionAuthenticationConfigurationError> {
    let value = env::var(name)
        .map_err(|_| ProductionAuthenticationConfigurationError::MissingVariable(name))?;
    if value.trim().is_empty() {
        return Err(ProductionAuthenticationConfigurationError::InvalidVariable(name));
    }
    Ok(value)
}

fn decode_hex_secret(
    name: &'static str,
    minimum_bytes: usize,
) -> Result<Zeroizing<Vec<u8>>, ProductionAuthenticationConfigurationError> {
    let encoded = required(name)?;
    if encoded.len() % 2 != 0 || encoded.len() / 2 < minimum_bytes {
        return Err(ProductionAuthenticationConfigurationError::InvalidSecret);
    }
    let mut decoded = Zeroizing::new(Vec::with_capacity(encoded.len() / 2));
    for pair in encoded.as_bytes().as_chunks::<2>().0 {
        let high = hex_nibble(pair[0]).ok_or(ProductionAuthenticationConfigurationError::InvalidSecret)?;
        let low = hex_nibble(pair[1]).ok_or(ProductionAuthenticationConfigurationError::InvalidSecret)?;
        decoded.push((high << 4) | low);
    }
    Ok(decoded)
}

fn hex_nibble(value: u8) -> Option<u8> {
    match value {
        b'0'..=b'9' => Some(value - b'0'),
        b'a'..=b'f' => Some(value - b'a' + 10),
        b'A'..=b'F' => Some(value - b'A' + 10),
        _ => None,
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ProductionAuthenticationConfigurationError {
    MissingVariable(&'static str),
    InvalidVariable(&'static str),
    InvalidSecret,
    InvalidPublicConfiguration,
}

impl fmt::Display for ProductionAuthenticationConfigurationError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::MissingVariable(name) => write!(formatter, "required environment variable {name} is missing"),
            Self::InvalidVariable(name) => write!(formatter, "environment variable {name} is invalid"),
            Self::InvalidSecret => formatter.write_str("authentication secret configuration is invalid"),
            Self::InvalidPublicConfiguration => formatter.write_str("public authentication configuration is invalid"),
        }
    }
}
impl Error for ProductionAuthenticationConfigurationError {}

#[cfg(test)]
mod tests {
    use super::hex_nibble;

    #[test]
    fn hex_decoder_accepts_only_hexadecimal_digits() {
        assert_eq!(hex_nibble(b'0'), Some(0));
        assert_eq!(hex_nibble(b'9'), Some(9));
        assert_eq!(hex_nibble(b'a'), Some(10));
        assert_eq!(hex_nibble(b'F'), Some(15));
        assert_eq!(hex_nibble(b'x'), None);
    }
}
