#!/usr/bin/env bash
set -euo pipefail

mode="${PLATFORM_REDIS_MODE:?PLATFORM_REDIS_MODE is required}"
profile="${BUSINESS_HTTP_TEST_PROFILE:-$mode}"
port="${BUSINESS_HTTP_TEST_PORT:-18080}"
log_file="${BUSINESS_HTTP_TEST_LOG:-/tmp/ensambria-business-${profile}.log}"
health_url="http://127.0.0.1:${port}/health"
business_url="http://127.0.0.1:${port}/v1/business/establishments"
k6_image="${K6_IMAGE:-grafana/k6:0.54.0}"
summary_file="artifacts/k6/business-${profile}.json"

case "$profile" in
    sentinel-primary|sentinel-after-failover|cluster-primary|cluster-after-failover) ;;
    *) echo "Unsupported BUSINESS smoke profile: $profile" >&2; exit 1 ;;
esac

./target/debug/business >"$log_file" 2>&1 &
pid=$!

cleanup() {
    kill "$pid" 2>/dev/null || true
    wait "$pid" 2>/dev/null || true
}
trap cleanup EXIT

for attempt in $(seq 1 30); do
    if curl --fail --silent --show-error "$health_url" | grep -qx "ok"; then
        echo "BUSINESS runtime started with required Redis mode: $mode ($profile)"
        break
    fi
    if ! kill -0 "$pid" 2>/dev/null; then
        cat "$log_file"
        echo "BUSINESS runtime exited before health became ready" >&2
        exit 1
    fi
    if [ "$attempt" -eq 30 ]; then
        cat "$log_file"
        echo "BUSINESS runtime did not become healthy using Redis mode: $mode" >&2
        exit 1
    fi
    sleep 1
done

# One local, read-only request verifies the real HTTP auth boundary without
# creating users, sending email, mutating tenant data, or generating load.
mkdir -p "$(dirname "$summary_file")"
docker run --rm --network host \
    -v "$PWD:/work" -w /work \
    "$k6_image" run --quiet --summary-export="/work/$summary_file" \
    load-tests/k6/business-auth-boundary-smoke.js \
    -e "TARGET_URL=$business_url"
