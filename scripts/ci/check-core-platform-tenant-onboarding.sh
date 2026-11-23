#!/usr/bin/env bash
set -euo pipefail

COMPOSE=(docker compose -f infra/nats/compose.ci.yml)
BOX=("${COMPOSE[@]}" exec -T nats-box)
RELAY_PID=""
CONSUMER_PID=""

: "${CORE_TENANT_DATABASE_URL:?CORE_TENANT_DATABASE_URL is required}"
: "${CORE_OUTBOX_DATABASE_URL:?CORE_OUTBOX_DATABASE_URL is required}"
: "${PLATFORM_EVENT_DATABASE_URL:?PLATFORM_EVENT_DATABASE_URL is required}"
: "${PLATFORM_VERIFY_DATABASE_URL:?PLATFORM_VERIFY_DATABASE_URL is required}"
export ENSAMBRIA_NATS_URL="${ENSAMBRIA_NATS_URL:-nats://127.0.0.1:4222}"

stop_process() {
  local pid="$1"
  if [[ -n "$pid" ]] && kill -0 "$pid" 2>/dev/null; then
    kill "$pid" 2>/dev/null || true
    wait "$pid" 2>/dev/null || true
  fi
}

cleanup() {
  stop_process "$RELAY_PID"
  stop_process "$CONSUMER_PID"
}
trap cleanup EXIT

wait_for_stream() {
  local server="$1"
  for attempt in $(seq 1 60); do
    if "${BOX[@]}" nats --server "$server" stream info ENSAMBRIA >/dev/null 2>&1; then
      return 0
    fi
    sleep 1
  done
  echo "JetStream stream did not become available through $server" >&2
  return 1
}

wait_for_log() {
  local file="$1"
  local pattern="$2"
  local process_id="$3"
  for attempt in $(seq 1 60); do
    if grep -q "$pattern" "$file"; then
      return 0
    fi
    if ! kill -0 "$process_id" 2>/dev/null; then
      echo "Process exited before expected fault-injection point: $pattern" >&2
      cat "$file" >&2 || true
      return 1
    fi
    sleep 1
  done
  echo "Timed out waiting for fault-injection point: $pattern" >&2
  cat "$file" >&2 || true
  return 1
}

wait_for_database_value() {
  local database_url="$1"
  local query="$2"
  local expected="$3"
  local description="$4"
  for attempt in $(seq 1 60); do
    local actual
    actual="$(psql "$database_url" -Atqc "$query")"
    if [[ "$actual" == "$expected" ]]; then
      return 0
    fi
    sleep 1
  done
  echo "Timed out waiting for $description (expected=$expected)" >&2
  psql "$database_url" -c "$query" >&2 || true
  return 1
}

for port in 8222 8223 8224; do
  curl --fail --silent "http://127.0.0.1:${port}/healthz?js-enabled-only=true" >/dev/null
done

if ! "${BOX[@]}" nats --server nats://nats-1:4222 stream info ENSAMBRIA >/dev/null 2>&1; then
  "${BOX[@]}" nats --server nats://nats-1:4222 stream add ENSAMBRIA \
    --subjects "ensambria.>" --storage file --replicas 3 --retention limits --defaults
fi

cargo build --locked -p ensambria-core --bin core-outbox-relay --bin core-tenant-onboarding-e2e
cargo build --locked -p ensambria-platform --bin platform-tenant-onboarding-consumer

# Create the real CORE tenant and its transactional Outbox row while the relay is stopped.
e2e_output="$(target/debug/core-tenant-onboarding-e2e)"
printf '%s\n' "$e2e_output"
event_id="$(sed -n 's/^EVENT_ID=//p' <<<"$e2e_output")"
tenant_id="$(sed -n 's/^TENANT_ID=//p' <<<"$e2e_output")"
[[ -n "$event_id" && -n "$tenant_id" ]]

outbox_query="SELECT status FROM core.outbox_events WHERE event_id='$event_id'"
wait_for_database_value "$CORE_OUTBOX_DATABASE_URL" "$outbox_query" "pending" "committed Outbox event before relay startup"

# Crash the relay after JetStream confirms the publish but before PostgreSQL marks it published.
ENSAMBRIA_ENABLE_FAULT_INJECTION=1 \
CORE_OUTBOX_LEASE_SECONDS=3 \
ENSAMBRIA_TEST_PAUSE_AFTER_PUBACK_EVENT_ID="$event_id" \
target/debug/core-outbox-relay > /tmp/ensambria-core-outbox-relay-first.log 2>&1 &
RELAY_PID=$!
wait_for_log /tmp/ensambria-core-outbox-relay-first.log \
  "FAULT_INJECTION: paused after JetStream PubAck" "$RELAY_PID"
stop_process "$RELAY_PID"
RELAY_PID=""

leased="$(psql "$CORE_OUTBOX_DATABASE_URL" -Atqc "SELECT status FROM core.outbox_events WHERE event_id='$event_id'")"
[[ "$leased" == "leased" ]]

# The lease expires; a restarted relay republishes with the same Nats-Msg-Id.
# JetStream must deduplicate the retry while the Outbox eventually reaches published.
sleep 4
target/debug/core-outbox-relay > /tmp/ensambria-core-outbox-relay-restart.log 2>&1 &
RELAY_PID=$!
wait_for_database_value "$CORE_OUTBOX_DATABASE_URL" "$outbox_query" "published" "Outbox recovery after relay crash"
stop_process "$RELAY_PID"
RELAY_PID=""

