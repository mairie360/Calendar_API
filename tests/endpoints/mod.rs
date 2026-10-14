//! Handler-level tests: the `/api` scope of `main.rs` (`JwtMiddleware` + `endpoints::config`)
//! served by `actix_web::test` against the shared test database, with forged JWTs. They cover the
//! refusals (401, 403, 404, 409) that the query tests cannot see, since the access rules are
//! applied by the handlers.

pub mod access;
pub mod calendar;
pub mod calendar_cache;
pub mod input;
pub mod members;
pub mod rate_limit;
pub mod service;
pub mod telemetry;
pub mod token_refusals;
pub mod transactions;
pub mod validation;

use std::any::Any;
use std::sync::Once;

use actix_web::web;
use chrono::{Duration, Utc};
use mairie360_api_lib::database::db_interface::Database;
use mairie360_api_lib::jwt_manager::generate_jwt;
use mairie360_api_lib::state::AppState;
use mairie360_api_lib::test_setup::queries_setup::get_shared_db;
use mairie360_api_lib::test_setup::redis_setup::start_redis_container;
use serde_json::{json, Value};

use crate::common;

/// Signs a JWT (no session) for `user_id` with the test secret.
pub fn bearer(user_id: u64) -> (&'static str, String) {
    static JWT_ENV: Once = Once::new();
    JWT_ENV.call_once(|| {
        std::env::set_var("JWT_SECRET", "calendar-endpoint-tests-secret-0123456789");
        std::env::set_var("JWT_TIMEOUT", "3600");
    });
    let token = generate_jwt(&user_id.to_string(), "User").expect("failed to sign test JWT");
    ("Authorization", format!("Bearer {token}"))
}

/// App state on the shared database, and a plain client of the same database for fixtures.
pub struct TestContext {
    pub state: web::Data<AppState>,
    pub db: Database,
    /// Redis container of [`Self::with_redis`], kept alive as long as the context.
    _redis: Option<Box<dyn Any + Send>>,
}

impl TestContext {
    /// Context whose Redis is unreachable: the read cache of `GET /calendar` is bypassed (every
    /// page comes from Postgres) and the test tokens carry no session to look up, so the tests
    /// can write fixtures behind the API's back.
    pub async fn new() -> Self {
        let (_container, pg_url) = get_shared_db().await;
        let state = AppState::new("redis://127.0.0.1:1".to_string(), pg_url.to_string()).await;
        Self {
            state: web::Data::new(state),
            db: Database::new(pg_url).await,
            _redis: None,
        }
    }

    /// Context on a fresh Redis container: the read cache is live (MAIR-524 cache tests).
    pub async fn with_redis() -> Self {
        let (_container, pg_url) = get_shared_db().await;
        let (redis, config) = start_redis_container().await;
        let state = AppState::new(config.url.clone(), pg_url.to_string()).await;
        Self {
            state: web::Data::new(state),
            db: Database::new(pg_url).await,
            _redis: Some(Box::new(redis)),
        }
    }

    /// Creates an event of `creator_id` with the given visibility and returns its id.
    pub async fn event(&self, creator_id: u64, visibility: &str) -> u64 {
        let start = Utc::now() + Duration::days(1);
        let mut input =
            common::event_input("Conseil municipal", None, start, start + Duration::hours(2));
        input.visibility = serde_json::from_value(json!(visibility)).expect("visibility");
        common::create_event(&self.db, creator_id, &input).await
    }
}

/// Builds the app exactly like `main.rs` mounts `/api`.
#[macro_export]
macro_rules! init_app {
    ($ctx:expr) => {{
        actix_web::test::init_service(
            actix_web::App::new().app_data($ctx.state.clone()).service(
                actix_web::web::scope("/api")
                    .wrap(mairie360_api_lib::security::JwtMiddleware)
                    .configure(calendar_api::endpoints::config),
            ),
        )
        .await
    }};
}

/// Sends a request and returns its status, whether the handler answered or a middleware (JWT)
/// refused it with an error.
#[macro_export]
macro_rules! status_of {
    ($app:expr, $req:expr) => {{
        match actix_web::test::try_call_service(&$app, $req.to_request()).await {
            Ok(response) => response.status().as_u16(),
            Err(error) => error.as_response_error().status_code().as_u16(),
        }
    }};
}

/// A valid creation body.
pub fn new_event_body(visibility: &str) -> Value {
    let start = Utc::now() + Duration::days(2);
    json!({
        "name": "Permanence des élus",
        "events_start_time": start,
        "events_end_time": start + Duration::hours(1),
        "visibility": visibility,
    })
}
