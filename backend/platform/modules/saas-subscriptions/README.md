# SaaS subscriptions — Phase 1.4

PLATFORM owns versioned commercial offers and the tenant contract. CORE owns runtime entitlement evaluation/enforcement. The tenant-onboarding consumer only creates `onboarding_pending`; it never creates a subscription.

The first executable slice includes immutable published offer versions, explicit subscription creation, suspend/reactivate/renew/cancel/expire commands, optimistic aggregate versions, tenant-scoped RLS, command idempotency, audit history, and a transactional Outbox.

The authenticated application adapter must validate CORE-issued actor/tenant claims, membership, role, MFA/privileged requirements and request purpose before invoking the module; it must set transaction-local `app.tenant_id`. This library is not a browser API and does not authenticate callers. The administrative offer authoring/publication adapter remains a required integration before commercial operators can manage offers.

Renewal only changes the contract end timestamp. There is no invented price, currency, tax, billing interval, trial, auto-renewal, grace period, payment, invoice, or ledger effect. Expiration is an explicit idempotent command; a scheduler/worker must be integrated separately. Outbox rows are transactionally durable, but a JetStream publisher and CORE entitlement consumer must be implemented and proven before this phase is accepted.
