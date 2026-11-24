use std::fmt;

const MAX_EMAIL_LENGTH: usize = 320;

pub fn normalize_email(value: &str) -> Result<String, EmailAddressError> {
    let normalized = value.trim().to_lowercase();
    if normalized.len() > MAX_EMAIL_LENGTH
        || normalized.chars().any(char::is_whitespace)
        || normalized.matches('@').count() != 1
    {
        return Err(EmailAddressError::Invalid);
    }
    let (local, domain) = normalized
        .split_once('@')
        .ok_or(EmailAddressError::Invalid)?;
    if local.is_empty() || domain.is_empty() || local.len() > 64 || domain.len() > 255 {
        return Err(EmailAddressError::Invalid);
    }
    Ok(normalized)
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum EmailAddressError {
    Invalid,
}

impl fmt::Display for EmailAddressError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("email address is invalid")
    }
}

impl std::error::Error for EmailAddressError {}

#[cfg(test)]
mod tests {
    use super::normalize_email;

    #[test]
    fn normalizes_case_and_whitespace() {
        assert_eq!(
            normalize_email("  Person@Example.Test ").unwrap(),
            "person@example.test"
        );
    }

    #[test]
    fn rejects_malformed_addresses() {
        assert!(normalize_email("missing-at-sign").is_err());
        assert!(normalize_email("two@@example.test").is_err());
    }
}
