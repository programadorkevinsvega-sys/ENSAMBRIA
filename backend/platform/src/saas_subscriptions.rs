use chrono::{DateTime, Utc};
use ring::digest::{digest, SHA256};
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use sqlx::{PgPool, Row};
use uuid::Uuid;

#[derive(Clone)]
pub struct SaaSSubscriptions { pool: PgPool }

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SubscriptionAction { Suspend, Reactivate, Renew, Cancel, Expire }

impl SubscriptionAction {
    fn as_str(self) -> &'static str {
        match self { Self::Suspend => "suspended", Self::Reactivate => "reactivated",
            Self::Renew => "renewed", Self::Cancel => "cancelled", Self::Expire => "expired" }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubscriptionSnapshot {
    pub subscription_id: Uuid, pub tenant_id: Uuid, pub product_id: Uuid,
    pub offer_id: Uuid, pub offer_version: i32, pub status: String,
    pub starts_at: DateTime<Utc>, pub ends_at: Option<DateTime<Utc>>, pub version: i64,
}

#[derive(Debug, Clone)]
pub struct CreateSubscription {
    pub tenant_id: Uuid, pub product_id: Uuid, pub offer_id: Uuid, pub offer_version: i32,
    pub actor_id: Uuid, pub idempotency_key: String, pub ends_at: Option<DateTime<Utc>>,
}
#[derive(Debug, Clone)]
pub struct ChangeSubscription {
    pub tenant_id: Uuid, pub subscription_id: Uuid, pub actor_id: Uuid,
    pub idempotency_key: String, pub expected_version: i64, pub action: SubscriptionAction,
    pub new_ends_at: Option<DateTime<Utc>>, pub reason_code: Option<String>,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum SubscriptionError {
    InvalidInput, OfferUnavailable, AlreadyExists, NotFound, InvalidTransition,
    VersionConflict, IdempotencyConflict, Storage,
}
impl std::fmt::Display for SubscriptionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Self::InvalidInput => "subscription command is invalid",
            Self::OfferUnavailable => "published commercial offer version is unavailable",
            Self::AlreadyExists => "tenant already has a live subscription for this product",
            Self::NotFound => "subscription was not found in the authorized tenant scope",
            Self::InvalidTransition => "subscription lifecycle transition is not allowed",
            Self::VersionConflict => "subscription version is stale",
            Self::IdempotencyConflict => "idempotency key was reused with a different command",
            Self::Storage => "PLATFORM subscription storage is unavailable",
        })
    }
}
impl std::error::Error for SubscriptionError {}

impl SaaSSubscriptions {
    pub fn new(pool: PgPool) -> Self { Self { pool } }

    /// The authenticated application adapter must derive actor/tenant from CORE claims,
    /// validate membership and permission, and never trust a tenant ID supplied by a browser.
    /// This method sets transaction-local RLS scope but is not itself an authentication layer.
    pub async fn create(&self, command: CreateSubscription) -> Result<SubscriptionSnapshot, SubscriptionError> {
        validate_key(&command.idempotency_key)?;
        if command.offer_version <= 0 || command.ends_at.is_some_and(|end| end <= Utc::now()) {
            return Err(SubscriptionError::InvalidInput);
        }
        let request = json!({"action":"create","tenant_id":command.tenant_id,"product_id":command.product_id,
            "offer_id":command.offer_id,"offer_version":command.offer_version,"ends_at":command.ends_at});
        let hash = request_hash(&request)?;
        let mut tx = self.pool.begin().await.map_err(|_| SubscriptionError::Storage)?;
        set_tenant(&mut tx, command.tenant_id).await?;
        if let Some(value) = replay_command(&mut tx, command.tenant_id, &command.idempotency_key, &hash).await? {
            tx.commit().await.map_err(|_| SubscriptionError::Storage)?;
            return decode_snapshot(value);
        }
        let offer = sqlx::query("SELECT o.product_id, o.status AS offer_status, v.status AS version_status FROM platform.commercial_offers o JOIN platform.commercial_offer_versions v ON v.offer_id=o.id WHERE o.id=$1 AND v.version=$2")
            .bind(command.offer_id).bind(command.offer_version).fetch_optional(&mut *tx).await
            .map_err(|_| SubscriptionError::Storage)?.ok_or(SubscriptionError::OfferUnavailable)?;
        if offer.get::<Uuid,_>("product_id") != command.product_id
            || offer.get::<String,_>("offer_status") != "published"
            || offer.get::<String,_>("version_status") != "published" {
            return Err(SubscriptionError::OfferUnavailable);
        }
        let id = Uuid::now_v7();
        let inserted = sqlx::query("INSERT INTO platform.saas_subscriptions (id,tenant_id,product_id,offer_id,offer_version,status,starts_at,ends_at,created_by) VALUES ($1,$2,$3,$4,$5,'active',clock_timestamp(),$6,$7) ON CONFLICT DO NOTHING RETURNING id")
            .bind(id).bind(command.tenant_id).bind(command.product_id).bind(command.offer_id)
            .bind(command.offer_version).bind(command.ends_at).bind(command.actor_id)
            .fetch_optional(&mut *tx).await.map_err(|_| SubscriptionError::Storage)?;
        if inserted.is_none() { return Err(SubscriptionError::AlreadyExists); }
        let snapshot = load_snapshot(&mut tx, command.tenant_id, id).await?;
        write_effects(&mut tx, &snapshot, command.actor_id, "created", None, None).await?;
        store_command(&mut tx, command.tenant_id, &command.idempotency_key, &hash, &snapshot).await?;
        tx.commit().await.map_err(|_| SubscriptionError::Storage)?;
        Ok(snapshot)
    }

