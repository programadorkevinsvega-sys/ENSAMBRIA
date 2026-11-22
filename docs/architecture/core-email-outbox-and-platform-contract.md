# CORE transactional email and PLATFORM product policy

## Durable email delivery

Authentication services must use `OutboxEmailDelivery` rather than a direct SMTP transport:

```rust
let email_queue = OutboxEmailDelivery::new(core_pool.clone());
let auth = AuthenticationService::new(
    core_pool.clone(),
    email_queue,
    mfa_protector,
    rate_limit_secret,
    public_base_url,
)?;
```

Run one or more worker processes independently from the request-serving process:

```rust
run_smtp_worker_from_env(core_pool, Duration::from_secs(5), 25).await?;
```

Configure these secrets in the deployment secret manager (never commit them):

- `CORE_SMTP_HOST`
- `CORE_SMTP_PORT` (STARTTLS; normally 587)
- `CORE_SMTP_USERNAME`
- `CORE_SMTP_PASSWORD`
- `CORE_SMTP_FROM`

The request path persists messages in `core.email_outbox`. Workers claim messages with row locks and expiring leases, so crashed workers do not strand jobs. Failures are retried with exponential backoff, up to ten attempts; exhausted messages become `dead` for operator inspection. The message body is never written to logs. Outbox rows contain short-lived verification/recovery links and must be protected as credentials.

## CORE–PLATFORM contract enforcement

CORE must not query PLATFORM tables or create cross-schema foreign keys. Construct `PlatformProductPolicy` with the PLATFORM-owned `ProductCatalog` adapter and call `require_tenant_capability` before product-gated operations. The method:

1. Reads the tenant/product association under PostgreSQL tenant RLS.
2. Requires the association to be active and reads its pinned configuration version.
3. Resolves the active published product through PLATFORM's catalog API.
4. Requires the capability to be enabled and its contract version to match.

Domain resolution must use `resolve_domain`, which delegates normalization and verified-domain lookup to PLATFORM. PLATFORM and CORE pools use their respective least-privilege runtime identities.

CI's PostgreSQL integration job seeds a published PLATFORM product and exercises the CORE policy against that live PostgreSQL contract, including mismatched versions, unavailable capabilities, and tenant association.
