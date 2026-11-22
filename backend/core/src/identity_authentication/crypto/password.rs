use std::{error::Error, fmt, num::NonZeroU32};

use ring::{
    pbkdf2,
    rand::{SecureRandom, SystemRandom},
};

const ITERATIONS: u32 = 600_000;
const SALT_LENGTH: usize = 16;
const HASH_LENGTH: usize = 32;
const MIN_PASSWORD_CHARACTERS: usize = 12;
const MAX_PASSWORD_BYTES: usize = 1024;

#[derive(Clone, Eq, PartialEq)]
pub struct PasswordHash {
    iterations: u32,
    salt: [u8; SALT_LENGTH],
    hash: [u8; HASH_LENGTH],
}

impl PasswordHash {
    pub fn from_parts(
        iterations: u32,
        salt: &[u8],
        hash: &[u8],
    ) -> Result<Self, PasswordHashError> {
        if iterations != ITERATIONS || salt.len() != SALT_LENGTH || hash.len() != HASH_LENGTH {
            return Err(PasswordHashError::InvalidHash);
        }
        let mut stored_salt = [0; SALT_LENGTH];
        let mut stored_hash = [0; HASH_LENGTH];
        stored_salt.copy_from_slice(salt);
        stored_hash.copy_from_slice(hash);
        Ok(Self {
            iterations,
            salt: stored_salt,
            hash: stored_hash,
        })
    }

    pub fn iterations(&self) -> u32 {
        self.iterations
    }

    pub fn salt(&self) -> &[u8] {
        &self.salt
    }

    pub fn hash(&self) -> &[u8] {
        &self.hash
    }
}

impl fmt::Debug for PasswordHash {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("PasswordHash([REDACTED])")
    }
}

pub fn hash_password(password: &str) -> Result<PasswordHash, PasswordHashError> {
    validate_new_password(password)?;
    let mut salt = [0; SALT_LENGTH];
    SystemRandom::new()
        .fill(&mut salt)
        .map_err(|_| PasswordHashError::RandomGeneration)?;
    let mut hash = [0; HASH_LENGTH];
    pbkdf2::derive(
        &pbkdf2::PBKDF2_HMAC_SHA256,
        NonZeroU32::new(ITERATIONS).ok_or(PasswordHashError::InvalidHash)?,
        &salt,
        password.as_bytes(),
        &mut hash,
    );
    Ok(PasswordHash {
        iterations: ITERATIONS,
        salt,
        hash,
    })
}

pub fn verify_password(password: &str, stored: &PasswordHash) -> Result<bool, PasswordHashError> {
    if password.len() > MAX_PASSWORD_BYTES {
        return Err(PasswordHashError::PasswordTooLong);
    }
    let iterations = NonZeroU32::new(stored.iterations).ok_or(PasswordHashError::InvalidHash)?;
    Ok(pbkdf2::verify(
        &pbkdf2::PBKDF2_HMAC_SHA256,
        iterations,
        &stored.salt,
        password.as_bytes(),
        &stored.hash,
    )
    .is_ok())
}

fn validate_new_password(password: &str) -> Result<(), PasswordHashError> {
    if password.chars().count() < MIN_PASSWORD_CHARACTERS {
        return Err(PasswordHashError::PasswordTooShort);
    }
    if password.len() > MAX_PASSWORD_BYTES {
        return Err(PasswordHashError::PasswordTooLong);
    }
    Ok(())
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PasswordHashError {
    PasswordTooShort,
    PasswordTooLong,
    InvalidHash,
    RandomGeneration,
}

impl fmt::Display for PasswordHashError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let message = match self {
            Self::PasswordTooShort => "password does not meet the minimum length",
            Self::PasswordTooLong => "password exceeds the maximum supported length",
            Self::InvalidHash => "stored password hash is invalid",
            Self::RandomGeneration => "secure password salt generation failed",
        };
        formatter.write_str(message)
    }
}

impl Error for PasswordHashError {}

#[cfg(test)]
mod tests {
    use super::{PasswordHash, hash_password, verify_password};

    #[test]
    fn password_hash_verifies_only_the_original_password_and_redacts_debug() {
        let stored = hash_password("correct horse battery staple").expect("hash must be created");
        assert!(verify_password("correct horse battery staple", &stored).unwrap());
        assert!(!verify_password("incorrect horse battery staple", &stored).unwrap());
        assert!(!format!("{stored:?}").contains("correct horse"));
        assert_eq!(stored.iterations(), 600_000);
        let restored = PasswordHash::from_parts(stored.iterations(), stored.salt(), stored.hash())
            .expect("stored parts must round-trip");
        assert!(verify_password("correct horse battery staple", &restored).unwrap());
    }

    #[test]
    fn rejects_short_passwords_and_malformed_stored_hashes() {
        assert!(hash_password("short").is_err());
        assert!(PasswordHash::from_parts(1, &[0; 16], &[0; 32]).is_err());
    }
}
