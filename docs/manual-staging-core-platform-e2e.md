# Manual CORE -> PLATFORM staging E2E

This is an operator-invoked staging acceptance test. It is deliberately not called
by GitHub Actions, deployment workflows, or systemd.

## What it proves

- Creates one fresh tenant through CORE's real tenant lifecycle transaction.
- Verifies its Outbox row, then runs the repository's real Outbox relay against
  the staging PostgreSQL and staging JetStream.
- Crashes the relay after JetStream PubAck and restarts it after lease expiry.
- Runs the real PLATFORM consumer with a dedicated, uniquely named durable that
  starts after the stream's current last sequence. It does not attach to or move
  the regular PLATFORM durable consumer's offset.
- Crashes the consumer before the Inbox transaction, then after Inbox/onboarding
  commit but before ACK, and verifies redelivery takes the duplicate path.
- Asserts exactly one applied Inbox row and exactly one onboarding projection.

## Requirements

Run from a clean local ENSAMBRIA checkout with SSH access to the staging VM.
The VM needs Rust/Cargo, PostgreSQL `psql`, NATS CLI (`nats`), `jq`, `pgrep`,
and access to the real staging PostgreSQL and JetStream endpoints.

Create a shell-compatible env file on the VM, readable by the SSH account. Do
not commit it. It must contain:

- `ENSAMBRIA_ENVIRONMENT=staging`
- `CORE_TENANT_DATABASE_URL`
- `CORE_OUTBOX_DATABASE_URL`
- `PLATFORM_EVENT_DATABASE_URL`
- `PLATFORM_VERIFY_DATABASE_URL`
- `ENSAMBRIA_NATS_URL`
- `ENSAMBRIA_E2E_PRODUCT_ID`: UUID of an active staging product whose
  `configuration_version` is 1.

Use the actual staging credentials and endpoints. Do not point these variables
at CI containers, local Docker services, or production. The env file is sourced
as shell code and therefore must be owned/managed by the staging operator.

## Run

From the repository root:

```bash
ENSAMBRIA_STAGING_SSH_TARGET=deploy@staging-host \
ENSAMBRIA_STAGING_ENV_FILE=/etc/apps/ensambria/manual-e2e.env \
bash scripts/manual-staging-core-platform-e2e.sh
```

Optionally set `ENSAMBRIA_STAGING_EXPECTED_HOSTNAME` to enforce an exact VM
hostname. SSH host-key verification is strict; provision the VM host key in
the local known_hosts file first.

The launcher asks for the exact confirmation `RUN-STAGING-E2E`. It uploads a
temporary source snapshot, builds debug binaries on the VM (fault injection is
debug-build-only), runs the scenario, copies the report into
`reports/manual-staging-e2e/` locally, and removes the temporary upload.

## Safety boundaries

- This writes one real test tenant, one Outbox event, one Inbox row and one
  onboarding projection into staging. Those records are retained as evidence;
  the script does not delete business data.
- It does not restart PostgreSQL, Redis, NATS nodes, systemd services, or the
  deployed CORE/BUSINESS services.
- It refuses to run if the environment marker is not exactly `staging`, the
  database name looks production-like, the selected product is not active,
  an Outbox relay process is already running, or pre-existing pending/expired
  Outbox events exist.
- It requires NATS CLI to snapshot the stream's last sequence and creates a
  separate durable starting at the next sequence, leaving the normal PLATFORM
  durable consumer untouched. Run during a quiet staging window: concurrent
  tenant-created events on the same subject can still share the next sequence.
- The temporary durable is retained for audit and can be inspected in JetStream.
- A failure does not imply rollback of the test tenant or its committed rows;
  inspect the report and staging records before retrying.

This is separate from CI's ephemeral PostgreSQL/Redis/NATS integration
infrastructure and separate from CD's staging deployment. No CI/CD workflow
references either manual script.
