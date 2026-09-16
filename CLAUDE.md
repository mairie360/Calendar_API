# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## What this is

`calendar_api` is an actix-web REST API for the **Mairie 360** project (Epitech). It manages
calendar events and their members, backed by PostgreSQL and Redis. It was bootstrapped from a
Rust API template (see `README.md`), so a few files still carry `#change api name` / `#change port`
markers from the template.

Most cross-cutting infrastructure (DB pool, Redis, JWT auth middleware, env-var loading, test
containers) lives in the external crate **`mairie360_api_lib`**, not in this repo. Read its source
under `~/.cargo/registry/src/*/mairie360_api_lib-<version>/` when you need to know how `AppState`,
`JwtMiddleware`, `AuthenticatedUser`, or the DB/cache helpers actually behave.

## Commands

Cargo aliases are defined in `.cargo/config.toml`:

| Command | Purpose |
| --- | --- |
| `cargo lint_check` | `cargo fmt --all -- --check` (CI gate) |
| `cargo lint_fix` | `cargo fmt --all` |
| `cargo check_code` | `cargo clippy --all-targets --all-features -- -D warnings` (CI gate) |
| `cargo open_api` | Regenerate the OpenAPI JSON: `cargo run --example generate_openapi` |
| `cargo cov_test` | `cargo llvm-cov` with a **60% line-coverage gate** (`--fail-under-lines 60`), ignoring `endpoints/`, `main.rs`, `lib.rs` — CI gate |
| `cargo cov` | Same as `cov_test` but also writes `codecov.json` (`--codecov`) |
| `cargo build` / `cargo run` | Build / run the server (needs the env vars below) |

Only the `database/` layer is coverage-gated; `endpoints/` (thin actix glue) and the binary
entrypoints are deliberately excluded, so put testable logic in `database/`.

Tests:

- `cargo test` — runs everything. Integration tests in `tests/` spin up a **throwaway PostgreSQL
  testcontainer** via `mairie360_api_lib::test_setup` (Docker must be available; no local DB needed).
- `cargo test --test integration_test` — only the integration suite.
- `cargo test test_create_event_by_user_success` — a single test by name.
- DB query tests are `#[tokio::test] #[serial]` and call `get_shared_db()` (a process-wide
  `OnceCell` container shared across all tests), then `Database::new(host).await`.

OpenAPI client generation (TypeScript, for consumers): `npx orval` reads `openapi.json` per
`orval.config.js` and writes `generated/`. Regenerate `openapi.json` with `cargo open_api` first.
Both `openapi.*` and `generated/` are gitignored.

`./performance_test.sh` (k6 load test, `docker-compose-performance.yml`) and `./security_test.sh`
(OWASP ZAP scan, `docker-compose-security.yml`) spin up a throwaway stack, run, and tear down.
The referenced compose files are not checked in — these are ops helpers, not part of `cargo test`.

### Running the full stack

`docker compose up` (or `docker compose watch` for live-reload dev). Services: `calendar` (this
API, port 3002), `postgres` (via `ghcr.io/mairie360/database`), `liquibase` (applies DB migrations —
**schema is not defined in this repo**), `seeder` (`init-test.sql`), `redis`, and `nginx`
(reverse proxy at `calendar.development.mairie360.fr`). `development.Dockerfile` runs
`cargo watch`; `Dockerfile` is the release build into a distroless image.

## Required environment variables

`main.rs` reads these via `get_critical_env_var` (the process **panics** if any is missing):
`REDIS_URL`, `DB_USER`, `DB_PASSWORD`, `DB_HOST`, `DB_PORT`, `DB_NAME`, `HOST`, `PORT`.
The Postgres URL is assembled from the `DB_*` parts. `JWT_SECRET` / `JWT_TIMEOUT` are consumed
by `mairie360_api_lib`'s JWT layer. See `docker-compose.yml` `x-common-env` for working values.

## Architecture

### Request routing (`src/main.rs` → `src/endpoints/`)

Three tiers, assembled in `main.rs`:
1. **Public, unauthenticated**: `/health`, `/` (`hello`), `/swagger-ui/*`, `/api-docs/openapi.json`.
2. **`/api` scope wrapped in `JwtMiddleware`** — everything under `endpoints::config` →
   `v1::config`. A valid JWT is required; handlers receive an `AuthenticatedUser` extractor
   exposing `auth_user.id` (the caller's user id).
3. Route tree: `/api/v1/events` (POST create, `/{event_id}/` GET/PATCH/DELETE, `/{event_id}/validation`
   PATCH, `/{event_id}/members/` GET/POST + `/{member_id}/` DELETE), `/api/v1/calendar` (GET, time-range
   query), `/api/v1/params/*` (currently a `501 Not Implemented` catch-all). Published paths are relative to
   `/api`; `tests/routing_test.rs` checks each published operation hits a mounted route.

### Events model and access rules

The schema is the Database **v1.2.0** changeset, shipped in the `1.1.0` release of the
`ghcr.io/mairie360/database` and `liquibase-migrations` images: `events.category` / `service_label` / `location`, and the
repetition of an event in `recurrence_rules` (`events.recurrence_id`, `is_exception = false`; `end_date` is the
day after `ends_on` at 00:00 UTC, NULL when the rule never ends). `src/database/event/model.rs` holds the shared
types (`EventInput`, `EventRecurrence`…) and the SQL fragments reused by several views. Integration tests use the
`TEST_DB_VERSION` pinned in `.cargo/config.toml`, the compose files use the same images.

Every `/{event_id}` operation goes through `endpoints/error.rs::require_event_access` (one
`EventAccessQueryView` query; 404 unknown event, 403 refused): reading needs the caller to be a member; editing
needs a member who is the creator or Responsable/Maire/Admin; deleting is the creator's; managing members is the
creator's or an editor's, and a user can only be assigned inside the caller's scope (Admin/Maire: anyone, others:
themselves and members of their groups); adding or removing a member recomputes the validation (pending when a
User/Guest creator shares a group with an assigned Responsable); `PATCH validation` is reserved to such an
assigned Responsable sharing a group with the creator. `GET /{event_id}/` returns the resulting
`approval_status` and `permissions`.

