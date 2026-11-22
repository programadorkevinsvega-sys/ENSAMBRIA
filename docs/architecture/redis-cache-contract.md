# Redis TTL cache contract

## Purpose and boundary

Redis is a distributed, disposable cache only. PostgreSQL remains the source of truth for identity, credentials, sessions, tenant memberships, authorization, product associations, catalog state, operational records and durable work. Redis must never be used as a second database, durable queue, session authority, revocation authority, event bus or replacement for NATS.

The neutral Rust primitive lives in `backend/shared-cache`; each backend owns its own namespace, cache keys, TTLs, invalidation and fallback behavior. BFFs and frontends do not access Redis directly. Cache payloads are minimized and contain no passwords, bearer/refresh/recovery tokens, MFA secrets, payment data or unnecessary personal data.

## Runtime contract

- Use TLS (`rediss://`) with authenticated, least-privilege Redis ACLs in staging/production; never disable certificate verification.
- Construct one long-lived client per backend process and share it across requests. Do not open a Redis connection per request.
- The shared client uses an asynchronous `ConnectionManager`, bounded exponential reconnect attempts, bounded delay and connection/response timeouts.
- Every entry must have a finite TTL. Keys are namespaced by backend and schema version; tenant-scoped keys, when introduced, must include the canonical tenant identifier and any relevant product/scope/version dimensions.
- Cache misses, corrupt values and Redis read/write failures fall back to the owning PostgreSQL/PLATFORM contract. A cache outage must not authorize, deny or corrupt a business operation.
- Cache invalidation follows the owner’s persisted version/change contract. TTL is a bounded safety net, not the only invalidation strategy for mutable high-impact configuration.
- Redis eviction policy must be configured for cache workloads (for example, `allkeys-lru` or a measured alternative), with explicit maxmemory, TLS, ACL, backups/restore expectations, metrics and alerts. Redis persistence is not relied upon for correctness.
- Production topology must be a managed highly available, horizontally shardable Redis/Valkey deployment. The current `TtlCache` primitive and standalone `ConnectionManager` integration are an initial adapter, not evidence that a single Redis node supports the production concurrency target. Cluster/Sentinel topology, connection budget, shard sizing and failover behavior must be selected and load-tested before production.
- Redis is not a substitute for NATS Core fan-out, JetStream durability, PostgreSQL cursor/snapshot recovery or transactional Outbox/Inbox.

## Phase 1 — CORE / PLATFORM

The shared cache primitive is introduced in Phase 1 so every backend can reuse the same transport, key validation, TTL enforcement, timeout and reconnect behavior.

PLATFORM owns the first cache entry: domain-to-product resolution as a routing hint, under `platform:domain:v1:<normalized-domain>`, TTL 10 seconds. A cache hit is never an authorization decision. The active product is reloaded through PLATFORM's authoritative catalog contract; CORE still validates the tenant/product association, actor, scope and capability against authoritative state. Cache errors fall back to the database lookup and are logged without including domain/customer data. Product/domain mutation flows must invalidate affected keys when those flows are implemented; TTL bounds stale routing hints meanwhile.

Authentication rate limiting remains PostgreSQL-backed in this cut. Redis must not become the authority for account lockout, session revocation, MFA challenges or recovery tokens. No authentication secret or bearer token is cached.

The current CORE executable is still a health-only runtime and PLATFORM has no production runtime composition in this branch. The cache adapter and PLATFORM catalog integration are reusable code, but production activation requires wiring the cache-enabled catalog into the future authorized service composition and provisioning the Redis endpoint through the secret manager.

## Phase 2 — BUSINESS

BUSINESS owns its own cache namespace and invalidation. Once Phase 2 introduces the relevant persisted capabilities, Redis TTL cache applies to read-heavy, non-authoritative projections such as:
- service/catalog and vertical metadata projections, keyed by product + tenant + establishment/scope + catalog version;
- workforce availability/read-model hints and resource/service presentation data, never the final eligibility or booking decision;
- short-lived tenant-scoped dashboard/read-model aggregates where privacy and authorization are rechecked on every request.

Reservations, availability holds, capacity, workforce eligibility, prices/charges, permissions and operational state are revalidated by their owning PostgreSQL-backed use cases before mutation/confirmation. A stale cache may affect display or cause a retry, never create a double booking, expose another tenant's data, grant access or confirm a resource that is no longer available. Each BUSINESS module/vertical defines explicit key schema, TTL, invalidation on owner writes and stale-read behavior in its Phase 2 integration matrix. Do not create BUSINESS runtime or placeholder packages before its roadmap subphase begins.

## Required verification and production gates

- Real Redis integration tests for namespaced keys, TTL expiry, deletion, cache miss, malformed values and fallback behavior.
- Restart/reconnect test against the isolated development Redis service, with the documented destructive-test restrictions in `AGENTS.md`.
- PostgreSQL + Redis tests for each owner’s actual cache-aside flow, including tenant isolation, stale-version invalidation, concurrent misses and Redis outage.
- Load/stress tests using k6 against the actual API/BFF/backend path, including hot-key distribution, pool/connection budgets, p95/p99 latency, Redis CPU/memory/network, eviction and failover.
- A measured sharded/HA topology and capacity plan before any claim that the system is ready for hundreds of thousands of businesses or millions of concurrent users.