    /// Renewal updates contract dates only; it never requests payment or creates an invoice.
    pub async fn change(&self, command: ChangeSubscription) -> Result<SubscriptionSnapshot, SubscriptionError> {
        validate_key(&command.idempotency_key)?;
        if command.expected_version <= 0 || command.reason_code.as_ref().is_some_and(|v| v.trim().is_empty() || v.len() > 80)
            || (matches!(command.action, SubscriptionAction::Renew) && !command.new_ends_at.is_some_and(|end| end > Utc::now()))
            || (!matches!(command.action, SubscriptionAction::Renew) && command.new_ends_at.is_some()) {
            return Err(SubscriptionError::InvalidInput);
        }
        let request = json!({"action":command.action,"subscription_id":command.subscription_id,
            "expected_version":command.expected_version,"new_ends_at":command.new_ends_at,"reason_code":command.reason_code});
        let hash = request_hash(&request)?;
        let mut tx = self.pool.begin().await.map_err(|_| SubscriptionError::Storage)?;
        set_tenant(&mut tx, command.tenant_id).await?;
        if let Some(value) = replay_command(&mut tx, command.tenant_id, &command.idempotency_key, &hash).await? {
            tx.commit().await.map_err(|_| SubscriptionError::Storage)?;
            return decode_snapshot(value);
        }
        let row = sqlx::query("SELECT status,ends_at,version FROM platform.saas_subscriptions WHERE id=$1 AND tenant_id=$2 FOR UPDATE")
            .bind(command.subscription_id).bind(command.tenant_id).fetch_optional(&mut *tx).await
            .map_err(|_| SubscriptionError::Storage)?.ok_or(SubscriptionError::NotFound)?;
        let current_version: i64 = row.get("version");
        if current_version != command.expected_version { return Err(SubscriptionError::VersionConflict); }
        let status: String = row.get("status");
        let next = match command.action {
            SubscriptionAction::Suspend if status == "active" => "suspended",
            SubscriptionAction::Reactivate if status == "suspended" => "active",
            SubscriptionAction::Renew if status == "active" || status == "suspended" => "active",
            SubscriptionAction::Cancel if status == "active" || status == "suspended" => "cancelled",
            SubscriptionAction::Expire if status == "active" && row.get::<Option<DateTime<Utc>>,_>("ends_at").is_some_and(|end| end <= Utc::now()) => "expired",
            _ => return Err(SubscriptionError::InvalidTransition),
        };
        let new_end = if matches!(command.action, SubscriptionAction::Renew) { command.new_ends_at } else { row.get("ends_at") };
        let suspended_at = if next == "suspended" { Some(Utc::now()) } else { None };
        let cancelled_at = if next == "cancelled" { Some(Utc::now()) } else { None };
        sqlx::query("UPDATE platform.saas_subscriptions SET status=$3,ends_at=$4,suspended_at=$5,cancelled_at=$6,version=$7,updated_at=clock_timestamp() WHERE id=$1 AND tenant_id=$2 AND version=$8")
            .bind(command.subscription_id).bind(command.tenant_id).bind(next).bind(new_end)
            .bind(suspended_at).bind(cancelled_at).bind(current_version + 1).bind(current_version)
            .execute(&mut *tx).await.map_err(|_| SubscriptionError::Storage)?;
        let snapshot = load_snapshot(&mut tx, command.tenant_id, command.subscription_id).await?;
        write_effects(&mut tx, &snapshot, command.actor_id, command.action.as_str(), Some(status), command.reason_code.as_deref()).await?;
        store_command(&mut tx, command.tenant_id, &command.idempotency_key, &hash, &snapshot).await?;
        tx.commit().await.map_err(|_| SubscriptionError::Storage)?;
        Ok(snapshot)
    }