stream_messages="$("${BOX[@]}" nats --server nats://nats-1:4222 stream info ENSAMBRIA | awk '/Messages:/ {print $2; exit}')"
[[ "$stream_messages" == "1" ]] || {
  echo "JetStream stored $stream_messages messages; expected one deduplicated event" >&2
  exit 1
}

# Restart the complete three-node JetStream cluster with the durable event persisted.
"${COMPOSE[@]}" stop nats-1 nats-2 nats-3
"${COMPOSE[@]}" start nats-1 nats-2 nats-3
for port in 8222 8223 8224; do
  for attempt in $(seq 1 60); do
    if curl --fail --silent "http://127.0.0.1:${port}/healthz?js-enabled-only=true" >/dev/null; then
      break
    fi
    sleep 1
  done
  curl --fail --silent "http://127.0.0.1:${port}/healthz?js-enabled-only=true" >/dev/null
done
wait_for_stream nats://nats-1:4222

# First delivery: kill PLATFORM after JetStream delivery but before opening/committing Inbox.
ENSAMBRIA_ENABLE_FAULT_INJECTION=1 \
PLATFORM_JETSTREAM_ACK_WAIT_SECONDS=2 \
ENSAMBRIA_TEST_PAUSE_BEFORE_APPLY_EVENT_ID="$event_id" \
target/debug/platform-tenant-onboarding-consumer > /tmp/ensambria-platform-consumer-before-commit.log 2>&1 &
CONSUMER_PID=$!
wait_for_log /tmp/ensambria-platform-consumer-before-commit.log \
  "FAULT_INJECTION: paused after JetStream delivery and before Inbox transaction" "$CONSUMER_PID"
stop_process "$CONSUMER_PID"
CONSUMER_PID=""

inbox_before_commit="$(psql "$PLATFORM_VERIFY_DATABASE_URL" -Atqc "SELECT count(*) FROM platform.inbox_messages WHERE event_id='$event_id'")"
onboarding_before_commit="$(psql "$PLATFORM_VERIFY_DATABASE_URL" -Atqc "SELECT count(*) FROM platform.tenant_onboardings WHERE tenant_id='$tenant_id'")"
[[ "$inbox_before_commit" == "0" && "$onboarding_before_commit" == "0" ]]

# Redelivery: commit Inbox + onboarding, then kill before ACK. JetStream must redeliver again.
ENSAMBRIA_ENABLE_FAULT_INJECTION=1 \
PLATFORM_JETSTREAM_ACK_WAIT_SECONDS=2 \
ENSAMBRIA_TEST_PAUSE_AFTER_COMMIT_EVENT_ID="$event_id" \
target/debug/platform-tenant-onboarding-consumer > /tmp/ensambria-platform-consumer-after-commit.log 2>&1 &
CONSUMER_PID=$!
wait_for_log /tmp/ensambria-platform-consumer-after-commit.log \
  "FAULT_INJECTION: paused after Inbox/onboarding commit and before JetStream ACK" "$CONSUMER_PID"
stop_process "$CONSUMER_PID"
CONSUMER_PID=""

inbox_after_commit="$(psql "$PLATFORM_VERIFY_DATABASE_URL" -Atqc "SELECT count(*) FROM platform.inbox_messages WHERE event_id='$event_id' AND status='applied'")"
onboarding_after_commit="$(psql "$PLATFORM_VERIFY_DATABASE_URL" -Atqc "SELECT count(*) FROM platform.tenant_onboardings WHERE tenant_id='$tenant_id' AND source_event_id='$event_id' AND status='onboarding_pending'")"
[[ "$inbox_after_commit" == "1" && "$onboarding_after_commit" == "1" ]]

# Final redelivery must hit the Inbox duplicate path, preserve one onboarding, and ACK.
PLATFORM_JETSTREAM_ACK_WAIT_SECONDS=2 \
target/debug/platform-tenant-onboarding-consumer > /tmp/ensambria-platform-consumer-redelivery.log 2>&1 &
CONSUMER_PID=$!
for attempt in $(seq 1 60); do
  if grep -q "Duplicate" /tmp/ensambria-platform-consumer-redelivery.log; then
    break
  fi
  sleep 1
done
grep -q "Duplicate" /tmp/ensambria-platform-consumer-redelivery.log || {
  echo "PLATFORM did not report the duplicate Inbox outcome after JetStream redelivery" >&2
  cat /tmp/ensambria-platform-consumer-redelivery.log >&2 || true
  exit 1
}
grep -q "PLATFORM tenant event committed and ACKed" /tmp/ensambria-platform-consumer-redelivery.log

final_inbox="$(psql "$PLATFORM_VERIFY_DATABASE_URL" -Atqc "SELECT count(*) FROM platform.inbox_messages WHERE event_id='$event_id' AND status='applied'")"
final_onboarding="$(psql "$PLATFORM_VERIFY_DATABASE_URL" -Atqc "SELECT count(*) FROM platform.tenant_onboardings WHERE tenant_id='$tenant_id' AND product_id='018f0f2a-7b3c-7abc-8def-0123456789ab' AND source_event_id='$event_id' AND status='onboarding_pending'")"
final_outbox="$(psql "$CORE_OUTBOX_DATABASE_URL" -Atqc "SELECT count(*) FROM core.outbox_events WHERE event_id='$event_id' AND status='published'")"
[[ "$final_inbox" == "1" && "$final_onboarding" == "1" && "$final_outbox" == "1" ]]

echo "PASS: Outbox pending recovery; relay crash after JetStream PubAck; same event republished and broker-deduplicated; three-node JetStream restart; consumer crash before Inbox transaction; consumer crash after Inbox commit but before ACK; real JetStream redelivery; Inbox duplicate handling; exactly one onboarding_pending projection."
