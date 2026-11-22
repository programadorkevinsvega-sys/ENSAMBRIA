use std::fmt;

use ring::{
    hmac,
    rand::{SecureRandom, SystemRandom},
};
use subtle::ConstantTimeEq;
use zeroize::Zeroize;

const SECRET_LENGTH: usize = 20;
const STEP_SECONDS: u64 = 30;
const CODE_DIGITS: u32 = 6;
const CODE_MODULUS: u32 = 10_u32.pow(CODE_DIGITS);

pub struct TotpSecret([u8; SECRET_LENGTH]);

impl TotpSecret {
    pub fn generate() -> Result<Self, TotpError> {
        let mut secret = [0; SECRET_LENGTH];
        SystemRandom::new()
            .fill(&mut secret)
            .map_err(|_| TotpError::RandomGeneration)?;
        Ok(Self(secret))
    }

    pub fn from_bytes(secret: [u8; SECRET_LENGTH]) -> Self {
        Self(secret)
    }

    pub fn from_base32(encoded: &str) -> Result<Self, TotpError> {
        if encoded.len() != 32 {
            return Err(TotpError::InvalidEncoding);
        }
        let mut decoded = Vec::with_capacity(SECRET_LENGTH);
        let mut buffer = 0_u32;
        let mut bits = 0_u8;
        for byte in encoded.bytes() {
            let value = match byte {
                b'A'..=b'Z' => u32::from(byte - b'A'),
                b'2'..=b'7' => u32::from(byte - b'2' + 26),
                _ => return Err(TotpError::InvalidEncoding),
            };
            buffer = (buffer << 5) | value;
            bits += 5;
            if bits >= 8 {
                bits -= 8;
                decoded.push(((buffer >> bits) & 0xff) as u8);
                buffer &= (1_u32 << bits) - 1;
            }
        }
        if bits != 0 && buffer != 0 {
            return Err(TotpError::InvalidEncoding);
        }
        let secret: [u8; SECRET_LENGTH] =
            decoded.try_into().map_err(|_| TotpError::InvalidEncoding)?;
        Ok(Self(secret))
    }

    pub fn as_bytes(&self) -> &[u8; SECRET_LENGTH] {
        &self.0
    }

    pub fn base32(&self) -> String {
        base32_encode(&self.0)
    }

    pub fn code_at_step(&self, step: u64) -> String {
        let key = hmac::Key::new(hmac::HMAC_SHA1_FOR_LEGACY_USE_ONLY, &self.0);
        let digest = hmac::sign(&key, &step.to_be_bytes());
        let bytes = digest.as_ref();
        let offset = usize::from(bytes[bytes.len() - 1] & 0x0f);
        let binary = (u32::from(bytes[offset] & 0x7f) << 24)
            | (u32::from(bytes[offset + 1]) << 16)
            | (u32::from(bytes[offset + 2]) << 8)
            | u32::from(bytes[offset + 3]);
        format!("{:06}", binary % CODE_MODULUS)
    }

    pub fn verify_at(&self, code: &str, unix_seconds: u64) -> Option<u64> {
        if code.len() != CODE_DIGITS as usize || !code.bytes().all(|b| b.is_ascii_digit()) {
            return None;
        }
        let current = unix_seconds / STEP_SECONDS;
        let candidates = [
            current.checked_sub(1),
            Some(current),
            current.checked_add(1),
        ];
        for step in candidates.into_iter().flatten() {
            let expected = self.code_at_step(step);
            if bool::from(code.as_bytes().ct_eq(expected.as_bytes())) {
                return Some(step);
            }
        }
        None
    }
}

impl Drop for TotpSecret {
    fn drop(&mut self) {
        self.0.zeroize();
    }
}

impl fmt::Debug for TotpSecret {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("TotpSecret([REDACTED])")
    }
}

fn base32_encode(bytes: &[u8]) -> String {
    const ALPHABET: &[u8; 32] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ234567";
    let mut output = String::with_capacity((bytes.len() * 8).div_ceil(5));
    let mut buffer = 0_u16;
    let mut bits = 0_u8;
    for byte in bytes {
        buffer = (buffer << 8) | u16::from(*byte);
        bits += 8;
        while bits >= 5 {
            bits -= 5;
            output.push(char::from(ALPHABET[((buffer >> bits) & 0x1f) as usize]));
            buffer &= (1_u16 << bits) - 1;
        }
    }
    if bits > 0 {
        output.push(char::from(
            ALPHABET[((buffer << (5 - bits)) & 0x1f) as usize],
        ));
    }
    output
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TotpError {
    RandomGeneration,
    InvalidEncoding,
}

impl fmt::Display for TotpError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(match self {
            Self::RandomGeneration => "TOTP secret generation failed",
            Self::InvalidEncoding => "TOTP secret encoding is invalid",
        })
    }
}

impl std::error::Error for TotpError {}

#[cfg(test)]
mod tests {
    use super::TotpSecret;

    #[test]
    fn matches_rfc_6238_sha1_vector_and_accepts_adjacent_clock_steps() {
        let secret = TotpSecret::from_bytes(*b"12345678901234567890");
        assert_eq!(secret.code_at_step(1), "287082");
        assert_eq!(secret.verify_at("287082", 59), Some(1));
        assert!(secret.verify_at("12345", 59).is_none());
        assert!(secret.verify_at("abcdef", 59).is_none());
    }

    #[test]
    fn secret_is_encoded_as_unpadded_base32_and_redacted() {
        let secret = TotpSecret::from_bytes(*b"12345678901234567890");
        assert_eq!(secret.base32(), "GEZDGNBVGY3TQOJQGEZDGNBVGY3TQOJQ");
        let restored = TotpSecret::from_base32(&secret.base32()).unwrap();
        assert_eq!(restored.as_bytes(), secret.as_bytes());
        assert!(TotpSecret::from_base32("not-base32").is_err());
        assert!(!format!("{secret:?}").contains("1234567890"));
    }
}
