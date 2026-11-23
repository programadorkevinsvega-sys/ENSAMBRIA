#!/usr/bin/env bash
set -euo pipefail

COMPOSE=(docker compose -f infra/nats/compose.ci.yml)
BOX=(${COMPOSE[@]} exec -T nats-box)
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

for port in 8222 8223 8224; do
  curl --fail --silent "http://127.0.0.1:${port}/healthz?js-enabled-only=true" >/dev/null
done

if ! "${BOX[@]}" nats --server nats://nats-1:4222 stream info ENSAMBRIA >/dev/null 2>&1; then
  "${BOX[@]}" nats --server nats://nats-1:4222 stream add ENSAMBRIA \
    --subjects "ensambria.>" --storage file --replicas 3 --retention limits --defaults
fi

"${BOX[@]}" nats --server nats://nats-1:4222 stream info ENSAMBRIA
"${BOX[@]}" nats --server nats://nats-1:4222 consumer add ENSAMBRIA ENSAMBRIA-CI \
  --filter "ensambria.>" --ack explicit --pull --defaults

"${BOX[@]}" nats --server nats://nats-1:4222 pub ensambria.ci.before-failure "before-failure" --ack
before="$("${BOX[@]}" nats --server nats://nats-1:4222 consumer next ENSAMBRIA ENSAMBRIA-CI --count 1 --ack --timeout 10s)"
grep -q "before-failure" <<<"$before"

# A three-replica stream must retain quorum with one server unavailable.
"${COMPOSE[@]}" stop nats-1
for attempt in $(seq 1 45); do
  if "${BOX[@]}" nats --server nats://nats-2:4222 stream info ENSAMBRIA >/dev/null 2>&1; then
    break
  fi
  sleep 1
done
"${BOX[@]}" nats --server nats://nats-2:4222 pub ensambria.ci.during-failure "during-failure" --ack
"${COMPOSE[@]}" start nats-1

wait_for_stream nats://nats-2:4222
wait_for_stream nats://nats-1:4222
"${BOX[@]}" nats --server nats://nats-2:4222 stream info ENSAMBRIA
after="$("${BOX[@]}" nats --server nats://nats-2:4222 consumer next ENSAMBRIA ENSAMBRIA-CI --count 1 --ack --timeout 10s)"
grep -q "during-failure" <<<"$after"

echo "NATS JetStream three-node cluster, replicated stream, publish acknowledgement, durable consumption and single-node recovery passed."
