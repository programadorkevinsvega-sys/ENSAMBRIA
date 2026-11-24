# Commercial policies and monetization — Phase 1.4

PLATFORM owns the versioned offer catalogue. The migration `20261124000003_platform_saas_subscriptions.sql` adds offers and immutable version snapshots with separate entitlement and commercial-terms documents. Published versions cannot be edited.

No prices, currency, tax, trial, billing interval, auto-renewal or other commercial defaults are seeded. Offer publication requires a privileged administrative adapter, approval policy, MFA, audit and optimistic concurrency; that adapter remains an integration item. Runtime may read published versions only.

CORE evaluates and enforces entitlement snapshots. PLATFORM does not mutate CORE entitlements, collect payment, or create billing/invoice records. Billing and payments remain Phase 7.
