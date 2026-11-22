use std::time::Duration;

use ensambria_shared_cache::TtlCache;
use uuid::Uuid;

#[tokio::test]
async fn redis_cache_round_trips_expires_and_deletes_values() {
    let redis_url = std::env::var("ENSAMBRIA_TEST_REDIS_URL")
        .expect("ENSAMBRIA_TEST_REDIS_URL must point to an isolated Redis test service");
    let cache = TtlCache::connect(&redis_url, "ensambria_test")
        .await
        .expect("Redis test service must be reachable");
    let key = format!("integration:{}", Uuid::now_v7());
    let value = b"versioned-cache-entry";

    cache
        .put(&key, value, Duration::from_secs(2))
        .await
        .unwrap();
    assert_eq!(
        cache.get(&key).await.unwrap().as_deref(),
        Some(value.as_slice())
    );

    let isolated_cache = TtlCache::connect(&redis_url, "ensambria_other_test")
        .await
        .expect("Redis test service must be reachable");
    assert_eq!(isolated_cache.get(&key).await.unwrap(), None);

    cache.delete(&key).await.unwrap();
    assert_eq!(cache.get(&key).await.unwrap(), None);

    cache
        .put(&key, value, Duration::from_secs(1))
        .await
        .unwrap();
    tokio::time::sleep(Duration::from_millis(1_100)).await;
    assert_eq!(cache.get(&key).await.unwrap(), None);
}
