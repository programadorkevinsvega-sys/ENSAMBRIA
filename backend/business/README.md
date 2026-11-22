# BUSINESS runtime

The BUSINESS bounded context now has a runnable HTTP process and a first tenant-scoped vertical slice: establishment registration and listing.

## Process configuration

Required:
- `DATABASE_URL`: CORE pool URL (role with CORE runtime permissions).
- `BUSINESS_DATABASE_URL`: BUSINESS pool URL (least-privilege `business_runtime` role).
- `CORE_MFA_ENCRYPTION_KEY_VERSION`, `CORE_MFA_ENCRYPTION_KEY_HEX`.
- `CORE_AUTH_RATE_LIMIT_SECRET_HEX`.
- `CORE_PUBLIC_BASE_URL`.
- `CORE_DATABASE_MAX_CONNECTIONS` and `CORE_DATABASE_ACQUIRE_TIMEOUT_MS` (required by CORE's PostgreSQL pool configuration).

Optional:
- `BUSINESS_HTTP_LISTEN_ADDRESS` (default `0.0.0.0:8080`).
- `BUSINESS_DATABASE_MAX_CONNECTIONS` (default 32).
- Redis configuration documented in `docs/architecture/core-email-outbox-and-platform-contract.md`.

Build and run:
```sh
cargo run -p ensambria-business --bin business
```

Apply CORE migrations first. Provision `business_schema_owner` as NOLOGIN and `business_runtime` as a least-privilege LOGIN role; grant `business_schema_owner` to the dedicated `core_migrator` role and grant `CONNECT` to `business_runtime`. Run `migrations/bootstrap/00000000000000_business_schema.sql` as the CORE schema administrator (it grants `USAGE`/`SELECT` on CORE tables to `business_runtime`), then run `migrations/20261122000001_business_establishments.sql` after `SET ROLE business_schema_owner` using the migration role. Do not run the bootstrap as `business_schema_owner`, because it does not own the CORE schema. Runtime roles must not own tables or have BYPASSRLS.

## HTTP contract

- `GET /health`
- `POST /v1/auth/register` — `{ "email", "display_name"?, "password" }`
- `POST /v1/auth/login` — `{ "email", "password" }`
- `POST /v1/auth/verify-email` — `{ "token" }`
- `POST /v1/auth/recovery` — `{ "email" }`
- `POST /v1/auth/recovery/complete` — `{ "token", "new_password" }`
- `POST /v1/auth/refresh` — `{ "refresh_token" }`
- `POST /v1/auth/mfa/challenge` — `{ "challenge_token", "factor" }`
- `POST /v1/auth/mfa/enrollment/begin` — Bearer token plus `{ "current_password" }`
- `POST /v1/auth/mfa/enrollment/confirm` — Bearer token plus `{ "current_password", "code" }`
- `POST /v1/auth/mfa/disable` — Bearer token plus `{ "current_password" }`
- `POST /v1/auth/verification/resend` — `{ "email" }`
- `POST /v1/auth/logout` and `POST /v1/auth/logout-all` — Bearer token
- `GET /v1/business/establishments`
- `POST /v1/business/establishments` — `{ "organization_id", "branch_id"?, "display_name" }`

BUSINESS requests require `Authorization: Bearer <access-token>` and `X-Tenant-Id: <tenant-uuid>`. The identity must have an active CORE membership for that tenant. Every BUSINESS query sets `app.tenant_id` inside its database transaction; PostgreSQL RLS is the isolation boundary. Organization and branch identifiers remain opaque cross-context references and are checked against CORE before creation.

The initial slice deliberately does not claim to implement every placeholder module in `backend/business/modules`. CRM, orders, inventory, billing, workforce, marketing and vertical-specific workflows remain separate implementation work.
