// k6 load test, run by ./performance_test.sh (docker-compose-performance.yml).
//
// Built on the shared OpenAPI coverage module (mairie360/CICD `tests/k6/coverage.js`, MAIR-194):
// every operation of the spec served by the API needs exactly one handler ("METHOD /path", path
// as in openapi.json), k6 aborts at init otherwise. When you add an endpoint, add its handler to
// `readHandlers` (GET) or `writeHandlers` (any other method) and send its request through
// `request()` (raw `http.*` calls are not counted).
//
// High load on a volume seed (MAIR-474): the performance stack also runs init-perf.sql (2 000
// agents, 50 000 events over 2026-2027 with 3 members each, 2 % public, 500 private weekly recurrences).
// Three scenarios:
// - `reads`: the GET operations, ramping up to 100 VUs, as a random seeded agent: their calendar
//   over a random month (a whole year one call in ten), one of their events and its members;
// - `writes`: every other operation with 10 VUs. Each handler is self-contained: it creates what it
//   needs through `fixture()`, sends its request, then deletes what it created, so the handlers
//   do not depend on their order and the database ends as it started;
// - `calendar_rush`: `GET /calendar` over a month as agents at a fixed arrival rate, failing if k6
//   has to drop iterations (the API no longer keeps up).
import http from 'k6/http';
import crypto from 'k6/crypto';
import encoding from 'k6/encoding';
import { check, fail, sleep } from 'k6';
import { createCoverage, loadSpec } from '/coverage.js';

const BASE_URL = (__ENV.BASE_URL || 'http://localhost:3002').replace(/\/+$/, '');

// The stack's JWT_SECRET, random per run (performance_test.sh, MAIR-428): no committed default.
const JWT_SECRET = __ENV.JWT_SECRET;
if (!JWT_SECRET) {
  throw new Error('JWT_SECRET is not set: run ./performance_test.sh, which generates it');
}
/** Token lifetime: the whole run, setup and teardown included. */
const JWT_TTL_SECONDS = 2 * 60 * 60;

/** HS256 JWT for a user seeded by liquibase / init-test.sql, valid for the run. */
function jwt(sub, role) {
  const header = encoding.b64encode(JSON.stringify({ alg: 'HS256', typ: 'JWT' }), 'rawurl');
  const payload = encoding.b64encode(JSON.stringify({ sub: String(sub), role, exp: Math.floor(Date.now() / 1000) + JWT_TTL_SECONDS }), 'rawurl');
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

// Rows of init-perf.sql: event 100000 + g belongs to agent 300001 + g % 2000.
const AGENTS = { first: 300001, count: 2000 };
const PERF_EVENTS = { first: 100000, count: 50000 };
const SEED_MONTHS = 24; // 2026 and 2027

// Fixed-rate `GET /calendar` as agents.
const CALENDAR_RUSH_RATE = 100; // requests per second
const CALENDAR_RUSH_BUDGET_MS = 200;

const randomInt = (max) => Math.floor(Math.random() * max);

const agentTokens = {};

/** A random seeded agent: its `Authorization` header and one of its events. */
function randomAgent() {
  const rank = randomInt(AGENTS.count);
  const id = AGENTS.first + rank;
  if (!agentTokens[id]) agentTokens[id] = { Authorization: `Bearer ${jwt(id, 'user')}` };
  const eventsPerAgent = PERF_EVENTS.count / AGENTS.count;
  return { headers: agentTokens[id], eventId: PERF_EVENTS.first + rank + AGENTS.count * randomInt(eventsPerAgent) };
}

/** A random month of the seed, or (one call in ten) the whole year that starts with it. */
function randomWindow() {
  const month = randomInt(SEED_MONTHS);
  const start = new Date(Date.UTC(2026, month, 1));
  const end = randomInt(10) === 0 ? new Date(Date.UTC(2026, month + 12, 1) - 1000) : new Date(Date.UTC(2026, month + 1, 1) - 1000);
  return { start: start.toISOString(), end: end.toISOString() };
}

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

/** Event created by `auth`, with `memberIds` assigned (the creator can only edit it once assigned). */
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
  'GET /ready': ({ request }) => check(request(), { 'ready 200': (r) => r.status === 200 }),
  'GET /api/v1/calendar': ({ request }) =>
    check(request({ query: randomWindow(), headers: randomAgent().headers }), {
      'calendar 200': (r) => r.status === 200,
      // About 40 public events a month in the seed: an empty calendar means it was not loaded.
      'calendar reads the seed': (r) => r.status === 200 && r.json('events').length > 0,
    }),
  'GET /api/v1/events/{event_id}/': ({ request }) => {
    const agent = randomAgent();
    check(request({ path: { event_id: agent.eventId }, headers: agent.headers }), {
      'get event 200': (r) => r.status === 200,
      'get event reads the seeded event': (r) => r.status === 200 && r.json('id') === agent.eventId,
    });
  },
  'GET /api/v1/events/{event_id}/members/': ({ request }) => {
    const agent = randomAgent();
    check(request({ path: { event_id: agent.eventId }, headers: agent.headers }), {
      'list members 200': (r) => r.status === 200,
      'list members reads the seeded members': (r) => r.status === 200 && r.json('members').length >= 3,
    });
  },
};

