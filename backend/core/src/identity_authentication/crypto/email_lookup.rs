use std::fmt;

use ring::hmac;

const MINIMUM_KEY_LENGTH: usize = 32;
const MAX_EMAIL_LENGTH: usize = 320;

pub struct EmailLookupKey {
    key: hmac::Key,
}

impl EmailLookupKey {
    pub fn new(secret: &[u8]) -> Result<Self, EmailLookupError> {
        if secret.len() < MINIMUM_KEY_LENGTH {
            return Err(EmailLookupError::InvalidKey);
        }
        Ok(Self {
            key: hmac::Key::new(&hmac::HMAC_SHA256, secret),
        })
    }

    pub fn digest(&self, normalized_email: &str) -> [u8; 32] {
        let tag = hmac::sign(&self.key, normalized_email.as_bytes());
        let mut digest = [0; 32];
        digest.copy_from_slice(tag.as_ref());
        digest
    }
}

pub fn normalize_email(value: &str) -> Result<String, EmailLookupError> {
    let normalized = value.trim().to_lowercase();
    if normalized.len() > MAX_EMAIL_LENGTH
        || normalized.chars().any(char::is_whitespace)
        || normalized.matches('@').count() != 1
    {
        return Err(EmailLookupError::InvalidEmail);
    }
    let (local, domain) = normalized
        .split_once('@')
        .ok_or(EmailLookupError::InvalidEmail)?;
    if local.is_empty() || domain.is_empty() || local.len() > 64 || domain.len() > 255 {
        return Err(EmailLookupError::InvalidEmail);
    }
    Ok(normalized)
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum EmailLookupError {
    InvalidKey,
    InvalidEmail,
}

impl fmt::Display for EmailLookupError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(match self {
            Self::InvalidKey => "identity lookup key is not configured securely",
            Self::InvalidEmail => "email address is invalid",
        })
    }
}

impl std::error::Error for EmailLookupError {}

#[cfg(test)]
mod tests {
    use super::{EmailLookupError, EmailLookupKey, normalize_email};

    #[test]
    fn email_lookup_is_normalized_and_keyed() {
        let key = EmailLookupKey::new(&[7; 32]).expect("a 256-bit key must be accepted");
        let normalized = normalize_email("  Person@Example.Test ").unwrap();
        assert_eq!(normalized, "person@example.test");
        assert_eq!(key.digest(&normalized), key.digest("person@example.test"));
        assert_ne!(key.digest(&normalized), key.digest("other@example.test"));
    }

    #[test]
    fn rejects_short_lookup_keys_and_malformed_emails() {
        assert_eq!(
            EmailLookupKey::new(&[1; 16]).err(),
            Some(EmailLookupError::InvalidKey)
        );
        assert!(normalize_email("missing-at-sign").is_err());
        assert!(normalize_email("two@@example.test").is_err());
    }
}
