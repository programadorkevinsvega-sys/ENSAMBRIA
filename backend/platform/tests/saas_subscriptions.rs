use ensambria_platform::saas_subscriptions::{ChangeSubscription, CreateSubscription, SaaSSubscriptions, SubscriptionAction, SubscriptionError};
use sqlx::PgPool;
use uuid::Uuid;

const PRODUCT_ID: &str = "018f0f2a-7b3c-7abc-8def-0123456789ab";
const OFFER_ID: &str = "018f0f2a-7b3c-7abc-8def-0123456789b1";

#[tokio::test]
async fn subscription_lifecycle_is_idempotent_versioned_and_tenant_scoped() {
    let url = std::env::var("PLATFORM_TEST_DATABASE_URL").expect("CI configures PLATFORM_TEST_DATABASE_URL");
    let pool = PgPool::connect(&url).await.unwrap();
    let service = SaaSSubscriptions::new(pool.clone());
    let tenant = Uuid::now_v7();
    let actor = Uuid::now_v7();
    let create = CreateSubscription { tenant_id: tenant, product_id: Uuid::parse_str(PRODUCT_ID).unwrap(),
        offer_id: Uuid::parse_str(OFFER_ID).unwrap(), offer_version: 1, actor_id: actor,
        idempotency_key: format!("create-{}", Uuid::now_v7()), ends_at_epoch: None };
    let created = service.create(create.clone()).await.expect("explicit contract creation");
    assert_eq!(created.status, "active");
    assert_eq!(created.version, 1);
    assert_eq!(created.entitlements["capabilities"][0], "identity.auth");
    assert_eq!(service.create(create).await.unwrap().subscription_id, created.subscription_id);
    let suspend = |expected_version, action, label: &str| ChangeSubscription {
        tenant_id: tenant, subscription_id: created.subscription_id, actor_id: actor,
        idempotency_key: format!("{label}-{}", Uuid::now_v7()), expected_version, action,
        new_ends_at_epoch: None, reason_code: None
    };
    assert_eq!(service.change(suspend(1, SubscriptionAction::Suspend, "suspend")).await.unwrap().status, "suspended");
    assert_eq!(service.change(suspend(2, SubscriptionAction::Reactivate, "reactivate")).await.unwrap().status, "active");
    let renewed = service.change(ChangeSubscription { new_ends_at_epoch: Some(4_102_444_800),
        ..suspend(3, SubscriptionAction::Renew, "renew") }).await.unwrap();
    assert_eq!(renewed.version, 4);
    let cancelled = service.change(suspend(4, SubscriptionAction::Cancel, "cancel")).await.unwrap();
    assert_eq!(cancelled.status, "cancelled");
    assert_eq!(service.get(tenant, created.subscription_id).await.unwrap().unwrap().version, 5);
    assert!(service.get(Uuid::now_v7(), created.subscription_id).await.unwrap().is_none());

    let mut tx = pool.begin().await.unwrap();
    sqlx::query("SELECT set_config('app.tenant_id',$1,true)").bind(tenant.to_string()).execute(&mut *tx).await.unwrap();
    let (audit, events, commands): (i64,i64,i64) = sqlx::query_as(
        "SELECT (SELECT count(*) FROM platform.saas_subscription_audit WHERE subscription_id=$1), (SELECT count(*) FROM platform.saas_subscription_outbox WHERE subscription_id=$1), (SELECT count(*) FROM platform.saas_subscription_commands WHERE tenant_id=$2)"
    ).bind(created.subscription_id).bind(tenant).fetch_one(&mut *tx).await.unwrap();
    assert_eq!(audit, 5);
    assert_eq!(events, 5);
    assert_eq!(commands, 5);
    tx.rollback().await.unwrap();
    pool.close().await;
}

#[tokio::test]
async fn stale_versions_and_reused_idempotency_keys_are_rejected() {
    let url = std::env::var("PLATFORM_TEST_DATABASE_URL").expect("CI configures PLATFORM_TEST_DATABASE_URL");
    let pool = PgPool::connect(&url).await.unwrap();
    let service = SaaSSubscriptions::new(pool.clone());
    let tenant = Uuid::now_v7();
    let create = CreateSubscription { tenant_id: tenant, product_id: Uuid::parse_str(PRODUCT_ID).unwrap(),
        offer_id: Uuid::parse_str(OFFER_ID).unwrap(), offer_version: 1, actor_id: Uuid::now_v7(),
        idempotency_key: format!("create-{}", Uuid::now_v7()), ends_at_epoch: None };
    let created = service.create(create.clone()).await.unwrap();
    let stale = ChangeSubscription { tenant_id: tenant, subscription_id: created.subscription_id,
        actor_id: create.actor_id, idempotency_key: format!("suspend-{}", Uuid::now_v7()),
        expected_version: 99, action: SubscriptionAction::Suspend, new_ends_at_epoch: None, reason_code: None };
    assert_eq!(service.change(stale).await.unwrap_err(), SubscriptionError::VersionConflict);
    let mut conflict = create.clone();
    conflict.offer_version = 2;
    assert_eq!(service.create(conflict).await.unwrap_err(), SubscriptionError::IdempotencyConflict);
    let cleanup = ChangeSubscription { tenant_id: tenant, subscription_id: created.subscription_id,
        actor_id: create.actor_id, idempotency_key: format!("cancel-{}", Uuid::now_v7()),
        expected_version: 1, action: SubscriptionAction::Cancel, new_ends_at_epoch: None, reason_code: None };
    service.change(cleanup).await.unwrap();
    pool.close().await;
}