const writeHandlers = {
  // Events: create → patch → delete.
  'POST /api/v1/events/': ({ request }) => {
    const res = request({ body: eventBody('k6 create event') });
    check(res, { 'create event 201': (r) => r.status === 201 });
    if (res.status === 201) deleteEvent(res.json('event_id'));
  },
  'PATCH /api/v1/events/{event_id}/': ({ request }) => {
    const eventId = createEvent('k6 patch event', [1]);
    check(request({
      path: { event_id: eventId },
      body: {
        events_start_time: '2026-10-05T19:00:00Z',
        events_end_time: '2026-10-05T21:00:00Z',
        location: 'Salle des mariages',
      },
    }), {
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
        { duration: '30s', target: 50 },
        { duration: '30s', target: 100 },
        { duration: '2m', target: 100 }, // Hold
        { duration: '20s', target: 0 },
      ],
    },
    writes: {
      executor: 'constant-vus',
      exec: 'writeScenario',
      vus: 10,
      duration: '3m20s',
    },
    calendar_rush: {
      executor: 'constant-arrival-rate',
      exec: 'calendarRushScenario',
      startTime: '1m', // once the reads are at full load
      rate: CALENDAR_RUSH_RATE,
      timeUnit: '1s',
      duration: '1m',
      preAllocatedVUs: 50,
      maxVUs: 200,
    },
  },
  thresholds: {
    ...reads.thresholds, // every operation exercised, no handler error (shared counters)
    ...latencyThresholds(reads, READ_BUDGET_MS),
    ...latencyThresholds(writes, WRITE_BUDGET_MS),
    'http_req_duration{op:calendar_rush}': [`p(95)<${CALENDAR_RUSH_BUDGET_MS}`],
    dropped_iterations: ['count==0'], // the rush kept its rate
    // Strict (MAIR-474): one wrong status or one missing seeded row fails the run.
    checks: ['rate==1'],
    http_req_failed: ['rate==0'],
  },
};

export function readScenario() {
  reads.run({ headers: ADMIN });
  sleep(1);
}

export function calendarRushScenario() {
  const month = randomInt(SEED_MONTHS);
  const res = http.get(
    `${BASE_URL}/api/v1/calendar?start=${new Date(Date.UTC(2026, month, 1)).toISOString()}` +
      `&end=${new Date(Date.UTC(2026, month + 1, 1) - 1000).toISOString()}`,
    { headers: randomAgent().headers, tags: { op: 'calendar_rush' } },
  );
  check(res, {
    'calendar rush 200': (r) => r.status === 200,
    'calendar rush reads the seed': (r) => r.status === 200 && r.json('events').length > 0,
  });
}

export function writeScenario(data) {
  writes.run({ headers: ADMIN, data });
  sleep(1);
}
