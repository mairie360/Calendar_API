//! MAIR-680: Core is the only signer of the session JWTs. This API holds Core's public key
//! (`JWT_PUBLIC_KEY`) and no `JWT_SECRET`: it accepts the `EdDSA` tokens signed with Core's key and
//! refuses every HS256 token, whatever secret signed it, so a service that leaks its configuration
//! can no longer forge an admin token for this API.

use actix_web::test::{call_and_read_body_json, TestRequest};
use chrono::{Duration, Utc};
use jsonwebtoken::{encode, EncodingKey, Header};
use mairie360_api_lib::jwt_manager::Claims;
use serde_json::Value;
use serial_test::serial;

use super::{bearer, new_event_body, now, TestContext};
use crate::common::create_user;
use crate::{init_app, status_of};

/// HS256 token of `user_id` signed with `secret`: what any holder of the old shared secret signs.
fn hs256(user_id: u64, secret: &[u8]) -> (&'static str, String) {
    let token = encode(
        &Header::default(),
        &Claims::new(&user_id.to_string(), "Admin", now() + 3600),
        &EncodingKey::from_secret(secret),
    )
    .unwrap();
    ("Authorization", format!("Bearer {token}"))
}

fn calendar_uri() -> String {
    let start = Utc::now();
    format!(
        "/api/v1/calendar?start={}&end={}",
        start.to_rfc3339_opts(chrono::SecondsFormat::Secs, true),
        (start + Duration::days(7)).to_rfc3339_opts(chrono::SecondsFormat::Secs, true)
    )
}

#[tokio::test]
#[serial]
async fn mair_680_runs_without_jwt_secret() {
    let _ctx = TestContext::new().await;
    assert!(std::env::var_os("JWT_SECRET").is_none());
    assert!(std::env::var_os("JWT_PRIVATE_KEY").is_none());
    assert!(std::env::var_os("JWT_PUBLIC_KEY").is_some());
}

#[tokio::test]
#[serial]
async fn mair_680_eddsa_token_signed_with_core_key_is_accepted() {
    let ctx = TestContext::new().await;
    let user = create_user(&ctx.db, "Eddsa", Some("User")).await;
    let app = init_app!(ctx);

    let request = TestRequest::get()
        .uri(&calendar_uri())
        .insert_header(bearer(user));
    assert_eq!(status_of!(app, request), 200);
}

#[tokio::test]
#[serial]
async fn mair_680_hs256_token_is_refused_without_jwt_secret() {
    let ctx = TestContext::new().await;
    let admin = create_user(&ctx.db, "Hs256", Some("Admin")).await;
    let app = init_app!(ctx);

    // The secrets the stacks and the test suites used to share, and any other one.
    for secret in [
        b"calendar-endpoint-tests-secret-0123456789".as_slice(),
        b"b\"secret\"",
        b"any-secret-of-a-compromised-service-0123456789",
    ] {
        for uri in [calendar_uri(), "/api/v1/events/".to_string()] {
            let request = TestRequest::get()
                .uri(&uri)
                .insert_header(hs256(admin, secret));
            assert_eq!(status_of!(app, request), 401, "HS256 on {uri}");
        }
    }
}

/// Business journey with Core's tokens: a user creates an event, reads it back and deletes it;
/// the same user's HS256 token is refused on that event at every step.
#[tokio::test]
#[serial]
async fn mair_680_user_manages_an_event_with_core_tokens_only() {
    let ctx = TestContext::new().await;
    let creator = create_user(&ctx.db, "Journey", Some("User")).await;
    let app = init_app!(ctx);

    let created: Value = call_and_read_body_json(
        &app,
        TestRequest::post()
            .uri("/api/v1/events/")
            .insert_header(bearer(creator))
            .set_json(new_event_body("Public"))
            .to_request(),
    )
    .await;
    let uri = format!("/api/v1/events/{}/", created["event_id"]);

    let event: Value = call_and_read_body_json(
        &app,
        TestRequest::get()
            .uri(&uri)
            .insert_header(bearer(creator))
            .to_request(),
    )
    .await;
    assert_eq!(event["name"], "Permanence des élus");

    let legacy = hs256(creator, b"b\"secret\"");
    for request in [
        TestRequest::get().uri(&uri),
        TestRequest::delete().uri(&uri),
    ] {
        assert_eq!(status_of!(app, request.insert_header(legacy.clone())), 401);
    }

    let delete = TestRequest::delete()
        .uri(&uri)
        .insert_header(bearer(creator));
    assert_eq!(status_of!(app, delete), 204);
    let gone = TestRequest::get().uri(&uri).insert_header(bearer(creator));
    assert_eq!(status_of!(app, gone), 404);
}
