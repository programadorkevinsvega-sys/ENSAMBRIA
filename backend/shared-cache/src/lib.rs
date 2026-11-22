use std::{
    sync::{
        Arc,
        atomic::{AtomicUsize, Ordering},
    },
    time::Duration,
};

use redis::aio::{ConnectionManager, ConnectionManagerConfig};
use redis::{RedisConnectionInfo, TlsMode};
use redis::sentinel::{SentinelClient, SentinelNodeConnectionInfo, SentinelServerType};
use redis::{Client, RedisError};
use url::Url;

const CONNECTION_TIMEOUT: Duration = Duration::from_secs(1);
const RESPONSE_TIMEOUT: Duration = Duration::from_millis(500);
const RECONNECT_ATTEMPTS: usize = 5;
const DEFAULT_CONNECTIONS_PER_PROCESS: usize = 4;
const MAX_CONNECTIONS_PER_PROCESS: usize = 64;

#[derive(Clone)]
pub struct TtlCache {
    connections: Arc<Vec<ConnectionManager>>,
    next_connection: Arc<AtomicUsize>,
    namespace: String,
    cluster: Option<Arc<Vec<tokio::sync::Mutex<redis::cluster_async::ClusterConnection>>>>,
    sentinel: Option<Arc<Vec<tokio::sync::Mutex<SentinelClient>>>>,
}

impl TtlCache {
    /// Builds the shared cache from deployment environment variables.
    /// Production requires TLS (rediss://) and Redis ACL credentials.
    pub async fn connect_from_env(namespace: &str) -> Result<Self, CacheError> {
        let production = std::env::var("ENSAMBRIA_ENV")
            .is_ok_and(|value| value.eq_ignore_ascii_case("production"));
        let mode = std::env::var("PLATFORM_REDIS_MODE").unwrap_or_else(|_| "direct".to_owned());
        let pool_size = std::env::var("PLATFORM_REDIS_POOL_SIZE")
            .ok()
            .map(|value| value.parse::<usize>().map_err(|_| CacheError::InvalidConfiguration))
            .transpose()?
            .unwrap_or(DEFAULT_CONNECTIONS_PER_PROCESS);
        match mode.as_str() {
            "direct" => {
                let url = std::env::var("PLATFORM_REDIS_URL").map_err(|_| CacheError::InvalidConfiguration)?;
                validate_redis_url(&url, production)?;
                Self::connect_with_pool_size(&url, namespace, pool_size).await
            }
            "cluster" => Self::connect_cluster(env_list("PLATFORM_REDIS_CLUSTER_URLS")?, namespace, pool_size, production).await,
            "sentinel" => Self::connect_sentinel_from_env(namespace, pool_size, production).await,
            _ => Err(CacheError::InvalidConfiguration),
        }
    }

    pub async fn connect(redis_url: &str, namespace: &str) -> Result<Self, CacheError> {
        Self::connect_with_pool_size(
            redis_url,
            namespace,
            DEFAULT_CONNECTIONS_PER_PROCESS,
        )
        .await
    }

    pub async fn connect_with_pool_size(
        redis_url: &str,
        namespace: &str,
        pool_size: usize,
    ) -> Result<Self, CacheError> {
        validate_segment(namespace)?;
        if redis_url.trim().is_empty()
            || !(1..=MAX_CONNECTIONS_PER_PROCESS).contains(&pool_size)
        {
            return Err(CacheError::InvalidConfiguration);
        }

        let _ = rustls::crypto::ring::default_provider().install_default();
        let client = Client::open(redis_url).map_err(|_| CacheError::InvalidConfiguration)?;
        let configuration = ConnectionManagerConfig::new()
            .set_exponent_base(2)
            .set_factor(100)
            .set_max_delay(2_000)
            .set_number_of_retries(RECONNECT_ATTEMPTS)
            .set_connection_timeout(CONNECTION_TIMEOUT)
            .set_response_timeout(RESPONSE_TIMEOUT);
        let mut connections = Vec::with_capacity(pool_size);
        for _ in 0..pool_size {
            let connection = ConnectionManager::new_with_config(client.clone(), configuration.clone())
                .await
                .map_err(|_| CacheError::Unavailable)?;
            connections.push(connection);
        }

        Ok(Self {
            connections: Arc::new(connections),
            next_connection: Arc::new(AtomicUsize::new(0)),
            namespace: namespace.to_owned(),
            cluster: None,
            sentinel: None,
        })
    }


