use std::fmt;

use ring::{
    hmac,
    rand::{SecureRandom, SystemRandom},
};

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
            if ring::constant_time::verify_slices_are_equal(code.as_bytes(), expected.as_bytes())
                .is_ok()
            {
                return Some(step);
            }
        }
        None
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
        }
    }
    if bits > 0 {
        output.push(char::from(ALPHABET[((buffer << (5 - bits)) & 0x1f) as usize]));
    }
    output
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TotpError {
    RandomGeneration,
}

impl fmt::Display for TotpError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("TOTP secret generation failed")
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
        assert!(!format!("{secret:?}").contains("1234567890"));
    }
}
