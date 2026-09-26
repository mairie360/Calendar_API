// k6 load test, run by ./performance_test.sh (docker-compose-performance.yml).
//
// Built on the shared OpenAPI coverage module (mairie360/CICD `tests/k6/coverage.js`, MAIR-194):
// every operation of the spec served by the API needs exactly one handler ("METHOD /path", path
// as in openapi.json), k6 aborts at init otherwise. When you add an endpoint, add its handler to
// `readHandlers` (GET) or `writeHandlers` (any other method) and send its request through
// `request()` (raw `http.*` calls are not counted).
//
// Two scenarios share the spec, split by HTTP method:
// - `reads`: the GET operations under the historical profile (ramp up to 20 VUs), against the
//   fixtures created once in setup() and removed in teardown();
// - `writes`: every other operation with 2 VUs. Each handler is self-contained: it creates what it
//   needs through `fixture()`, sends its request, then deletes what it created, so the handlers
//   do not depend on their order and the database ends as it started.
import http from 'k6/http';
import crypto from 'k6/crypto';
import encoding from 'k6/encoding';
import { check, fail, sleep } from 'k6';
import { createCoverage, loadSpec } from '/coverage.js';

const BASE_URL = (__ENV.BASE_URL || 'http://localhost:3002').replace(/\/+$/, '');

// Same secret as the stack's JWT_SECRET (the literal string `b"secret"`).
const JWT_SECRET = __ENV.JWT_SECRET || 'b"secret"';

/** HS256 JWT for a user seeded by liquibase / init-test.sql, valid until 2100. */
function jwt(sub, role) {
  const header = encoding.b64encode(JSON.stringify({ alg: 'HS256', typ: 'JWT' }), 'rawurl');
  const payload = encoding.b64encode(JSON.stringify({ sub: String(sub), role, exp: 4102444800 }), 'rawurl');
  return `${header}.${payload}.${crypto.hmac('sha256', JWT_SECRET, `${header}.${payload}`, 'base64rawurl')}`;
}

// User 1 is the Admin seeded by liquibase (the `sub` ZAP uses too); users 2 (User) and
// 3 (Responsable, sharing group 1000 with user 2) come from init-test.sql.
const ADMIN = { Authorization: `Bearer ${jwt(1, 'admin')}` };
const AGENT_ID = 2;
const AGENT = { Authorization: `Bearer ${jwt(AGENT_ID, 'user')}` };
const RESPONSABLE_ID = 3;
const RESPONSABLE = { Authorization: `Bearer ${jwt(RESPONSABLE_ID, 'responsable')}` };

// p(95) latency budget of each family of operations, in ms (reference machine).
const READ_BUDGET_MS = 200;
const WRITE_BUDGET_MS = 500;

const READ_METHODS = ['get', 'head', 'options'];

// Window read by GET /calendar; every fixture event sits inside it.
const WINDOW = { start: '2026-10-01T00:00:00Z', end: '2026-10-31T23:59:59Z' };

/** The served spec restricted to the operations whose method passes `keep`. */
function specSubset(spec, keep) {
  const paths = {};
  for (const path of Object.keys(spec.paths)) {
    const kept = {};
    for (const method of Object.keys(spec.paths[path])) {
      if (keep(method)) kept[method] = spec.paths[path][method];
    }
    if (Object.keys(kept).length > 0) paths[path] = kept;
  }
  return Object.assign({}, spec, { paths });
}

/**
 * Raw call for the fixtures of setup(), teardown() and the write handlers, outside the coverage
 * count. Tagged `op: fixture` so it stays out of the per-operation latency thresholds, but it
 * still counts in `http_req_failed`. Aborts the handler (or setup) on a non-2xx answer.
 */
function fixture(method, path, body, auth = ADMIN) {
  const res = http.request(
    method,
    `${BASE_URL}${path}`,
    body === undefined ? null : JSON.stringify(body),
    { headers: Object.assign({ 'Content-Type': 'application/json' }, auth), tags: { op: 'fixture' } },
  );
  if (res.status < 200 || res.status >= 300) {
    fail(`fixture ${method} ${path} answered ${res.status}: ${res.body}`);
  }
  return res;
}

function eventBody(name) {
  return {
    name,
    description: 'k6 fixture',
    events_start_time: '2026-10-05T18:00:00Z',
    events_end_time: '2026-10-05T20:00:00Z',
    location: 'Salle du conseil',
    category: 'meeting',
  };
}

/** Event created by `auth`, with `memberIds` assigned (the creator only sees it once assigned). */
function createEvent(name, memberIds, auth = ADMIN) {
  const eventId = fixture('POST', '/api/v1/events/', eventBody(name), auth).json('event_id');
  for (const userId of memberIds) {
    fixture('POST', `/api/v1/events/${eventId}/members/`, { user_id: userId }, auth);
  }
  return eventId;
}

