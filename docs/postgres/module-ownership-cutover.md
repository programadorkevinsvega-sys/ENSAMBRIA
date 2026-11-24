# PostgreSQL module ownership cutover plan

**Decision:** ADR-0012 and owner confirmation on 2026-11-24.
**Status:** phase 0 prepared; data is NOT moved and legacy runtime access is NOT revoked.
**Canonical inventory:** [architecture/data-ownership.yaml](../../architecture/data-ownership.yaml).

## Approved owners and destination schemas

| Bounded context | Backend | Destination schema | Schema owner | Runtime role |
|---|---|---|---|---|
| identity-authentication | CORE | `core_identity_auth` | `core_identity_auth_schema_owner` | `core_identity_auth_runtime` |
| tenancy-membership-organization-branch-product | CORE | `core_tenancy` | `core_tenancy_schema_owner` | `core_tenancy_runtime` |
| establishment-catalog-verticals | BUSINESS | `business_establishment_catalog` | `business_establishment_catalog_schema_owner` | `business_establishment_catalog_runtime` |
| product-catalog-configuration | PLATFORM | `platform_product_catalog` | `platform_product_catalog_schema_owner` | `platform_product_catalog_runtime` |
| commercial-account-onboarding | PLATFORM | `platform_tenant_onboarding` | `platform_tenant_onboarding_schema_owner` | `platform_tenant_onboarding_runtime` |

The module runtime roles are LOGIN, NOINHERIT, NOSUPERUSER and NOBYPASSRLS. Schema-owner roles are NOLOGIN and never used by applications. `core_migrator` receives membership only to perform explicit `SET ROLE` during migrations. Passwords/credentials for LOGIN roles are provisioned out-of-band; no secret is stored in SQL, CI or this repository.

## Phase 0 — safe provisioning (this change)

`backend/core/deployment/provision-module-ownership.sql` creates the five owner/runtime role pairs and destination schemas. It revokes PUBLIC access and intentionally grants no runtime USAGE on the new schemas yet. The old schemas, tables, grants, application SQL and deployed credentials remain unchanged. This is preparatory and does not claim that data ownership cutover is complete.

CI runs this provisioning against ephemeral PostgreSQL and executes `scripts/ci/check-module-schema-isolation.sh`, which verifies:
- each destination schema is owned by its exact NOLOGIN owner role;
- each runtime role has LOGIN, NOINHERIT and NOBYPASSRLS;
- runtime roles have no USAGE on destination schemas before cutover;
- cross-module USAGE is denied;
- PUBLIC has neither USAGE nor CREATE.

## Phase 1 — CORE identity and tenancy separation

Move identity/authentication tables, auth-only lookup/state-transition functions, security audit and email Outbox into `core_identity_auth`. Move tenant, organization, branch, membership, role/permission, tenant-product and tenant-created Outbox data into `core_tenancy`.

This is not a blind `ALTER TABLE SET SCHEMA`:
- `core.tenant_memberships.identity_id` must become an opaque UUID reference; remove the cross-owner FK to identities.
- Auth SQL functions currently join identities with tenants/memberships and call `has_permission` / regional context functions. Replace those SQL cross-context reads with the public tenancy contract (trait + own DTOs, Protobuf v1). The CORE composition root injects the in-process implementation. Do not leave SECURITY DEFINER functions or views that bridge schemas.
- Auth identity validation and tenancy membership/authorization remain distinct responsibilities. The auth module does not query tenancy tables.
- `core.outbox_events` belongs to tenancy because tenant creation and its Outbox row commit atomically. `core.email_outbox` belongs to identity-authentication.
- Preserve RLS semantics and validate transaction-local `app.tenant_id` / `app.actor_id` with the new schemas and roles.

## Phase 2 — BUSINESS

Move `business.establishments` to `business_establishment_catalog.establishments`. Replace SQL reads of `core.tenant_memberships`, `core.organizations` and `core.branches` with tenancy contract operations. BUSINESS receives no CORE schema grants, FKs or SQL joins. Validate tenant, organization and branch ownership/active state in CORE before BUSINESS commits its own local transaction; do not pretend this is a distributed atomic transaction.

The current BUSINESS executable imports CORE implementation and starts HTTP domain endpoints. Those are separate architectural violations and must be removed in the same functional cutover: BUSINESS consumes public contracts and only BFFs expose business HTTP.

## Phase 3 — PLATFORM catalog and onboarding

Move product catalog tables/functions/views to `platform_product_catalog`; move Inbox and tenant onboarding to `platform_tenant_onboarding`.

- `platform.tenant_onboardings.product_id` currently has an FK to `platform.products`; remove it because the two contexts have separate ownership. Keep product_id as an opaque ID.
- The onboarding consumer currently queries `platform.active_product_catalog`. Replace that direct SQL with the product-catalog contract before revoking catalog access from the consumer role.
- Product-catalog runtime and onboarding consumer have independent credentials and schemas. The consumer only writes its Inbox/onboarding data and asks product-catalog through its public contract for product/version validation.
- CORE tenant-product association uses the PLATFORM contract; no cross-schema FK or direct SQL.

## Phase 4 — cutover and denial tests

For each module, deploy code that reads/writes only its destination schema with its runtime role. Run data reconciliation and count/hash comparisons before/after move. Use an expand/contract sequence with rollback plan and backup. Then revoke legacy broad grants and prove negative access using the exact runtime roles:
- BUSINESS cannot USAGE/SELECT/DML CORE schemas.
- CORE identity-auth cannot USAGE/SELECT/DML tenancy schema, and vice versa.
- PLATFORM onboarding cannot access product-catalog schema; product-catalog cannot access onboarding schema.
- Workers and integration tests use the same module runtime role, not owner, superuser or setup credentials.

The migration is accepted only after full PostgreSQL integration/E2E tests, permission-denial tests, repeatable migration checks, staging deployment and runtime evidence on the exact SHA. Phase 0 provisioning alone is not acceptance.

## Rollback and safety

No existing data is moved or legacy grants revoked in phase 0. Later schema/table moves require a backup, tested forward/rollback strategy, transaction/lock assessment, object dependency inventory (FKs, triggers, policies, functions, views, sequences, default privileges) and expand/contract compatibility. If the application or tests still depend on legacy schema names, the cutover must not be applied to staging/production. No compatibility view may grant a consumer access to another module's underlying data.