    async fn connect_cluster(nodes: Vec<String>, namespace: &str, pool_size: usize, production: bool) -> Result<Self, CacheError> {
        validate_segment(namespace)?;
        if nodes.is_empty() || nodes.iter().any(|node| validate_redis_url(node, production).is_err()) {
            return Err(CacheError::InvalidConfiguration);
        }
        let _ = rustls::crypto::ring::default_provider().install_default();
        let client = redis::cluster::ClusterClient::new(nodes).map_err(|_| CacheError::InvalidConfiguration)?;
        let mut connections = Vec::with_capacity(pool_size);
        for _ in 0..pool_size {
            connections.push(tokio::sync::Mutex::new(client.get_async_connection().await.map_err(map_redis_error)?));
        }
        Ok(Self {
            connections: Arc::new(Vec::new()),
            next_connection: Arc::new(AtomicUsize::new(0)),
            namespace: namespace.to_owned(),
            cluster: Some(Arc::new(connections)),
            sentinel: None,
        })
    }

    async fn connect_sentinel_from_env(namespace: &str, pool_size: usize, production: bool) -> Result<Self, CacheError> {
        validate_segment(namespace)?;
        let nodes = env_list("PLATFORM_REDIS_SENTINEL_URLS")?;
        let master = std::env::var("PLATFORM_REDIS_SENTINEL_MASTER_NAME").map_err(|_| CacheError::InvalidConfiguration)?;
        if nodes.iter().any(|node| validate_redis_url(node, production).is_err()) || master.trim().is_empty() {
            return Err(CacheError::InvalidConfiguration);
        }
        let username = std::env::var("PLATFORM_REDIS_USERNAME").map_err(|_| CacheError::InvalidConfiguration)?;
        let password = std::env::var("PLATFORM_REDIS_PASSWORD").map_err(|_| CacheError::InvalidConfiguration)?;
        if username.is_empty() || password.is_empty() {
            return Err(CacheError::InvalidConfiguration);
        }
        let redis_info = RedisConnectionInfo::default().set_username(username).set_password(password);
        let node_info = SentinelNodeConnectionInfo::default()
            .set_tls_mode(if production { TlsMode::Secure)
            .set_redis_connection_info(redis_info);
        let mut clients = Vec::with_capacity(pool_size);
        for _ in 0..pool_size {
            let mut client = SentinelClient::build(nodes.clone(), master.clone(), Some(node_info.clone()), SentinelServerType::Master)
                .map_err(map_redis_error)?;
            let mut probe = client.get_async_connection().await.map_err(map_redis_error)?;
            redis::cmd("PING").query_async::<String>(&mut probe).await.map_err(map_redis_error)?;
            clients.push(tokio::sync::Mutex::new(client));
        }
        Ok(Self {
            connections: Arc::new(Vec::new()),
            next_connection: Arc::new(AtomicUsize::new(0)),
            namespace: namespace.to_owned(),
            cluster: None,
            sentinel: Some(Arc::new(clients)),
        })
    }

    fn connection(&self) -> ConnectionManager {
        let index = self.next_connection.fetch_add(1, Ordering::Relaxed)
            % self.connections.len();
        self.connections[index].clone()
    }

    pub async fn get(&self, key: &str) -> Result<Option<Vec<u8>>, CacheError> {
        let key = self.qualified_key(key)?;
        if let Some(cluster) = &self.cluster {
            let index = self.next_connection.fetch_add(1, Ordering::Relaxed) % cluster.len();
            let mut connection = cluster[index].lock().await;
            return redis::cmd("GET").arg(key).query_async(&mut *connection).await.map_err(map_redis_error);
        }
        if let Some(sentinel) = &self.sentinel {
            let index = self.next_connection.fetch_add(1, Ordering::Relaxed) % sentinel.len();
            let mut client = sentinel[index].lock().await;
            let mut connection = client.get_async_connection().await.map_err(map_redis_error)?;
            return redis::cmd("GET").arg(key).query_async(&mut connection).await.map_err(map_redis_error);
        }
        let mut connection = self.connection();
        redis::cmd("GET").arg(key).query_async(&mut connection).await.map_err(map_redis_error)
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

        if let Some(cluster) = &self.cluster {
            let index = self.next_connection.fetch_add(1, Ordering::Relaxed) % cluster.len();
            let mut connection = cluster[index].lock().await;
            return redis::cmd("SET").arg(key).arg(value).arg("EX").arg(ttl_seconds).query_async::<()>(&mut *connection).await.map_err(map_redis_error);
        }
        if let Some(sentinel) = &self.sentinel {
            let index = self.next_connection.fetch_add(1, Ordering::Relaxed) % sentinel.len();
            let mut client = sentinel[index].lock().await;
            let mut connection = client.get_async_connection().await.map_err(map_redis_error)?;
            return redis::cmd("SET").arg(key).arg(value).arg("EX").arg(ttl_seconds).query_async::<()>(&mut connection).await.map_err(map_redis_error);
        }
        let mut connection = self.connection();
        redis::cmd("SET").arg(key).arg(value).arg("EX").arg(ttl_seconds).query_async::<()>(&mut connection).await.map_err(map_redis_error)
    }

    pub async fn delete(&self, key: &str) -> Result<(), CacheError> {
        let key = self.qualified_key(key)?;
        if let Some(cluster) = &self.cluster {
            let index = self.next_connection.fetch_add(1, Ordering::Relaxed) % cluster.len();
            let mut connection = cluster[index].lock().await;
            return redis::cmd("DEL").arg(key).query_async::<()>(&mut *connection).await.map_err(map_redis_error);
        }
        if let Some(sentinel) = &self.sentinel {
            let index = self.next_connection.fetch_add(1, Ordering::Relaxed) % sentinel.len();
            let mut client = sentinel[index].lock().await;
            let mut connection = client.get_async_connection().await.map_err(map_redis_error)?;
            return redis::cmd("DEL").arg(key).query_async::<()>(&mut connection).await.map_err(map_redis_error);
        }
        let mut connection = self.connection();
        redis::cmd("DEL").arg(key).query_async::<()>(&mut connection).await.map_err(map_redis_error)
    }

    /// Atomic cross-replica rate limit. Redis is the fast path; callers may fall back to PostgreSQL on error.
    pub async fn consume_rate_limit(
        &self,
        key: &str,
        window_seconds: u64,
        limit: u64,
        block_seconds: u64,
    ) -> Result<bool, CacheError> {
        if window_seconds == 0 || limit == 0 || block_seconds == 0 {
            return Err(CacheError::InvalidTtl);
        }
        let key = self.qualified_key(key)?;
        let script = "local count = redis.call('INCR', KEYS[1]); if count == 1 then redis.call('EXPIRE', KEYS[1], ARGV[1]); end; if count > tonumber(ARGV[2]) then redis.call('EXPIRE', KEYS[1], ARGV[3]); return 0; end; return 1";
        let result = if let Some(cluster) = &self.cluster {
            let index = self.next_connection.fetch_add(1, Ordering::Relaxed) % cluster.len();
            let mut connection = cluster[index].lock().await;
            redis::cmd("EVAL").arg(script).arg(1).arg(key).arg(window_seconds).arg(limit).arg(block_seconds).query_async::<i64>(&mut *connection).await
        } else if let Some(sentinel) = &self.sentinel {
            let mut client = sentinel.lock().await;
            let mut connection = client.get_async_connection().await.map_err(map_redis_error)?;
            redis::cmd("EVAL").arg(script).arg(1).arg(key).arg(window_seconds).arg(limit).arg(block_seconds).query_async::<i64>(&mut connection).await
        } else {
            let mut connection = self.connection();
            redis::cmd("EVAL").arg(script).arg(1).arg(key).arg(window_seconds).arg(limit).arg(block_seconds).query_async::<i64>(&mut connection).await
        };
        result.map(|allowed| allowed == 1).map_err(map_redis_error)
    }

    fn qualified_key(&self, key: &str) -> Result<String, CacheError> {
        validate_segment(key)?;
        Ok(format!("{}:{}", self.namespace, key))
    }
}

fn env_list(name: &'static str) -> Result<Vec<String>, CacheError> {
    let value = std::env::var(name).map_err(|_| CacheError::InvalidConfiguration)?;
    let values = value.split(';').map(str::trim).filter(|item| !item.is_empty()).map(str::to_owned).collect::<Vec<_>>();
    if values.is_empty() { return Err(CacheError::InvalidConfiguration); }
    Ok(values)
}

fn validate_redis_url(redis_url: &str, production: bool) -> Result<(), CacheError> {
    let parsed = Url::parse(redis_url).map_err(|_| CacheError::InvalidConfiguration)?;
    if production
        && (parsed.scheme() != "rediss"
            || parsed.username().is_empty()
            || parsed.password().is_none_or(str::is_empty))
    {
        return Err(CacheError::InvalidConfiguration);
    }
    if !matches!(parsed.scheme(), "redis" | "rediss")
        || parsed.host_str().is_none()
        || parsed.query().is_some()
        || parsed.fragment().is_some()
    {
        return Err(CacheError::InvalidConfiguration);
    }
    Ok(())
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
    use url::Url;

    #[test]
    fn production_redis_configuration_requires_tls_and_acl() {
        let insecure = Url::parse("redis://cache.internal:6379").unwrap();
        let tls_without_acl = Url::parse("rediss://cache.internal:6379").unwrap();
        let secured = Url::parse("rediss://ensambria:secret@cache.internal:6379").unwrap();
        let secure = |url: &Url| url.scheme() == "rediss"
            && !url.username().is_empty()
            && url.password().is_some_and(|password| !password.is_empty());
        assert!(!secure(&insecure));
        assert!(!secure(&tls_without_acl));
        assert!(secure(&secured));
    }
    #[test]
    fn cache_keys_allow_only_bounded_safe_segments() {
        assert_eq!(validate_segment("tenant:product:v1"), Ok(()));
        assert_eq!(validate_segment(""), Err(CacheError::InvalidKey));
        assert_eq!(validate_segment("tenant/{secret}"), Err(CacheError::InvalidKey));
        assert_eq!(validate_segment(&"x".repeat(241)), Err(CacheError::InvalidKey));
    }
}
