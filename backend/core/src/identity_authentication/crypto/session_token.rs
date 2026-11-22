use std::fmt;

use ring::{
    digest::{SHA256, digest},
    rand::{SecureRandom, SystemRandom},
};

const TOKEN_BYTES: usize = 32;
const TOKEN_HEX_LENGTH: usize = TOKEN_BYTES * 2;

pub struct SessionToken(String);

impl SessionToken {
    pub fn generate() -> Result<(Self, SessionTokenHash), SessionTokenError> {
        let mut bytes = [0; TOKEN_BYTES];
        SystemRandom::new()
            .fill(&mut bytes)
            .map_err(|_| SessionTokenError::RandomGeneration)?;
        let token = Self(hex_encode(&bytes));
        let hash = SessionTokenHash::from_presented(token.as_str())?;
        Ok((token, hash))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Debug for SessionToken {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("SessionToken([REDACTED])")
    }
}

#[derive(Clone, Eq, PartialEq)]
pub struct SessionTokenHash([u8; 32]);

impl SessionTokenHash {
    pub fn from_presented(token: &str) -> Result<Self, SessionTokenError> {
        if token.len() != TOKEN_HEX_LENGTH
            || !token.bytes().all(|byte| byte.is_ascii_hexdigit())
        {
            return Err(SessionTokenError::InvalidToken);
        }
        let digest = digest(&SHA256, token.as_bytes());
        let mut hash = [0; 32];
        hash.copy_from_slice(digest.as_ref());
        Ok(Self(hash))
    }

    pub fn as_bytes(&self) -> &[u8] {
        &self.0
    }
}

impl fmt::Debug for SessionTokenHash {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("SessionTokenHash([REDACTED])")
    }
}

fn hex_encode(bytes: &[u8]) -> String {
    let mut output = String::with_capacity(bytes.len() * 2);
    for byte in bytes {
        use fmt::Write;
        let _ = write!(output, "{byte:02x}");
    }
    output
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SessionTokenError {
    InvalidToken,
    RandomGeneration,
}

impl fmt::Display for SessionTokenError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(match self {
            Self::InvalidToken => "session token is invalid",
            Self::RandomGeneration => "secure session token generation failed",
        })
    }
}

impl std::error::Error for SessionTokenError {}

#[cfg(test)]
mod tests {
    use super::{SessionToken, SessionTokenHash};

    #[test]
    fn generates_high_entropy_opaque_tokens_and_redacts_debug() {
        let (token, stored_hash) = SessionToken::generate().expect("token must be generated");
        assert_eq!(token.as_str().len(), 64);
        assert_eq!(
            stored_hash,
            SessionTokenHash::from_presented(token.as_str()).unwrap()
        );
        assert!(!format!("{token:?}").contains(token.as_str()));
        assert!(!format!("{stored_hash:?}").contains(token.as_str()));
    }

    #[test]
    fn rejects_malformed_presented_tokens() {
        assert!(SessionTokenHash::from_presented("not-a-session-token").is_err());
    }
}
