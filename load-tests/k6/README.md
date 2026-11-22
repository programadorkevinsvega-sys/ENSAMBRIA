# Functional API load tests (k6)

This harness intentionally requires an explicit application endpoint. It refuses health, readiness, liveness, and metrics paths so that infrastructure probes cannot be mistaken for business-capacity evidence.

## Run

```sh
k6 run load-tests/k6/functional-api.js \
  -e TARGET_URL='https://staging.example.test/<real-functional-route>' \
  -e METHOD=POST \
  -e BODY_JSON='{"example":"replace-with-real-contract"}' \
  -e BEARER_TOKEN="$ENSAMBRIA_STAGING_TEST_TOKEN" \
  -e ARRIVAL_RATE=100 \
  -e PREALLOCATED_VUS=100 \
  -e MAX_VUS=1000 \
  -e DURATION=5m
```

Optional `HEADERS_JSON` supplies additional JSON headers. Use dedicated synthetic tenants/accounts and non-production data. Never put credentials directly in source control or command history; inject them through the CI secret manager.

The default gates are fewer than 1% failed requests, p95 below 500 ms, and p99 below 1 s. Tune them to the service SLO and environment before a capacity run. Record the tested commit, endpoint contract, arrival rate, achieved throughput, p95/p99, error rate, CPU/memory, PostgreSQL pool saturation, Redis connections/latency, and replica count.

## Current limitation

CORE currently exposes health checks but does not yet expose a functional authentication HTTP/gRPC API runtime, and the BUSINESS runtime is not present in this branch. Therefore this harness is not wired into CI and cannot honestly produce an ENSAMBRIA business-capacity result yet. Add the real route contract and staging secrets before executing it; do not substitute `/health`.
