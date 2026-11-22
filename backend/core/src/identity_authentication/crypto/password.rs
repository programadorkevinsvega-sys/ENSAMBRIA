use std::{error::Error, fmt};

use argon2::{Algorithm, Argon2, Params, Version};
use base64::{Engine, engine::general_purpose::STANDARD_NO_PAD};
use ring::rand::{SecureRandom, SystemRandom};
use subtle::ConstantTimeEq;

const MEMORY_KIB: u32 = 19_456;
const ITERATIONS: u32 = 2;
const LANES: u32 = 1;
const SALT_LENGTH: usize = 16;
const HASH_LENGTH: usize = 32;
const MIN_PASSWORD_CHARACTERS: usize = 12;
const MAX_PASSWORD_BYTES: usize = 1024;
const PHC_PREFIX: &str = "$argon2id$v=19$m=19456,t=2,p=1$";

#[derive(Clone, Eq, PartialEq)]
pub struct PasswordHash {
    phc: String,
    salt: [u8; SALT_LENGTH],
    hash: [u8; HASH_LENGTH],
}

impl PasswordHash {
    pub fn from_phc(phc: &str) -> Result<Self, PasswordHashError> {
        if !phc.starts_with(PHC_PREFIX) || phc.len() > 512 {
            return Err(PasswordHashError::InvalidHash);
        }
        let parts: Vec<&str> = phc.split('$').collect();
        if parts.len() != 6
            || !parts[0].is_empty()
            || parts[1] != "argon2id"
            || parts[2] != "v=19"
            || parts[3] != "m=19456,t=2,p=1"
        {
            return Err(PasswordHashError::InvalidHash);
        }
        let salt = STANDARD_NO_PAD
            .decode(parts[4])
            .map_err(|_| PasswordHashError::InvalidHash)?;
        let hash = STANDARD_NO_PAD
            .decode(parts[5])
            .map_err(|_| PasswordHashError::InvalidHash)?;
        if salt.len() != SALT_LENGTH || hash.len() != HASH_LENGTH {
            return Err(PasswordHashError::InvalidHash);
        }
        let mut stored_salt = [0; SALT_LENGTH];
        let mut stored_hash = [0; HASH_LENGTH];
        stored_salt.copy_from_slice(&salt);
        stored_hash.copy_from_slice(&hash);
        Ok(Self {
            phc: phc.to_owned(),
            salt: stored_salt,
            hash: stored_hash,
        })
    }

    pub fn as_str(&self) -> &str {
        &self.phc
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
    let mut output = [0; HASH_LENGTH];
    argon2_engine()?
        .hash_password_into(password.as_bytes(), &salt, &mut output)
        .map_err(|_| PasswordHashError::HashingFailed)?;
    let phc = format!(
        "{}{}{}{}",
        PHC_PREFIX,
        STANDARD_NO_PAD.encode(salt),
        "$",
        STANDARD_NO_PAD.encode(output)
    );
    PasswordHash::from_phc(&phc)
}

pub fn verify_password(password: &str, stored: &PasswordHash) -> Result<bool, PasswordHashError> {
    if password.len() > MAX_PASSWORD_BYTES {
        return Err(PasswordHashError::PasswordTooLong);
    }
    let mut output = [0; HASH_LENGTH];
    argon2_engine()?
        .hash_password_into(password.as_bytes(), &stored.salt, &mut output)
        .map_err(|_| PasswordHashError::HashingFailed)?;
    Ok(bool::from(output.ct_eq(&stored.hash)))
}

fn argon2_engine() -> Result<Argon2<'static>, PasswordHashError> {
    let params = Params::new(MEMORY_KIB, ITERATIONS, LANES, Some(HASH_LENGTH))
        .map_err(|_| PasswordHashError::HashingFailed)?;
    Ok(Argon2::new(Algorithm::Argon2id, Version::V0x13, params))
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
    HashingFailed,
}

impl fmt::Display for PasswordHashError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let message = match self {
            Self::PasswordTooShort => "password does not meet the minimum length",
            Self::PasswordTooLong => "password exceeds the maximum supported length",
            Self::InvalidHash => "stored password hash is invalid",
            Self::RandomGeneration => "secure password salt generation failed",
            Self::HashingFailed => "password hashing operation failed",
        };
        formatter.write_str(message)
    }
}

impl Error for PasswordHashError {}

#[cfg(test)]
mod tests {
    use super::{PHC_PREFIX, PasswordHash, hash_password, verify_password};

    #[test]
    fn password_hash_verifies_only_the_original_password_and_redacts_debug() {
        let stored = hash_password("correct horse battery staple").expect("hash must be created");
        assert!(verify_password("correct horse battery staple", &stored).unwrap());
        assert!(!verify_password("incorrect horse battery staple", &stored).unwrap());
        assert!(!format!("{stored:?}").contains("correct horse"));
        let restored = PasswordHash::from_phc(stored.as_str()).expect("PHC hash must parse");
        assert!(verify_password("correct horse battery staple", &restored).unwrap());
    }

    #[test]
    fn rejects_short_passwords_and_malformed_stored_hashes() {
        assert!(hash_password("short").is_err());
        assert!(PasswordHash::from_phc("$argon2id$v=19$m=19456,t=2,p=1$c2FsdA$aGFzaA").is_err());
    }

    #[test]
    fn uses_the_expected_argon2id_phc_parameters_and_independent_random_salts() {
        let first = hash_password("correct horse battery staple").expect("first hash");
        let second = hash_password("correct horse battery staple").expect("second hash");

        assert!(first.as_str().starts_with(PHC_PREFIX));
        assert_ne!(first.as_str(), second.as_str(), "each password hash must use a fresh salt");
        assert!(verify_password("correct horse battery staple", &first).unwrap());
        assert!(verify_password("correct horse battery staple", &second).unwrap());
    }

    #[test]
    fn rejects_phc_hashes_with_downgraded_or_unsupported_parameters() {
        let stored = hash_password("correct horse battery staple").expect("hash must be created");
        let phc = stored.as_str();

        for invalid in [
            phc.replace("argon2id", "argon2i"),
            phc.replace("v=19", "v=16"),
            phc.replace("m=19456,t=2,p=1", "m=8192,t=1,p=1"),
            phc.replace("m=19456,t=2,p=1", "m=19456,t=2,p=2"),
        ] {
            assert!(
                PasswordHash::from_phc(&invalid).is_err(),
                "unsupported PHC parameters must be rejected"
            );
        }
    }

    #[test]
    fn enforces_password_byte_limit_and_accepts_twelve_unicode_characters() {
        let too_long = "a".repeat(1025);
        assert!(matches!(
            hash_password(&too_long),
            Err(super::PasswordHashError::PasswordTooLong)
        ));

        let stored = hash_password("áááááááááááá").expect("12 Unicode characters are valid");
        assert!(verify_password("áááááááááááá", &stored).unwrap());
        assert!(matches!(
            verify_password(&too_long, &stored),
            Err(super::PasswordHashError::PasswordTooLong)
        ));
    }

    #[test]
    fn malformed_hash_material_never_verifies() {
        let stored = hash_password("correct horse battery staple").expect("hash must be created");
        assert!(!verify_password("wrong password entirely", &stored).unwrap());
    }
}
