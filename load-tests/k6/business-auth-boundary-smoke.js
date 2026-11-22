import http from 'k6/http';
import { check } from 'k6';
import { Rate } from 'k6/metrics';

const unexpectedStatus = new Rate('business_auth_boundary_unexpected_status');
const target = __ENV.TARGET_URL || 'http://127.0.0.1:18080/v1/business/establishments';

const parsed = new URL(target);
if (
  parsed.protocol !== 'http:' ||
  !['127.0.0.1', 'localhost'].includes(parsed.hostname) ||
  parsed.pathname !== '/v1/business/establishments' ||
  parsed.search !== '' ||
  parsed.hash !== ''
) {
  throw new Error('The CI smoke target must be the local BUSINESS establishments route');
}

http.setResponseCallback(http.expectedStatuses(401));

export const options = {
  vus: 1,
  iterations: 1,
  thresholds: {
    business_auth_boundary_unexpected_status: ['rate==0'],
  },
};

export default function () {
  const response = http.get(target, {
    headers: { Accept: 'application/json' },
    tags: { component: 'business', check: 'authentication-boundary' },
    timeout: '5s',
  });

  const expected = check(response, {
    'protected BUSINESS route rejects an unauthenticated request': (res) => res.status === 401,
  });
  unexpectedStatus.add(!expected);
}
