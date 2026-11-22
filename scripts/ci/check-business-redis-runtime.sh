#!/usr/bin/env bash
set -euo pipefail

mode="${PLATFORM_REDIS_MODE:?PLATFORM_REDIS_MODE is required}"
port="${BUSINESS_HTTP_TEST_PORT:-18080}"
log_file="${BUSINESS_HTTP_TEST_LOG:-/tmp/ensambria-business-${mode}.log}"
health_url="http://127.0.0.1:${port}/health"

./target/debug/business >"$log_file" 2>&1 &
pid=$!

cleanup() {
    kill "$pid" 2>/dev/null || true
    wait "$pid" 2>/dev/null || true
}
trap cleanup EXIT

for attempt in $(seq 1 30); do
    if curl --fail --silent --show-error "$health_url" | grep -qx "ok"; then
        echo "BUSINESS runtime started with required Redis mode: $mode"
        exit 0
    fi
    if ! kill -0 "$pid" 2>/dev/null; then
        cat "$log_file"
        echo "BUSINESS runtime exited before health became ready" >&2
        exit 1
    fi
    sleep 1
done

cat "$log_file"
echo "BUSINESS runtime did not become healthy using Redis mode: $mode" >&2
exit 1
