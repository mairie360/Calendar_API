//! MAIR-1228: `GET /api/v1/search`, Calendar's part of the global search of the shared header
//! (events the caller sees in the calendar). Each test names its events with a unique marker
//! used as the term.

use actix_web::test::{call_and_read_body_json, TestRequest};
use serde_json::Value;

use super::{bearer, TestContext};
use crate::common::{add_member, create_user, raw_sql};
use crate::{init_app, status_of};

/// A marker made of letters only, unique to the test.
fn marker(prefix: &str) -> String {
    static COUNTER: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);
    let nanos = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    let unique = format!(
        "{nanos}{}",
        COUNTER.fetch_add(1, std::sync::atomic::Ordering::Relaxed)
    );
    let letters: String = unique
        .bytes()
        .map(|digit| char::from(b'a' + (digit - b'0')))
        .collect();
    format!("{prefix}{letters}")
}

async fn rename(ctx: &TestContext, event_id: u64, name: &str) {
    raw_sql(
        &ctx.db,
        format!("UPDATE events SET name = '{name}' WHERE id = {event_id}"),
    )
    .await;
}

fn ids(body: &Value) -> Vec<u64> {
    body["items"]
        .as_array()
        .expect("items")
        .iter()
        .map(|item| item["id"].as_u64().unwrap())
        .collect()
}

#[tokio::test]
async fn mair_1228_finds_public_events_and_the_private_ones_of_the_caller_only() {
    let ctx = TestContext::new().await;
    let creator = create_user(&ctx.db, "Creator", Some("User")).await;
    let member = create_user(&ctx.db, "Member", Some("User")).await;
    let outsider = create_user(&ctx.db, "Outsider", Some("User")).await;
    let term = marker("conseil");
    let public = ctx.event(creator, "Public").await;
    let private = ctx.event(creator, "Private").await;
    rename(&ctx, public, &format!("{term} public")).await;
    rename(&ctx, private, &format!("Réunion {term} privée")).await;
    add_member(&ctx.db, private, member).await;
    let app = init_app!(ctx);

    let search = |caller: u64| {
        TestRequest::get()
            .uri(&format!("/api/v1/search?q={term}"))
            .insert_header(bearer(caller))
            .to_request()
    };
    let as_creator: Value = call_and_read_body_json(&app, search(creator)).await;
    // The name starting with the term comes first.
    assert_eq!(ids(&as_creator), vec![public, private], "{as_creator}");
    let first = &as_creator["items"][0];
    assert_eq!(first["kind"], "event");
    assert_eq!(first["title"], format!("{term} public"));
    assert!(first["parent_id"].is_null());
    assert!(first["updated_at"].as_str().unwrap().ends_with('Z'));

    let as_member: Value = call_and_read_body_json(&app, search(member)).await;
    assert_eq!(ids(&as_member), vec![public, private]);
    let as_outsider: Value = call_and_read_body_json(&app, search(outsider)).await;
    assert_eq!(
        ids(&as_outsider),
        vec![public],
        "a private event stays hidden"
    );
}

#[tokio::test]
async fn mair_1228_limit_cuts_and_wildcards_match_themselves() {
    let ctx = TestContext::new().await;
    let creator = create_user(&ctx.db, "Creator", Some("User")).await;
    let term = marker("marche");
    for index in 0..3 {
        let event = ctx.event(creator, "Public").await;
        rename(&ctx, event, &format!("Lot {index} {term}")).await;
    }
    let app = init_app!(ctx);

    let limited: Value = call_and_read_body_json(
        &app,
        TestRequest::get()
            .uri(&format!("/api/v1/search?q={term}&limit=2"))
            .insert_header(bearer(creator))
            .to_request(),
    )
    .await;
    assert_eq!(ids(&limited).len(), 2, "{limited}");
    // `%_` would match every event if the wildcards were not escaped.
    let wildcards: Value = call_and_read_body_json(
        &app,
        TestRequest::get()
            .uri("/api/v1/search?q=%25_")
            .insert_header(bearer(creator))
            .to_request(),
    )
    .await;
    assert!(ids(&wildcards).is_empty(), "{wildcards}");
}

#[tokio::test]
async fn mair_1228_rejects_a_short_term_a_bad_limit_and_no_session() {
    let ctx = TestContext::new().await;
    let user = create_user(&ctx.db, "Reader", Some("User")).await;
    let app = init_app!(ctx);
    for query in [
        "?q=a",
        "?q=%20a%20",
        "?q=%0Aab",
        "",
        "?q=ab&limit=21",
        "?q=ab&limit=0",
    ] {
        let request = TestRequest::get()
            .uri(&format!("/api/v1/search{query}"))
            .insert_header(bearer(user));
        assert_eq!(status_of!(app, request), 400, "{query}");
    }
    let anonymous = TestRequest::get().uri("/api/v1/search?q=conseil");
    assert_eq!(status_of!(app, anonymous), 401);
}