    pub async fn get(&self, tenant_id: Uuid, subscription_id: Uuid) -> Result<Option<SubscriptionSnapshot>, SubscriptionError> {
        let mut tx = self.pool.begin().await.map_err(|_| SubscriptionError::Storage)?;
        set_tenant(&mut tx, tenant_id).await?;
        let row = sqlx::query("SELECT id,tenant_id,product_id,offer_id,offer_version,status,starts_at,ends_at,version FROM platform.saas_subscriptions WHERE id=$1 AND tenant_id=$2")
            .bind(subscription_id).bind(tenant_id).fetch_optional(&mut *tx).await.map_err(|_| SubscriptionError::Storage)?;
        tx.commit().await.map_err(|_| SubscriptionError::Storage)?;
        row.map(snapshot_from_row).transpose()
    }
}
fn validate_key(key: &str) -> Result<(), SubscriptionError> {
    if !(16..=128).contains(&key.len()) || !key.bytes().all(|b| b.is_ascii_graphic()) { return Err(SubscriptionError::InvalidInput); }
    Ok(())
}
fn request_hash(value: &Value) -> Result<Vec<u8>, SubscriptionError> {
    let bytes = serde_json::to_vec(value).map_err(|_| SubscriptionError::InvalidInput)?;
    Ok(digest(&SHA256, &bytes).as_ref().to_vec())
}
fn decode_snapshot(value: Value) -> Result<SubscriptionSnapshot, SubscriptionError> {
    serde_json::from_value(value).map_err(|_| SubscriptionError::Storage)
}
async fn set_tenant(tx: &mut sqlx::Transaction<'_, sqlx::Postgres>, tenant: Uuid) -> Result<(), SubscriptionError> {
    sqlx::query("SELECT set_config('app.tenant_id',$1,true)").bind(tenant.to_string())
        .execute(&mut **tx).await.map_err(|_| SubscriptionError::Storage)?;
    Ok(())
}
async fn replay_command(tx: &mut sqlx::Transaction<'_, sqlx::Postgres>, tenant: Uuid, key: &str, hash: &[u8]) -> Result<Option<Value>, SubscriptionError> {
    let inserted = sqlx::query("INSERT INTO platform.saas_subscription_commands (tenant_id,idempotency_key,request_hash) VALUES ($1,$2,$3) ON CONFLICT DO NOTHING RETURNING idempotency_key")
        .bind(tenant).bind(key).bind(hash).fetch_optional(&mut **tx).await.map_err(|_| SubscriptionError::Storage)?;
    if inserted.is_some() { return Ok(None); }
    let row = sqlx::query("SELECT request_hash,response FROM platform.saas_subscription_commands WHERE tenant_id=$1 AND idempotency_key=$2")
        .bind(tenant).bind(key).fetch_one(&mut **tx).await.map_err(|_| SubscriptionError::Storage)?;
    if row.get::<Vec<u8>,_>("request_hash") != hash { return Err(SubscriptionError::IdempotencyConflict); }
    row.get::<Option<Value>,_>("response").ok_or(SubscriptionError::Storage).map(Some)
}
async fn store_command(tx: &mut sqlx::Transaction<'_, sqlx::Postgres>, tenant: Uuid, key: &str, hash: &[u8], snapshot: &SubscriptionSnapshot) -> Result<(), SubscriptionError> {
    let response = serde_json::to_value(snapshot).map_err(|_| SubscriptionError::Storage)?;
    sqlx::query("UPDATE platform.saas_subscription_commands SET subscription_id=$3,response=$4 WHERE tenant_id=$1 AND idempotency_key=$2 AND request_hash=$5")
        .bind(tenant).bind(key).bind(snapshot.subscription_id).bind(response).bind(hash)
        .execute(&mut **tx).await.map_err(|_| SubscriptionError::Storage)?;
    Ok(())
}
async fn load_snapshot(tx: &mut sqlx::Transaction<'_, sqlx::Postgres>, tenant: Uuid, id: Uuid) -> Result<SubscriptionSnapshot, SubscriptionError> {
    let row = sqlx::query("SELECT id,tenant_id,product_id,offer_id,offer_version,status,starts_at,ends_at,version FROM platform.saas_subscriptions WHERE id=$1 AND tenant_id=$2")
        .bind(id).bind(tenant).fetch_one(&mut **tx).await.map_err(|_| SubscriptionError::Storage)?;
    snapshot_from_row(row)
}
fn snapshot_from_row(row: sqlx::postgres::PgRow) -> Result<SubscriptionSnapshot, SubscriptionError> {
    Ok(SubscriptionSnapshot {
        subscription_id: row.try_get("id").map_err(|_| SubscriptionError::Storage)?,
        tenant_id: row.try_get("tenant_id").map_err(|_| SubscriptionError::Storage)?,
        product_id: row.try_get("product_id").map_err(|_| SubscriptionError::Storage)?,
        offer_id: row.try_get("offer_id").map_err(|_| SubscriptionError::Storage)?,
        offer_version: row.try_get("offer_version").map_err(|_| SubscriptionError::Storage)?,
        status: row.try_get("status").map_err(|_| SubscriptionError::Storage)?,
        starts_at: row.try_get("starts_at").map_err(|_| SubscriptionError::Storage)?,
        ends_at: row.try_get("ends_at").map_err(|_| SubscriptionError::Storage)?,
        version: row.try_get("version").map_err(|_| SubscriptionError::Storage)?,
    })
}
async fn write_effects(tx: &mut sqlx::Transaction<'_, sqlx::Postgres>, s: &SubscriptionSnapshot, actor: Uuid, action: &str, previous: Option<String>, reason: Option<&str>) -> Result<(), SubscriptionError> {
    let event_id = Uuid::now_v7();
    let kind = if action == "created" { "platform.saas-subscription.created.v1" } else { "platform.saas-subscription.changed.v1" };
    let payload = json!({"eventId":event_id,"tenantId":s.tenant_id,"subscriptionId":s.subscription_id,
        "productId":s.product_id,"offerId":s.offer_id,"offerVersion":s.offer_version,
        "status":s.status,"startsAt":s.starts_at,"endsAt":s.ends_at,"aggregateVersion":s.version});
    sqlx::query("INSERT INTO platform.saas_subscription_audit (audit_id,subscription_id,tenant_id,actor_id,action,previous_status,new_status,previous_version,new_version,reason_code) VALUES ($1,$2,$3,$4,$5,$6,$7,$8,$9,$10)")
        .bind(Uuid::now_v7()).bind(s.subscription_id).bind(s.tenant_id).bind(actor).bind(action)
        .bind(previous).bind(&s.status).bind(if s.version > 1 { Some(s.version - 1) } else { None })
        .bind(s.version).bind(reason).execute(&mut **tx).await.map_err(|_| SubscriptionError::Storage)?;
    sqlx::query("INSERT INTO platform.saas_subscription_outbox (event_id,tenant_id,subscription_id,aggregate_version,event_type,payload) VALUES ($1,$2,$3,$4,$5,$6)")
        .bind(event_id).bind(s.tenant_id).bind(s.subscription_id).bind(s.version).bind(kind).bind(payload)
        .execute(&mut **tx).await.map_err(|_| SubscriptionError::Storage)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn idempotency_key_is_bounded_and_printable() {
        assert_eq!(validate_key("short"), Err(SubscriptionError::InvalidInput));
        assert_eq!(validate_key("request-key-000001"), Ok(()));
        assert_eq!(validate_key("contains space 123"), Err(SubscriptionError::InvalidInput));
    }
    #[test]
    fn request_hash_is_stable_and_command_sensitive() {
        let a = json!({"action":"cancel","version":3});
        assert_eq!(request_hash(&a).unwrap(), request_hash(&a).unwrap());
        assert_ne!(request_hash(&a).unwrap(), request_hash(&json!({"action":"renew"})).unwrap());
    }
}
