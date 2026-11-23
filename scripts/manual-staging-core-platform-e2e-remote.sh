#!/usr/bin/env bash
# Runs only when explicitly invoked by manual-staging-core-platform-e2e.sh.
set -Eeuo pipefail
umask 077

fail() { printf 'ERROR: %s\n' "$*" >&2; exit 1; }
[[ $# -eq 2 ]] || fail "usage: $0 <uploaded-source-dir> <remote-env-file>"
readonly source_dir="$1"
readonly env_file="$2"
[[ -d "$source_dir" && ! -L "$source_dir" ]] || fail "invalid uploaded source directory"
[[ -r "$env_file" && -f "$env_file" ]] || fail "remote E2E env file is not readable"
[[ -f "$source_dir/Cargo.toml" ]] || fail "uploaded source snapshot is incomplete"

actual_hostname="$(hostname -f 2>/dev/null || hostname)"
if [[ -n "${ENSAMBRIA_STAGING_EXPECTED_HOSTNAME:-}" && "$actual_hostname" != "$ENSAMBRIA_STAGING_EXPECTED_HOSTNAME" ]]; then
  fail "remote hostname mismatch (expected=$ENSAMBRIA_STAGING_EXPECTED_HOSTNAME actual=$actual_hostname)"
fi

# The env file is an operator-managed, shell-compatible staging-only file.
set -a
# shellcheck disable=SC1090
source "$env_file"
set +a
[[ "${ENSAMBRIA_ENVIRONMENT:-}" == "staging" ]] || fail "ENSAMBRIA_ENVIRONMENT must equal staging"
for name in CORE_TENANT_DATABASE_URL CORE_OUTBOX_DATABASE_URL PLATFORM_EVENT_DATABASE_URL PLATFORM_VERIFY_DATABASE_URL ENSAMBRIA_NATS_URL; do
  [[ -n "${!name:-}" ]] || fail "required variable $name is missing"
done
[[ "${ENSAMBRIA_ENABLE_FAULT_INJECTION:-0}" != "1" ]] || fail "do not pre-enable fault injection in the env file"
export RUST_LOG="${RUST_LOG:-info}"
export ENSAMBRIA_ENABLE_FAULT_INJECTION=1

command -v cargo >/dev/null || fail "cargo is required on the staging VM to build the debug-only fault-injection binaries"
command -v psql >/dev/null || fail "psql is required on the staging VM"
command -v timeout >/dev/null || fail "timeout is required on the staging VM"
command -v nats >/dev/null || fail "NATS CLI is required to isolate this run from historical JetStream messages"
command -v jq >/dev/null || fail "jq is required to inspect JetStream stream metadata"
: "${ENSAMBRIA_E2E_PRODUCT_ID:?Set ENSAMBRIA_E2E_PRODUCT_ID to an active staging product UUID}"

run_dir="$source_dir/.manual-staging-e2e"
mkdir -m 700 -p "$run_dir"
report="$run_dir/report.txt"
exec > >(tee -a "$report") 2>&1
relay_pid=""
consumer_pid=""
cleanup() {
  local status=$?
  trap - EXIT INT TERM
  for pid in "$relay_pid" "$consumer_pid"; do
    if [[ -n "$pid" ]] && kill -0 "$pid" 2>/dev/null; then
      kill "$pid" 2>/dev/null || true
      wait "$pid" 2>/dev/null || true
    fi
  done
  printf '\nRESULT=%s\nHOST=%s\nUTC_FINISHED=%s\n' "$([[ $status -eq 0 ]] && echo PASS || echo FAIL)" "$actual_hostname" "$(date -u +%FT%TZ)" | tee -a "$report"
  printf 'Report: %s\n' "$report"
  exit "$status"
}
trap cleanup EXIT INT TERM

cd "$source_dir"
printf 'ENSAMBRIA manual CORE -> PLATFORM staging E2E\nHOST=%s\nUTC_STARTED=%s\n' "$actual_hostname" "$(date -u +%FT%TZ)"
printf 'Guard: environment=staging; no systemd service, NATS node, Redis, or PostgreSQL service will be restarted.\n'

# Verify each configured connection without printing connection strings.
for var in CORE_TENANT_DATABASE_URL CORE_OUTBOX_DATABASE_URL PLATFORM_EVENT_DATABASE_URL PLATFORM_VERIFY_DATABASE_URL; do
  url="${!var}"
  psql "$url" -X -v ON_ERROR_STOP=1 -Atqc 'SELECT 1' >/dev/null || fail "$var connection failed"
  printf 'PREFLIGHT_%s=ok\n' "$var"
done
for url in "$CORE_TENANT_DATABASE_URL" "$CORE_OUTBOX_DATABASE_URL" "$PLATFORM_EVENT_DATABASE_URL" "$PLATFORM_VERIFY_DATABASE_URL"; do
  dbname="$(psql "$url" -X -Atqc 'SELECT current_database()')"
  [[ "$dbname" != *prod* && "$dbname" != *production* ]] || fail "database name looks production-like; refusing to continue"
done

printf 'BUILD=debug fault-injection binaries\n'
cargo build --locked -p ensambria-core --bin core-tenant-onboarding-e2e --bin core-outbox-relay
cargo build --locked -p ensambria-platform --bin platform-tenant-onboarding-consumer
sql_scalar() {
  local url="$1" query="$2"
  psql "$url" -X -v ON_ERROR_STOP=1 -Atqc "$query"
}
wait_value() {
  local url="$1" query="$2" expected="$3" label="$4" actual=""
  for _ in $(seq 1 90); do
    actual="$(sql_scalar "$url" "$query")"
    [[ "$actual" == "$expected" ]] && { printf '%s=%s\n' "$label" "$actual"; return 0; }
    sleep 1
  done
  printf 'Timed out: %s expected=%s actual=%s\n' "$label" "$expected" "$actual" >&2
  return 1
}
wait_log() {
  local file="$1" needle="$2" pid="$3"
  for _ in $(seq 1 90); do
    grep -Fq "$needle" "$file" && return 0
    kill -0 "$pid" 2>/dev/null || { cat "$file" >&2; return 1; }
    sleep 1
  done
  cat "$file" >&2
  return 1
}
stop_pid() {
  local pid="$1"
  [[ -n "$pid" ]] || return 0
  kill "$pid" 2>/dev/null || true
  wait "$pid" 2>/dev/null || true
}

# Do not let this temporary relay drain unrelated production-like staging backlog.
if pgrep -af '[c]ore-outbox-relay' >/dev/null; then
  fail "a CORE Outbox relay process is already running; refusing to compete with it"
fi
pending_before="$(sql_scalar "$CORE_OUTBOX_DATABASE_URL" "SELECT count(*) FROM core.outbox_events WHERE status IN ('pending','retry_wait') OR (status='leased' AND lease_until < clock_timestamp())")"
[[ "$pending_before" == 0 ]] || fail "CORE Outbox has $pending_before pre-existing pending/expired events; drain and inspect them before the manual run"
product_active="$(sql_scalar "$PLATFORM_VERIFY_DATABASE_URL" "SELECT EXISTS (SELECT 1 FROM platform.active_product_catalog WHERE product_id='$ENSAMBRIA_E2E_PRODUCT_ID' AND configuration_version=1)")"
[[ "$product_active" == t ]] || fail "ENSAMBRIA_E2E_PRODUCT_ID is not active at configuration version 1 in staging PLATFORM"

# Create a dedicated durable consumer starting after all existing stream messages.
stream_info="$(nats --server "$ENSAMBRIA_NATS_URL" stream info ENSAMBRIA --json)"
last_sequence="$(jq -er '.state.last_seq' <<<"$stream_info")"
[[ "$last_sequence" =~ ^[0-9]+$ ]] || fail "could not read JetStream last sequence"
start_sequence=$((last_sequence + 1))
durable="manual-staging-e2e-${$}-${RANDOM}"
export PLATFORM_TENANT_ONBOARDING_DURABLE="$durable"
export PLATFORM_TENANT_ONBOARDING_START_SEQUENCE="$start_sequence"
printf 'JETSTREAM_STREAM=ENSAMBRIA\\nJETSTREAM_START_SEQUENCE=%s\\nJETSTREAM_DURABLE=%s\\n' "$start_sequence" "$durable"

e2e_output="$(target/debug/core-tenant-onboarding-e2e)"
printf '%s\n' "$e2e_output"
event_id="$(sed -n 's/^EVENT_ID=//p' <<<"$e2e_output")"
tenant_id="$(sed -n 's/^TENANT_ID=//p' <<<"$e2e_output")"
[[ "$event_id" =~ ^[0-9a-fA-F-]{36}$ && "$tenant_id" =~ ^[0-9a-fA-F-]{36}$ ]] || fail "E2E binary did not return valid IDs"
printf 'CORRELATION_EVENT_ID=%s\nTENANT_ID=%s\n' "$event_id" "$tenant_id"


outbox="SELECT status FROM core.outbox_events WHERE event_id='$event_id'"
wait_value "$CORE_OUTBOX_DATABASE_URL" "$outbox" pending outbox_initial_pending

# Relay failure after broker PubAck, then lease expiry and relay restart.
ENSAMBRIA_TEST_PAUSE_AFTER_PUBACK_EVENT_ID="$event_id" \
CORE_OUTBOX_LEASE_SECONDS=3 target/debug/core-outbox-relay >"$run_dir/relay-before-crash.log" 2>&1 &
relay_pid=$!
wait_log "$run_dir/relay-before-crash.log" 'FAULT_INJECTION: paused after JetStream PubAck' "$relay_pid"
stop_pid "$relay_pid"; relay_pid=""
sleep 4
target/debug/core-outbox-relay >"$run_dir/relay-restarted.log" 2>&1 &
relay_pid=$!
wait_value "$CORE_OUTBOX_DATABASE_URL" "$outbox" published outbox_after_relay_recovery
stop_pid "$relay_pid"; relay_pid=""

# Kill PLATFORM before its Inbox transaction; the durable consumer must redeliver.
ENSAMBRIA_TEST_PAUSE_BEFORE_APPLY_EVENT_ID="$event_id" \
PLATFORM_TENANT_ONBOARDING_DURABLE="$durable" PLATFORM_TENANT_ONBOARDING_START_SEQUENCE="$start_sequence" PLATFORM_JETSTREAM_ACK_WAIT_SECONDS=2 target/debug/platform-tenant-onboarding-consumer >"$run_dir/consumer-before-inbox.log" 2>&1 &
consumer_pid=$!
wait_log "$run_dir/consumer-before-inbox.log" 'FAULT_INJECTION: paused after JetStream delivery and before Inbox transaction' "$consumer_pid"
stop_pid "$consumer_pid"; consumer_pid=""
inbox_zero="$(sql_scalar "$PLATFORM_VERIFY_DATABASE_URL" "SELECT count(*) FROM platform.inbox_messages WHERE event_id='$event_id'")"
onboarding_zero="$(sql_scalar "$PLATFORM_VERIFY_DATABASE_URL" "SELECT count(*) FROM platform.tenant_onboardings WHERE tenant_id='$tenant_id'")"
[[ "$inbox_zero" == 0 && "$onboarding_zero" == 0 ]] || fail "unexpected PLATFORM rows before Inbox commit"

# Redelivery applies Inbox + onboarding atomically, then process is killed before ACK.
ENSAMBRIA_TEST_PAUSE_AFTER_COMMIT_EVENT_ID="$event_id" \
PLATFORM_TENANT_ONBOARDING_DURABLE="$durable" PLATFORM_TENANT_ONBOARDING_START_SEQUENCE="$start_sequence" PLATFORM_JETSTREAM_ACK_WAIT_SECONDS=2 target/debug/platform-tenant-onboarding-consumer >"$run_dir/consumer-after-commit.log" 2>&1 &
consumer_pid=$!
wait_log "$run_dir/consumer-after-commit.log" 'FAULT_INJECTION: paused after Inbox/onboarding commit and before JetStream ACK' "$consumer_pid"
stop_pid "$consumer_pid"; consumer_pid=""
inbox_one="$(sql_scalar "$PLATFORM_VERIFY_DATABASE_URL" "SELECT count(*) FROM platform.inbox_messages WHERE event_id='$event_id' AND status='applied'")"
onboarding_one="$(sql_scalar "$PLATFORM_VERIFY_DATABASE_URL" "SELECT count(*) FROM platform.tenant_onboardings WHERE tenant_id='$tenant_id' AND source_event_id='$event_id' AND status='onboarding_pending'")"
[[ "$inbox_one" == 1 && "$onboarding_one" == 1 ]] || fail "Inbox/onboarding transaction did not produce exactly one row"

# Final redelivery must take the duplicate path and ACK without duplicating onboarding.
PLATFORM_TENANT_ONBOARDING_DURABLE="$durable" PLATFORM_TENANT_ONBOARDING_START_SEQUENCE="$start_sequence" PLATFORM_JETSTREAM_ACK_WAIT_SECONDS=2 target/debug/platform-tenant-onboarding-consumer >"$run_dir/consumer-final-redelivery.log" 2>&1 &
consumer_pid=$!
wait_log "$run_dir/consumer-final-redelivery.log" 'Duplicate' "$consumer_pid"
grep -Fq 'PLATFORM tenant event committed and ACKed' "$run_dir/consumer-final-redelivery.log" || fail "duplicate delivery was not ACKed"
stop_pid "$consumer_pid"; consumer_pid=""

final_inbox="$(sql_scalar "$PLATFORM_VERIFY_DATABASE_URL" "SELECT count(*) FROM platform.inbox_messages WHERE event_id='$event_id' AND status='applied'")"
final_onboarding="$(sql_scalar "$PLATFORM_VERIFY_DATABASE_URL" "SELECT count(*) FROM platform.tenant_onboardings WHERE tenant_id='$tenant_id' AND product_id='018f0f2a-7b3c-7abc-8def-0123456789ab' AND source_event_id='$event_id' AND status='onboarding_pending'")"
final_outbox="$(sql_scalar "$CORE_OUTBOX_DATABASE_URL" "SELECT count(*) FROM core.outbox_events WHERE event_id='$event_id' AND status='published'")"
[[ "$final_inbox" == 1 && "$final_onboarding" == 1 && "$final_outbox" == 1 ]] || fail "final exactly-once assertions failed"
printf 'ASSERTIONS=outbox_published:1,inbox_applied:1,onboarding_pending:1\n'
printf 'FAULTS=relay_crash_after_puback,consumer_crash_before_inbox,consumer_crash_after_commit_before_ack,jetstream_redelivery\n'
printf 'PASS: staging CORE -> Outbox -> JetStream -> PLATFORM Inbox -> onboarding; recovery without duplicate tenant onboarding.\n'
