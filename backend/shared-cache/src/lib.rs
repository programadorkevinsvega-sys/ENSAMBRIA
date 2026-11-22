use std::time::Duration;

use redis::aio::{ConnectionManager, ConnectionManagerConfig};
use redis::{Client, RedisError};

const MIN_RECONNECT_DELAY: Duration = Duration::from_millis(100);
const MAX_RECONNECT_DELAY: Duration = Duration::from_secs(2);
const CONNECTION_TIMEOUT: Duration = Duration::from_secs(1);
const RESPONSE_TIMEOUT: Duration = Duration::from_millis(500);
const RECONNECT_ATTEMPTS: usize = 5;

#[derive(Clone)]
pub struct TtlCache {
    connection: ConnectionManager,
    namespace: String,
}

impl TtlCache {
    pub async fn connect(redis_url: &str, namespace: &str) -> Result<Self, CacheError> {
        validate_segment(namespace)?;
        if redis_url.trim().is_empty() {
            return Err(CacheError::InvalidConfiguration);
        }

        let client = Client::open(redis_url).map_err(|_| CacheError::InvalidConfiguration)?;
        let configuration = ConnectionManagerConfig::new()
            .set_min_delay(MIN_RECONNECT_DELAY)
            .set_max_delay(MAX_RECONNECT_DELAY)
            .set_number_of_retries(RECONNECT_ATTEMPTS)
            .set_connection_timeout(Some(CONNECTION_TIMEOUT))
            .set_response_timeout(Some(RESPONSE_TIMEOUT));
        let connection = ConnectionManager::new_with_config(client, configuration)
            .await
            .map_err(|_| CacheError::Unavailable)?;

        Ok(Self {
            connection,
            namespace: namespace.to_owned(),
        })
    }

    pub async fn get(&self, key: &str) -> Result<Option<Vec<u8>>, CacheError> {
        let key = self.qualified_key(key)?;
        let mut connection = self.connection.clone();
        redis::cmd("GET")
            .arg(key)
            .query_async(&mut connection)
            .await
            .map_err(map_redis_error)
    }

    pub async fn put(
        &self,
        key: &str,
        value: &[u8],
        ttl: Duration,
    ) -> Result<(), CacheError> {
        let key = self.qualified_key(key)?;
        let ttl_seconds = ttl.as_secs();
        if ttl_seconds == 0 {
            return Err(CacheError::InvalidTtl);
        }

        let mut connection = self.connection.clone();
        redis::cmd("SET")
            .arg(key)
            .arg(value)
            .arg("EX")
            .arg(ttl_seconds)
            .query_async::<()>(&mut connection)
            .await
            .map_err(map_redis_error)
    }

    pub async fn delete(&self, key: &str) -> Result<(), CacheError> {
        let key = self.qualified_key(key)?;
        let mut connection = self.connection.clone();
        redis::cmd("DEL")
            .arg(key)
            .query_async::<()>(&mut connection)
            .await
            .map_err(map_redis_error)
    }

    fn qualified_key(&self, key: &str) -> Result<String, CacheError> {
        validate_segment(key)?;
        Ok(format!("{}:{}", self.namespace, key))
    }
}

fn validate_segment(value: &str) -> Result<(), CacheError> {
    if value.is_empty()
        || value.len() > 240
        || !value.bytes().all(|byte| {
            byte.is_ascii_alphanumeric() || matches!(byte, b':' | b'_' | b'-' | b'.')
        })
    {
        return Err(CacheError::InvalidKey);
    }
    Ok(())
}

fn map_redis_error(_: RedisError) -> CacheError {
    CacheError::Unavailable
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CacheError {
    InvalidConfiguration,
    InvalidKey,
    InvalidTtl,
    Unavailable,
}

impl std::fmt::Display for CacheError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(match self {
            Self::InvalidConfiguration => "Redis cache configuration is invalid",
            Self::InvalidKey => "Redis cache key is invalid",
            Self::InvalidTtl => "Redis cache TTL must be at least one second",
            Self::Unavailable => "Redis cache is unavailable",
        })
    }
}

impl std::error::Error for CacheError {}

#[cfg(test)]
mod tests {
    use super::{CacheError, validate_segment};

    #[test]
    fn cache_keys_allow_only_bounded_safe_segments() {
        assert_eq!(validate_segment("tenant:product:v1"), Ok(()));
        assert_eq!(validate_segment(""), Err(CacheError::InvalidKey));
        assert_eq!(validate_segment("tenant/{secret}"), Err(CacheError::InvalidKey));
        assert_eq!(validate_segment(&"x".repeat(241)), Err(CacheError::InvalidKey));
    }
}
