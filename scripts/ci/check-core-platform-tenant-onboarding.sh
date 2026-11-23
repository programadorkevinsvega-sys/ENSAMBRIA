#!/usr/bin/env bash
set -euo pipefail

export ENSAMBRIA_NATS_URL="${ENSAMBRIA_NATS_URL:-nats://127.0.0.1:4222}"
: "${PLATFORM_VERIFY_DATABASE_URL:?PLATFORM_VERIFY_DATABASE_URL is required}"
docker compose -f infra/nats/compose.ci.yml exec -T nats-box \
  nats --server nats://nats-1:4222 stream add ENSAMBRIA \
  --subjects "ensambria.>" --storage file --replicas 3 --retention limits --defaults
: "${CORE_TENANT_DATABASE_URL:?CORE_TENANT_DATABASE_URL is required}"
: "${CORE_OUTBOX_DATABASE_URL:?CORE_OUTBOX_DATABASE_URL is required}"
: "${PLATFORM_EVENT_DATABASE_URL:?PLATFORM_EVENT_DATABASE_URL is required}"

cargo build --locked -p ensambria-core --bin core-outbox-relay --bin core-tenant-onboarding-e2e
cargo build --locked -p ensambria-platform --bin platform-tenant-onboarding-consumer

target/debug/platform-tenant-onboarding-consumer > /tmp/ensambria-platform-consumer.log 2>&1 &
consumer_pid=$!
target/debug/core-outbox-relay > /tmp/ensambria-core-outbox-relay.log 2>&1 &
relay_pid=$!
cleanup() {
  kill "$relay_pid" "$consumer_pid" 2>/dev/null || true
  wait "$relay_pid" "$consumer_pid" 2>/dev/null || true
}
trap cleanup EXIT

sleep 2
target/debug/core-tenant-onboarding-e2e

for attempt in $(seq 1 60); do
  applied="$(psql "$PLATFORM_VERIFY_DATABASE_URL" -Atqc     "SELECT count(*) FROM platform.tenant_onboardings WHERE tenant_id='018f0f2a-7b3c-7abc-8def-0123456789ae' AND product_id='018f0f2a-7b3c-7abc-8def-0123456789ab' AND status='onboarding_pending'")"
  published="$(psql "$CORE_OUTBOX_DATABASE_URL" -Atqc     "SELECT count(*) FROM core.outbox_events WHERE aggregate_id='018f0f2a-7b3c-7abc-8def-0123456789ae' AND status='published'")"
  inbox="$(psql "$PLATFORM_VERIFY_DATABASE_URL" -Atqc     "SELECT count(*) FROM platform.inbox_messages WHERE aggregate_id='018f0f2a-7b3c-7abc-8def-0123456789ae' AND status='applied'")"
  if [[ "$applied" == "1" && "$published" == "1" && "$inbox" == "1" ]]; then
    echo "CORE transaction -> Outbox -> JetStream publish ACK -> PLATFORM Inbox + onboarding commit -> consumer ACK verified"
    exit 0
  fi
  sleep 1
done

echo "CORE -> PLATFORM event flow did not reach published/applied state" >&2
cat /tmp/ensambria-core-outbox-relay.log >&2 || true
cat /tmp/ensambria-platform-consumer.log >&2 || true
exit 1