### Endpoint module convention

Every endpoint is its own directory following the URL path, with up to four files:
- `mod.rs` — declares submodules and an actix `config(cfg)` fn that wires `web::scope(...)`.
- `endpoint.rs` — the handler: a public `#[get/post/...]` + `#[utoipa::path(...)]` function that checks the
  caller's access, validates the input and runs the query views. Errors are the shared
  `endpoints/error.rs::ApiError`, not a per-endpoint enum.
- `view.rs` — request/response DTOs. Structs use private fields + explicit getters, `#[derive(ToSchema)]`
  for OpenAPI, and `TryFrom<web::Json<T>>` / `TryFrom<web::Query<T>>` impls for input validation.
- `doc.rs` — a `#[derive(OpenApi)]` struct listing this endpoint's `paths(...)` and schema
  `components(...)`. These nest upward: `get/doc.rs` → `events/doc.rs` → `v1/doc.rs` →
  `endpoints/swagger.rs::ApiDoc` (the single doc consumed by Swagger UI and `generate_openapi`). utoipa
  *replaces* rather than merges two `nest` entries landing on the same path, so operations sharing a path are
  listed in one document (see `events/doc.rs`), and a nest path keeps the trailing slash of the real route.

When adding an endpoint: create the directory + 4 files, register it in the parent `mod.rs`
`config`, and add its `doc.rs` struct to the parent `doc.rs` nest, or Swagger will not show it.

### Database layer (`src/database/`)

Mirrors `endpoints/` but for persistence. Each operation is a directory holding a single
`view.rs` (there are **no more `query.rs` files** — removed in the 1.2.0 migration). `view.rs`
contains:
- A "query view" struct — a thin wrapper around `params: Vec<QueryParam>`
  (`mairie360_api_lib::database::db_interface::QueryParam`) — implementing
  `ApiRequestDto`: `query_sql()` returns a `&'static str` SQL string (`$1`, `$2`, … placeholders)
  and `query_params()` returns `&self.params`. Keep a `new(...)` constructor with typed args plus
  getters that read back out of `params`. The struct must `#[derive(serde::Deserialize, serde::Serialize)]`
  (`ApiRequestDto: DeserializeOwned`).
- For read queries, a result struct deriving `serde::Deserialize + serde::Serialize` (no
  `sqlx::FromRow`).

Endpoints run these through `state.get_smart_db()` (a `SmartDatabase`, cache-aside over Redis):
- `execute(view)` — writes (`INSERT`/`UPDATE`/`DELETE`), takes the view **by value**, returns
  `Result<(), ApiLibError>` (**no `rows_affected`** — see below).
- `fetch_scalar::<T, _>(&view)` — one scalar column decoded directly by sqlx (`i32`, `bool`, …).
  Used for `RETURNING id` on create and `DELETE … RETURNING …` (0 rows → `DbError::NotFound`).
- `fetch_one::<T, _>(&view)` / `fetch_all::<T, _>(&view)` — the SQL **must return a single JSON
  column** (`SELECT to_jsonb(t) FROM (SELECT …) t`); the lib decodes it to `serde_json::Value`
  then `serde_json::from_value::<T>()`. `fetch_one` on 0 rows → `DbError::NotFound`.

`QueryParam` has no `Option<String>`/`Option<DateTime>` variant — pass `QueryParam::Text(x.unwrap_or_default())`
and wrap the placeholder in `NULLIF($n, '')` in the SQL to store NULL.

Endpoints return the shared `endpoints/error.rs::ApiError` (400/403/404/409/500, text body).

Tests (`tests/queries/`) hit a real Postgres testcontainer: `Database::new(host).await` (from
`get_shared_db()`), then the same `fetch_*`/`execute` methods (on `Database` these take `&view`).
Use `#[tokio::test]` + `#[serial]` (`serial_test`); `tests/common` creates events, users (with roles) and groups.
`tests/model_test.rs` covers input validation and partial updates without a database.

`i32` is the DB id type; the API layer uses `u64` and casts at the boundary (`x as i32`).
`src/database/event/update_user_status` has no endpoint wired up (kept for its tests).

## CI

`.github/workflows/cicd.yml` delegates to the shared `mairie360/CICD` reusable workflow
(fmt check, clippy `-D warnings`, tests, Postman collection run, Docker image publish as
`calendar-api`). Renovate PRs are auto-approved (`.github/workflows/auto-approve.yml`);
`renovate.json` extends `github>mairie360/renovace`.