function deleteEvent(eventId, auth = ADMIN) {
  fixture('DELETE', `/api/v1/events/${eventId}/`, undefined, auth);
}

const spec = loadSpec();

const readHandlers = {
  'GET /health': ({ request }) => check(request(), { 'health 200': (r) => r.status === 200 }),
  'GET /api/v1/calendar': ({ request }) =>
    check(request({ query: WINDOW }), { 'calendar 200': (r) => r.status === 200 }),
  'GET /api/v1/events/{event_id}/': ({ request, data }) =>
    check(request({ path: { event_id: data.eventId } }), { 'get event 200': (r) => r.status === 200 }),
  'GET /api/v1/events/{event_id}/members/': ({ request, data }) =>
    check(request({ path: { event_id: data.eventId } }), {
      'list members 200': (r) => r.status === 200,
    }),
};

const writeHandlers = {
  'POST /': ({ request }) => check(request(), { 'hello 200': (r) => r.status === 200 }),

  // Events: create → patch → delete.
  'POST /api/v1/events/': ({ request }) => {
    const res = request({ body: eventBody('k6 create event') });
    check(res, { 'create event 201': (r) => r.status === 201 });
    if (res.status === 201) deleteEvent(res.json('event_id'));
  },
  'PATCH /api/v1/events/{event_id}/': ({ request }) => {
    const eventId = createEvent('k6 patch event', [1]);
    check(request({ path: { event_id: eventId }, body: { location: 'Salle des mariages' } }), {
      'patch event 204': (r) => r.status === 204,
    });
    deleteEvent(eventId);
  },
  'DELETE /api/v1/events/{event_id}/': ({ request }) => {
    const eventId = createEvent('k6 delete event', []);
    check(request({ path: { event_id: eventId } }), { 'delete event 204': (r) => r.status === 204 });
  },

  // Participants: add → remove.
  'POST /api/v1/events/{event_id}/members/': ({ request }) => {
    const eventId = createEvent('k6 add member', []);
    check(request({ path: { event_id: eventId }, body: { user_id: AGENT_ID } }), {
      'add member 201': (r) => r.status === 201,
    });
    deleteEvent(eventId);
  },
  'DELETE /api/v1/events/{event_id}/members/{member_id}/': ({ request }) => {
    const eventId = createEvent('k6 remove member', [AGENT_ID]);
    check(request({ path: { event_id: eventId, member_id: AGENT_ID } }), {
      'remove member 204': (r) => r.status === 204,
    });
    deleteEvent(eventId);
  },

  // Validation circuit: user 2 (User) creates an event with the Responsable (user 3, same group)
  // assigned, so it waits for validation; the Responsable approves it.
  'PATCH /api/v1/events/{event_id}/validation': ({ request }) => {
    const eventId = createEvent('k6 validation', [AGENT_ID, RESPONSABLE_ID], AGENT);
    check(request({ path: { event_id: eventId }, body: { status: 'approved' }, headers: RESPONSABLE }), {
      'validate event 204': (r) => r.status === 204,
    });
    deleteEvent(eventId, AGENT);
  },
};

const reads = createCoverage(readHandlers, {
  spec: specSubset(spec, (method) => READ_METHODS.includes(method)),
});
const writes = createCoverage(writeHandlers, {
  spec: specSubset(spec, (method) => !READ_METHODS.includes(method)),
});

/** One `p(95)` threshold per operation (`op` tag) of `coverage`. */
function latencyThresholds(coverage, budgetMs) {
  const thresholds = {};
  for (const operation of coverage.operations) {
    thresholds[`http_req_duration{op:${operation.op}}`] = [`p(95)<${budgetMs}`];
  }
  return thresholds;
}

export const options = {
  scenarios: {
    reads: {
      executor: 'ramping-vus',
      exec: 'readScenario',
      stages: [
        { duration: '30s', target: 20 }, // Ramp up to 20 virtual users
        { duration: '1m', target: 20 }, // Hold
        { duration: '10s', target: 0 }, // Ramp down
      ],
    },
    writes: {
      executor: 'constant-vus',
      exec: 'writeScenario',
      vus: 2,
      duration: '1m40s',
    },
  },
  thresholds: {
    ...reads.thresholds, // every operation exercised, no handler error (shared counters)
    ...latencyThresholds(reads, READ_BUDGET_MS),
    ...latencyThresholds(writes, WRITE_BUDGET_MS),
    http_req_failed: ['rate<0.01'], // Less than 1% errors
  },
};

/** Read fixture: an event of the calendar window with the Admin and user 2 assigned. */
export function setup() {
  return { eventId: createEvent('k6 read fixture', [1, AGENT_ID]) };
}

export function teardown(data) {
  deleteEvent(data.eventId);
}

export function readScenario(data) {
  reads.run({ headers: ADMIN, data });
  sleep(1);
}

export function writeScenario(data) {
  writes.run({ headers: ADMIN, data });
  sleep(1);
}
