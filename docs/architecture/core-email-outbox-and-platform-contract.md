# CORE transactional email and PLATFORM product policy

## Authentication composition and durable email

The production composition factory is `identity_authentication::production::authentication_service_from_env(pool)`. It constructs `AuthenticationService<OutboxEmailDelivery>`; production authentication must not instantiate the service with direct SMTP. The service persists verification and recovery tokens as hashes in CORE and enqueues the corresponding message in `core.email_outbox`.

Required authentication secrets/configuration are injected into the API process by the deployment secret manager:

- `CORE_MFA_ENCRYPTION_KEY_VERSION`
- `CORE_MFA_ENCRYPTION_KEY_HEX` (32-byte AES-256 key, 64 hexadecimal characters)
- `CORE_AUTH_RATE_LIMIT_SECRET_HEX` (at least 32 random bytes, hexadecimal)
- `CORE_PUBLIC_BASE_URL` (HTTPS origin)

The AES key is loaded from a secret-manager-provided environment variable and is not committed. This is an environment-backed key protector, not a native KMS/Vault integration; KMS/Vault key unwrap and rotation remain a deployment/security integration requirement before production acceptance.

## SMTP worker process

Build and run the separate `core-email-worker` binary. It connects using its own PostgreSQL login, polls every five seconds, and processes batches of 25. The systemd unit is installed by staging deployment but intentionally is not enabled automatically: enable it only after the dedicated worker environment file has been provisioned.

Before enabling the worker, the database operator must provision its dedicated login (the restricted migrator cannot create roles), grant database connection, and apply the outbox grants. For example, using the approved database administration channel and a secret-managed password:

```sql
CREATE ROLE core_email_worker LOGIN NOSUPERUSER NOCREATEDB NOCREATEROLE NOINHERIT PASSWORD '<injected-secret>';
GRANT CONNECT ON DATABASE ensambria TO core_email_worker;
GRANT USAGE ON SCHEMA core TO core_email_worker;
GRANT SELECT, UPDATE ON core.email_outbox TO core_email_worker;
```

The migration applies the grants automatically when the role already exists; it deliberately does not create privileged database logins. The worker environment file must use this dedicated login.

Worker environment file: `/etc/apps/ensambria/core/email-worker.env`, owned by root with mode 0600. It contains:

- `DATABASE_URL`: the dedicated `core_email_worker` PostgreSQL role (not `core_runtime`)
- `CORE_DATABASE_MAX_CONNECTIONS`
- `CORE_DATABASE_ACQUIRE_TIMEOUT_MS`
- `CORE_SMTP_HOST`
- `CORE_SMTP_PORT` (STARTTLS; normally 587)
- `CORE_SMTP_USERNAME`
- `CORE_SMTP_PASSWORD`
- `CORE_SMTP_FROM`

The worker validates the SMTP configuration at startup and uses STARTTLS. Missing SMTP credentials mean delivery is not configured; CI's fake transport does not prove real provider delivery. After provisioning the secret file, an operator can enable and start `ensambria-core-email-worker.service`.

Messages are claimed with row locks and expiring leases, so crashed workers do not strand jobs. Failures are retried with exponential backoff, up to ten attempts; exhausted messages become `dead` for operator inspection. Delivery is at-least-once: a crash after SMTP accepts a message but before the database marks it sent can cause a duplicate. Message bodies contain short-lived links and are scrubbed after success or terminal failure; database access to the outbox must remain restricted.

The PostgreSQL integration test `authentication_outbox.rs` exercises the complete registration → outbox → worker → email verification and recovery request → outbox → worker → password reset path using a deterministic in-process delivery transport. The separate authentication integration suite exercises TOTP enrollment, challenge, recovery codes, replay rejection, and MFA disable. A real provider acceptance test still requires provisioned SMTP credentials and a controlled mailbox.

## CORE–PLATFORM contract enforcement

CORE must not query PLATFORM tables or create cross-schema foreign keys. Construct `PlatformProductPolicy` with the PLATFORM-owned `ProductCatalog` adapter and call `require_tenant_capability` before product-gated operations. The method:

1. Reads the tenant/product association under PostgreSQL tenant RLS.
2. Requires the association to be active and reads its pinned configuration version.
3. Resolves the active published product through PLATFORM's catalog API.
4. Requires the capability to be enabled and its contract version to match.

Domain resolution must use `resolve_domain`, which delegates normalization and verified-domain lookup to PLATFORM. PLATFORM and CORE pools use their respective least-privilege runtime identities.

CI's PostgreSQL integration job seeds a published PLATFORM product and exercises the CORE policy against that live PostgreSQL contract, including mismatched versions, unavailable capabilities, and tenant association.

## Redis shared-cache configuration

The shared cache reads `PLATFORM_REDIS_URL` and optional `PLATFORM_REDIS_POOL_SIZE` (default 4, maximum 64 connections per process). Set `ENSAMBRIA_ENV=production` in production deployments. In that mode startup rejects Redis URLs unless they use `rediss://` and include both an ACL username and password. Credentials must be injected by the deployment secret manager; do not commit them or place them in logs. Non-production environments may use an isolated `redis://` service.

The cache is an optimization only: PostgreSQL remains authoritative and PLATFORM falls back to PostgreSQL when Redis is unavailable. This client currently targets a Redis endpoint and does not implement Redis Sentinel discovery, Redis Cluster sharding, or managed failover orchestration; those capabilities must be supplied and tested by the deployment topology before claiming high availability or million-user capacity. Pool size is per process, so total Redis connections equal the configured pool multiplied by the number of service replicas.
