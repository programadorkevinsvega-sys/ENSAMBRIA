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

## Runtime BUSINESS, Redis topologies and API entrypoint

The executable `ensambria-business` now serves the functional HTTP entrypoint. It composes the CORE authentication service and the BUSINESS establishment service in one process while retaining separate CORE and BUSINESS PostgreSQL pools and runtime roles. The first BUSINESS vertical slice is intentionally limited to tenant-scoped establishment listing/creation; the other module directories remain planned boundaries, not completed features.

BUSINESS endpoints require a valid CORE access token and `X-Tenant-Id`. The service verifies an active CORE tenant membership, validates organization/branch references through CORE, sets `app.tenant_id` transaction-locally, and relies on forced PostgreSQL RLS for BUSINESS row isolation. Redis caches only the establishment list for 10 seconds and is invalidated after creation. PostgreSQL remains authoritative.

Authentication endpoints are exposed from the same runtime, including registration, login, email verification/resend, recovery, refresh, MFA challenge/enrollment/disable, and logout. Redis is used for atomic cross-replica login/recovery/MFA throttling. If Redis is unavailable, CORE falls back to its PostgreSQL rate-limit function. Access tokens, refresh tokens, MFA secrets, and membership decisions are never cached.

### Redis topology selection

Set `PLATFORM_REDIS_MODE` to `direct` (default), `sentinel`, or `cluster`. Set `PLATFORM_REDIS_POOL_SIZE` to the per-process connection count (default 4, maximum 64).

- Direct: `PLATFORM_REDIS_URL=rediss://acl-user:secret@redis.example:6379/`.
- Sentinel: `PLATFORM_REDIS_SENTINEL_URLS` is a semicolon-separated list of authenticated Sentinel `rediss://` endpoints; `PLATFORM_REDIS_SENTINEL_MASTER_NAME` identifies the monitored primary; `PLATFORM_REDIS_USERNAME` and `PLATFORM_REDIS_PASSWORD` authenticate to the discovered data primary. Each operation requests the current primary from Sentinel, so the next operation rediscovers after failover.
- Cluster: `PLATFORM_REDIS_CLUSTER_URLS` is a semicolon-separated list of authenticated `rediss://` seed nodes. redis-rs discovers cluster slots and refreshes topology/reconnects when it observes topology changes.

In production set `ENSAMBRIA_ENV=production`. Production rejects non-TLS URLs or URLs without ACL credentials. Inject every credential from the secret manager; never commit it. Configure Sentinel with at least three independent Sentinel nodes and a primary plus replicas, or Redis Cluster with at least three primaries and replicas distributed across failure domains. Configure Redis persistence/replication and monitor replication lag, failovers, evictions, memory, command latency and connection saturation. These application clients implement topology discovery/routing; they do not provision Redis infrastructure or prove that a specific deployment has quorum, replicas, persistence or cross-zone resilience.

The staging k6 workflow is manually dispatched and requires `ENSAMBRIA_STAGING_FUNCTIONAL_URL`, `ENSAMBRIA_STAGING_TEST_TOKEN`, `ENSAMBRIA_STAGING_TEST_HEADERS_JSON` (including `X-Tenant-Id`), and optionally `ENSAMBRIA_STAGING_TEST_BODY_JSON`. It targets a real functional route over HTTPS, rejects health/metrics paths, and gates error rate and p95/p99 latency. A successful k6 run is still required before making a capacity claim; CI does not invent production credentials or run a synthetic health endpoint as a business benchmark.
