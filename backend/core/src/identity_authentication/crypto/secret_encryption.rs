use std::fmt;

use ring::{
    aead::{self, Aad, LessSafeKey, Nonce, UnboundKey},
    rand::{SecureRandom, SystemRandom},
};
use uuid::Uuid;

const NONCE_LENGTH: usize = 12;
const TAG_LENGTH: usize = 16;

pub struct Aes256GcmSecretProtector {
    key: LessSafeKey,
    key_version: String,
}

pub struct EncryptedSecret {
    pub ciphertext: Vec<u8>,
    pub key_version: String,
}

impl fmt::Debug for EncryptedSecret {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("EncryptedSecret")
            .field("ciphertext", &"[REDACTED]")
            .field("key_version", &self.key_version)
            .finish()
    }
}

impl Aes256GcmSecretProtector {
    pub fn new(key_version: String, key_bytes: &[u8]) -> Result<Self, SecretProtectionError> {
        if key_version.trim().is_empty() || key_version.len() > 64 || key_bytes.len() != 32 {
            return Err(SecretProtectionError::InvalidKeyConfiguration);
        }
        let unbound = UnboundKey::new(&aead::AES_256_GCM, key_bytes)
            .map_err(|_| SecretProtectionError::InvalidKeyConfiguration)?;
        Ok(Self {
            key: LessSafeKey::new(unbound),
            key_version,
        })
    }

    pub fn seal(
        &self,
        identity_id: Uuid,
        plaintext: &[u8],
    ) -> Result<EncryptedSecret, SecretProtectionError> {
        if plaintext.is_empty() || plaintext.len() > 4096 {
            return Err(SecretProtectionError::InvalidPlaintext);
        }
        let mut nonce_bytes = [0; NONCE_LENGTH];
        SystemRandom::new()
            .fill(&mut nonce_bytes)
            .map_err(|_| SecretProtectionError::RandomGeneration)?;
        let nonce = Nonce::assume_unique_for_key(nonce_bytes);
        let aad = associated_data(&self.key_version, identity_id);
        let mut ciphertext = plaintext.to_vec();
        self.key
            .seal_in_place_append_tag(nonce, Aad::from(aad.as_slice()), &mut ciphertext)
            .map_err(|_| SecretProtectionError::EncryptionFailed)?;
        let mut stored = Vec::with_capacity(NONCE_LENGTH + ciphertext.len());
        stored.extend_from_slice(&nonce_bytes);
        stored.extend_from_slice(&ciphertext);
        Ok(EncryptedSecret {
            ciphertext: stored,
            key_version: self.key_version.clone(),
        })
    }

    pub fn open(
        &self,
        identity_id: Uuid,
        encrypted: &EncryptedSecret,
    ) -> Result<Vec<u8>, SecretProtectionError> {
        if encrypted.key_version != self.key_version
            || encrypted.ciphertext.len() < NONCE_LENGTH + TAG_LENGTH + 1
            || encrypted.ciphertext.len() > NONCE_LENGTH + 4096 + TAG_LENGTH
        {
            return Err(SecretProtectionError::InvalidCiphertext);
        }
        let mut nonce_bytes = [0; NONCE_LENGTH];
        nonce_bytes.copy_from_slice(&encrypted.ciphertext[..NONCE_LENGTH]);
        let nonce = Nonce::assume_unique_for_key(nonce_bytes);
        let aad = associated_data(&self.key_version, identity_id);
        let mut ciphertext = encrypted.ciphertext[NONCE_LENGTH..].to_vec();
        let plaintext = self
            .key
            .open_in_place(nonce, Aad::from(aad.as_slice()), &mut ciphertext)
            .map_err(|_| SecretProtectionError::AuthenticationFailed)?;
        Ok(plaintext.to_vec())
    }
}

fn associated_data(key_version: &str, identity_id: Uuid) -> Vec<u8> {
    format!("ENSAMBRIA:CORE:MFA:{key_version}:{identity_id}").into_bytes()
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SecretProtectionError {
    InvalidKeyConfiguration,
    InvalidPlaintext,
    RandomGeneration,
    EncryptionFailed,
    InvalidCiphertext,
    AuthenticationFailed,
}

impl fmt::Display for SecretProtectionError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(match self {
            Self::InvalidKeyConfiguration => "MFA encryption key configuration is invalid",
            Self::InvalidPlaintext => "MFA secret payload is invalid",
            Self::RandomGeneration => "secure nonce generation failed",
            Self::EncryptionFailed => "MFA secret encryption failed",
            Self::InvalidCiphertext => "MFA secret ciphertext is invalid",
            Self::AuthenticationFailed => "MFA secret authentication failed",
        })
    }
}

impl std::error::Error for SecretProtectionError {}

#[cfg(test)]
mod tests {
    use super::{Aes256GcmSecretProtector, SecretProtectionError};
    use uuid::Uuid;

    #[test]
    fn encrypts_authenticates_and_binds_secret_to_identity_and_key_version() {
        let protector = Aes256GcmSecretProtector::new("kms-key-v1".into(), &[7; 32]).unwrap();
        let identity = Uuid::now_v7();
        let encrypted = protector.seal(identity, b"totp-secret-material").unwrap();
        assert_ne!(encrypted.ciphertext, b"totp-secret-material");
        assert_eq!(
            protector.open(identity, &encrypted).unwrap(),
            b"totp-secret-material"
        );
        assert_eq!(
            protector.open(Uuid::now_v7(), &encrypted).unwrap_err(),
            SecretProtectionError::AuthenticationFailed
        );
        let wrong_key = Aes256GcmSecretProtector::new("kms-key-v2".into(), &[8; 32]).unwrap();
        assert_eq!(
            wrong_key.open(identity, &encrypted).unwrap_err(),
            SecretProtectionError::InvalidCiphertext
        );
    }
}
