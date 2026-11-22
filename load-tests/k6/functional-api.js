import http from 'k6/http';
import { check } from 'k6';
import { Rate, Trend } from 'k6/metrics';

const failures = new Rate('functional_request_failures');
const latency = new Trend('functional_request_latency', true);

function required(name) {
  const value = __ENV[name];
  if (!value || value.trim() === '') {
    throw new Error(`${name} must be explicitly approved and provided`);
  }
  return value;
}

function positiveNumber(name) {
  const value = Number(required(name));
  if (!Number.isFinite(value) || value <= 0) {
    throw new Error(`${name} must be a positive number`);
  }
  return value;
}

const target = required('TARGET_URL');
const parsed = new URL(target);
if (parsed.protocol !== 'https:') {
  throw new Error('TARGET_URL must use HTTPS');
}
if (['/health', '/ready', '/live', '/metrics'].includes(parsed.pathname.replace(/\/$/, ''))) {
  throw new Error('Health/metrics endpoints are not valid business load-test targets');
}

const method = (__ENV.METHOD || 'GET').toUpperCase();
if (method !== 'GET') {
  throw new Error('Only read-only GET requests are supported by this generic harness');
}
const bearerToken = required('BEARER_TOKEN');
const rate = positiveNumber('ARRIVAL_RATE');
const preAllocatedVUs = positiveNumber('PREALLOCATED_VUS');
const maxVUs = positiveNumber('MAX_VUS');
const p95Threshold = positiveNumber('P95_THRESHOLD_MS');
const p99Threshold = positiveNumber('P99_THRESHOLD_MS');
const errorRateThreshold = positiveNumber('ERROR_RATE_THRESHOLD');
const duration = required('DURATION');

if (!Number.isInteger(rate) || !Number.isInteger(preAllocatedVUs) ||
    !Number.isInteger(maxVUs) || maxVUs < preAllocatedVUs) {
  throw new Error('ARRIVAL_RATE, PREALLOCATED_VUS, and MAX_VUS must be valid integers; MAX_VUS must be >= PREALLOCATED_VUS');
}
if (errorRateThreshold >= 1) {
  throw new Error('ERROR_RATE_THRESHOLD must be a fraction between 0 and 1');
}
if (!/^\d+(\.\d+)?(ms|s|m|h)$/.test(duration)) {
  throw new Error('DURATION must be an explicit k6 duration such as 30s or 5m');
}

let headers = { Accept: 'application/json', Authorization: `Bearer ${bearerToken}` };
if (__ENV.HEADERS_JSON) {
  headers = { ...headers, ...JSON.parse(__ENV.HEADERS_JSON), Authorization: `Bearer ${bearerToken}` };
}

export const options = {
  scenarios: {
    functional_api: {
      executor: 'constant-arrival-rate',
      rate,
      timeUnit: '1s',
      duration,
      preAllocatedVUs,
      maxVUs,
      gracefulStop: '30s',
    },
  },
  thresholds: {
    http_req_failed: [`rate<${errorRateThreshold}`],
    http_req_duration: [`p(95)<${p95Threshold}`, `p(99)<${p99Threshold}`],
    functional_request_failures: [`rate<${errorRateThreshold}`],
    functional_request_latency: [`p(95)<${p95Threshold}`, `p(99)<${p99Threshold}`],
  },
};

export default function () {
  const response = http.get(target, {
    headers,
    tags: { endpoint: parsed.pathname, method },
    timeout: __ENV.REQUEST_TIMEOUT || '10s',
  });
  const ok = check(response, {
    'functional endpoint returns success': (res) => res.status >= 200 && res.status < 300,
  });
  failures.add(!ok);
  latency.add(response.timings.duration);
}
