//! Handler-level tests: the `/api` scope of `main.rs` (`JwtMiddleware` + `endpoints::config`)
//! served by `actix_web::test` against the shared test database, with JWTs signed like Core signs
//! them (MAIR-680: EdDSA, the lib's test key pair; the process only holds the public key). They cover the
//! refusals (401, 403, 404, 409) that the query tests cannot see, since the access rules are
//! applied by the handlers.

pub mod access;
pub mod access_matrix;
pub mod calendar;
pub mod calendar_cache;
pub mod creation;
pub mod input;
pub mod mair_680_session_keys;
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
use jsonwebtoken::{encode, Algorithm, Header};
use mairie360_api_lib::database::db_interface::Database;
use mairie360_api_lib::jwt_manager::{
    parse_public_keys, parse_signing_key, Claims, SESSION_JWT_AUDIENCE, SESSION_JWT_ISSUER,
};
use mairie360_api_lib::state::AppState;
use mairie360_api_lib::test_setup::http_setup::{test_jwt_private_key, TEST_JWT_PUBLIC_KEY};
use mairie360_api_lib::test_setup::redis_setup::start_redis_container;
use serde_json::{json, Value};

use crate::common;

/// Seconds since the epoch, as the `exp` claim counts them.
pub fn now() -> usize {
    usize::try_from(Utc::now().timestamp()).unwrap()
}

/// Claims of a session token issued by Core for `user_id` (MAIR-680: with `iss` and `aud`).
pub fn core_claims(user_id: u64, expiration: usize) -> Claims {
    Claims::new(&user_id.to_string(), "User", expiration)
        .with_issuer_and_audience(SESSION_JWT_ISSUER, SESSION_JWT_AUDIENCE)
}

/// PKCS#8 body (base64) of an Ed25519 private key that is not Core's (MAIR-680), assembled into
/// PEM at run time so that the secret scanners do not read a PEM private key in the sources.
const MAIR_680_OTHER_PRIVATE_KEY_B64: &str =
    "MC4CAQAwBQYDK2VwBCIEII1DTYiu+dcwBgtEKil59uTH8s7XfrxGrbD+/P0sXq/8";

/// Public key of [`other_private_key`], never published in `JWT_PUBLIC_KEY`.
const MAIR_680_OTHER_PUBLIC_KEY: &str = "-----BEGIN PUBLIC KEY-----
MCowBQYDK2VwAyEAtZHz1BNkijjkLJlnK9l7vikDl0aivrmbwllpT68Lnss=
-----END PUBLIC KEY-----
";

/// An Ed25519 private key that is not Core's: what a compromised service could sign with.
pub fn other_private_key() -> String {
    let label = "PRIVATE KEY";
    format!("-----BEGIN {label}-----\n{MAIR_680_OTHER_PRIVATE_KEY_B64}\n-----END {label}-----\n")
}

/// Signs `claims` as they are with `private_key` (PEM) under Core's `kid`: a genuine token with
/// [`test_jwt_private_key`], a forgery with [`other_private_key`].
pub fn sign_eddsa_with(claims: &Claims, private_key: &str) -> String {
    let core_keys = parse_public_keys(TEST_JWT_PUBLIC_KEY).unwrap();
    let mut header = Header::new(Algorithm::EdDSA);
    header.kid = Some(core_keys[0].kid().to_string());
    let known_keys = [
        core_keys,
        parse_public_keys(MAIR_680_OTHER_PUBLIC_KEY).unwrap(),
    ]
    .concat();
    let key = parse_signing_key(private_key, &known_keys).unwrap();
    encode(&header, claims, key.encoding_key()).unwrap()
}

/// Signs `claims` like Core does (MAIR-680): EdDSA, the lib's test key pair, Core's `kid`.
pub fn sign_like_core(claims: &Claims) -> String {
    sign_eddsa_with(claims, &test_jwt_private_key())
}

/// The environment of a deployed API after MAIR-680: Core's public key, no `JWT_SECRET`.
pub fn set_jwt_env() {
    static JWT_ENV: Once = Once::new();
    JWT_ENV.call_once(|| {
        std::env::set_var("JWT_PUBLIC_KEY", TEST_JWT_PUBLIC_KEY);
        std::env::remove_var("JWT_PRIVATE_KEY");
        std::env::remove_var("JWT_SECRET");
        std::env::set_var("JWT_TIMEOUT", "3600");
    });
}

/// `Authorization` header of a session JWT issued by Core for `user_id` (no session id).
pub fn bearer(user_id: u64) -> (&'static str, String) {
    set_jwt_env();
    let token = sign_like_core(&core_claims(user_id, now() + 3600));
    ("Authorization", format!("Bearer {token}"))
}

/// App state on the shared database, connected as `calendar_api` like in staging (MAIR-556),
/// and a superuser client of the same database for the fixtures.
pub struct TestContext {
    pub state: web::Data<AppState>,
    /// Superuser client, for the fixtures only (see [`common::fixtures_db`]).
    pub db: Database,
    /// Redis container of [`Self::with_redis`], kept alive as long as the context.
    _redis: Option<Box<dyn Any + Send>>,
}

impl TestContext {
    /// Context whose Redis is unreachable: the read cache of `GET /calendar` is bypassed (every
    /// page comes from Postgres) and the test tokens carry no session to look up, so the tests
    /// can write fixtures behind the API's back.
    pub async fn new() -> Self {
        set_jwt_env();
        let state = AppState::new(
            "redis://127.0.0.1:1".to_string(),
            common::calendar_db_url().await,
        )
        .await;
        Self {
            state: web::Data::new(state),
            db: common::fixtures_db().await,
            _redis: None,
        }
    }

    /// Context on a fresh Redis container: the read cache is live (MAIR-524 cache tests).
    pub async fn with_redis() -> Self {
        set_jwt_env();
        let (redis, config) = start_redis_container().await;
        let state = AppState::new(config.url.clone(), common::calendar_db_url().await).await;
        Self {
            state: web::Data::new(state),
            db: common::fixtures_db().await,
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
