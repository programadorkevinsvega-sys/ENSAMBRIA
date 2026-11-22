import http from 'k6/http';
import { check } from 'k6';
import { Rate, Trend } from 'k6/metrics';

const failures = new Rate('functional_request_failures');
const latency = new Trend('functional_request_latency', true);
const target = __ENV.TARGET_URL || '';
const method = (__ENV.METHOD || 'GET').toUpperCase();
const vus = Number(__ENV.PREALLOCATED_VUS || 100);
const maxVus = Number(__ENV.MAX_VUS || 1000);
const rate = Number(__ENV.ARRIVAL_RATE || 100);
const duration = __ENV.DURATION || '5m';

if (!target) {
  throw new Error('TARGET_URL must be a real functional API endpoint');
}
const parsed = new URL(target);
if (['/health', '/ready', '/live', '/metrics'].includes(parsed.pathname.replace(/\/$/, ''))) {
  throw new Error('Health/metrics endpoints are not valid business load-test targets');
}
if (!['GET', 'POST', 'PUT', 'PATCH', 'DELETE'].includes(method)) {
  throw new Error('METHOD must be GET, POST, PUT, PATCH, or DELETE');
}

let headers = { 'Accept': 'application/json' };
if (__ENV.HEADERS_JSON) {
  headers = { ...headers, ...JSON.parse(__ENV.HEADERS_JSON) };
}
if (__ENV.BEARER_TOKEN) {
  headers.Authorization = `Bearer ${__ENV.BEARER_TOKEN}`;
}
const body = __ENV.BODY_JSON ? JSON.parse(__ENV.BODY_JSON) : null;

export const options = {
  scenarios: {
    functional_api: {
      executor: 'constant-arrival-rate',
      rate,
      timeUnit: '1s',
      duration,
      preAllocatedVUs: vus,
      maxVUs,
      gracefulStop: '30s',
    },
  },
  thresholds: {
    http_req_failed: ['rate<0.01'],
    http_req_duration: ['p(95)<500', 'p(99)<1000'],
    functional_request_failures: ['rate<0.01'],
    functional_request_latency: ['p(95)<500', 'p(99)<1000'],
  },
};

export default function () {
  const response = http.request(method, target, body === null ? null : JSON.stringify(body), {
    headers,
    tags: { endpoint: parsed.pathname, method },
    timeout: __ENV.REQUEST_TIMEOUT || '10s',
  });
  const ok = check(response, {
    'functional endpoint returns success': (res) => res.status >= 200 && res.status < 400,
  });
  failures.add(!ok);
  latency.add(response.timings.duration);
}
