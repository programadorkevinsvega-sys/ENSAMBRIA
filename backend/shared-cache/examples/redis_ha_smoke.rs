use std::time::Duration;

use ensambria_shared_cache::TtlCache;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mode = std::env::var("PLATFORM_REDIS_MODE")?;
    let cache = TtlCache::connect_from_env("ensambria-ha-ci").await?;
    let key = format!("{}-{}", mode, std::process::id());
    let value = format!("connected:{mode}").into_bytes();

    cache.put(&key, &value, Duration::from_secs(30)).await?;
    let actual = cache.get(&key).await?.ok_or("cache key disappeared")?;
    if actual != value {
        return Err("Redis round-trip returned a different value".into());
    }

    let rate_key = format!("{key}-rate");
    assert!(cache.consume_rate_limit(&rate_key, 30, 2, 60).await?);
    assert!(cache.consume_rate_limit(&rate_key, 30, 2, 60).await?);
    assert!(!cache.consume_rate_limit(&rate_key, 30, 2, 60).await?);
    cache.delete(&key).await?;
    println!("Redis {mode}: read/write/delete and atomic rate-limit passed");
    Ok(())
}
