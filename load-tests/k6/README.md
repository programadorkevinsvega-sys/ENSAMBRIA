# k6 performance tests

## CI smoke: real BUSINESS authentication boundary

The Redis HA integration job starts the compiled BUSINESS HTTP binary against the
ephemeral GitHub Actions PostgreSQL and Redis topology. For each Redis mode
(Sentinel, Sentinel after failover, Cluster, and Cluster after failover), the
runtime check executes a single local request with k6 and exports its summary
to a GitHub Actions artifact:

```sh
docker run --rm --network host -v "$PWD:/work" -w /work \
  grafana/k6:0.54.0 run --quiet \
  --summary-export=/work/artifacts/k6/business-smoke.json \
  load-tests/k6/business-auth-boundary-smoke.js
```

This is deliberately one unauthenticated, read-only request to
`GET /v1/business/establishments`. It must return HTTP 401. The test verifies
that the actual BUSINESS HTTP router enforces its authentication boundary while
Redis is configured and after failover. It creates no accounts, does not send
email, and does not mutate tenant data.

This is a functional smoke check, **not** a capacity, throughput, load, stress,
spike, soak, or production-readiness result. It has no latency SLO because no
approved performance objective has been supplied.

## Functional API load harness

`functional-api.js` is retained for a future explicitly authorized target.
It supports only authenticated, read-only HTTPS GET requests and rejects
health, readiness, liveness, and metrics endpoints. It has no default workload
or latency/error thresholds. Every profile value must be supplied from an
approved test plan:

```sh
k6 run load-tests/k6/functional-api.js \
  -e TARGET_URL="$APPROVED_HTTPS_ENDPOINT" \
  -e BEARER_TOKEN="$ENSAMBRIA_TEST_TOKEN" \
  -e ARRIVAL_RATE="$APPROVED_ITERATIONS_PER_SECOND" \
  -e PREALLOCATED_VUS="$APPROVED_PREALLOCATED_VUS" \
  -e MAX_VUS="$APPROVED_MAX_VUS" \
  -e DURATION="$APPROVED_DURATION" \
  -e P95_THRESHOLD_MS="$APPROVED_P95_MS" \
  -e P99_THRESHOLD_MS="$APPROVED_P99_MS" \
  -e ERROR_RATE_THRESHOLD="$APPROVED_ERROR_RATE"
```

Do not run it until the target owner authorizes the destination and the
synthetic-data lifecycle, workload, thresholds, duration, observability, and
stop conditions are approved. Inject secrets through the approved secret
manager; never commit or print credentials.

The previous externally hosted staging workflow has been removed because this
repository currently has no deployed VM or authorized external staging target.
No external target is inferred from repository secrets.

## Evidence required before capacity testing

Before enabling load, stress, spike, breakpoint, or soak profiles, record:

- Authorized target and owning environment.
- Business flow and synthetic test-data creation/cleanup procedure.
- Closed/open workload model and justified VUs or arrival rate.
- Approved p50/p95/p99 and error-rate thresholds.
- Duration, ramp profile, maximum load, and stop conditions.
- Application, PostgreSQL pool, Redis, and runner saturation signals.
- Integrity checks, cleanup result, and post-test recovery.

Never treat a successful smoke check as evidence of supported capacity.
