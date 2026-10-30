//! Redis read cache of `GET /calendar` (MAIR-524), on a real Redis container: a second read is
//! served from the cache, every write endpoint invalidates it once committed, and two callers
//! never share a page.

use actix_web::test::{call_and_read_body_json, TestRequest};
use chrono::{Duration, SecondsFormat, Utc};
use serde_json::{json, Value};
use serial_test::serial;

use super::{bearer, new_event_body, TestContext};
use crate::common::{add_member, create_group, create_user, raw_sql};
use crate::{init_app, status_of};

/// One fixed period (the next week) per test: the cache key is the exact `(start, end)`, so
/// every read of a test must send the same bounds.
fn next_week() -> String {
    let start = Utc::now() - Duration::hours(1);
    let end = start + Duration::days(8);
    format!(
        "/api/v1/calendar?start={}&end={}",
        start.to_rfc3339_opts(SecondsFormat::Secs, true),
        end.to_rfc3339_opts(SecondsFormat::Secs, true)
    )
}

macro_rules! calendar {
    ($app:expr, $uri:expr, $user:expr) => {{
        let calendar: Value = call_and_read_body_json(
            &$app,
            TestRequest::get()
                .uri(&$uri)
                .insert_header(bearer($user))
                .to_request(),
        )
        .await;
        calendar
    }};
}

fn listed(calendar: &Value, event: u64) -> Option<Value> {
    calendar["events"]
        .as_array()
        .unwrap()
        .iter()
        .find(|listed| listed["id"].as_u64() == Some(event))
        .cloned()
}

fn member_ids(calendar: &Value, event: u64) -> Vec<u64> {
    listed(calendar, event).expect("event listed")["members"]
        .as_array()
        .unwrap()
        .iter()
        .map(|member| member["id"].as_u64().unwrap())
        .collect()
}

#[tokio::test]
#[serial]
async fn mair_524_a_second_read_is_served_from_the_cache() {
    let ctx = TestContext::with_redis().await;
    let mayor = create_user(&ctx.db, "Mayor", Some("Maire")).await;
    let event = ctx.event(mayor, "Public").await;
    add_member(&ctx.db, event, mayor).await;
    let app = init_app!(ctx);
    let uri = next_week();

    let first = calendar!(app, uri, mayor);
    assert_eq!(listed(&first, event).unwrap()["name"], "Conseil municipal");

    // Changed behind the API's back: nothing invalidates the page.
    raw_sql(
        &ctx.db,
        format!("UPDATE events SET name = 'Renamed behind the API' WHERE id = {event}"),
    )
    .await;
    let cached = calendar!(app, uri, mayor);
    assert_eq!(cached, first, "served from the cache");

    // A write through the API invalidates it.
    assert_eq!(
        status_of!(
            app,
            TestRequest::patch()
                .uri(&format!("/api/v1/events/{event}/"))
                .insert_header(bearer(mayor))
                .set_json(json!({ "name": "Conseil extraordinaire" }))
        ),
        204
    );
    let fresh = calendar!(app, uri, mayor);
    assert_eq!(
        listed(&fresh, event).unwrap()["name"],
        "Conseil extraordinaire"
    );
}

#[tokio::test]
#[serial]
async fn mair_524_every_event_write_invalidates_the_calendar() {
    let ctx = TestContext::with_redis().await;
    let mayor = create_user(&ctx.db, "Mayor", Some("Maire")).await;
    let colleague = create_user(&ctx.db, "Colleague", Some("User")).await;
    let app = init_app!(ctx);
    let uri = next_week();
    let event_uri = |event: u64| format!("/api/v1/events/{event}/");
    let before = calendar!(app, uri, mayor);

    // POST /events
    let created: Value = call_and_read_body_json(
        &app,
        TestRequest::post()
            .uri("/api/v1/events/")
            .insert_header(bearer(mayor))
            .set_json(new_event_body("Public"))
            .to_request(),
    )
    .await;
    let event = created["event_id"].as_u64().expect("created");
    assert!(listed(&before, event).is_none());
    assert_eq!(
        member_ids(&calendar!(app, uri, mayor), event),
        Vec::<u64>::new()
    );

    // POST /events/{id}/members (twice: the creator, then a colleague)
    for user in [mayor, colleague] {
        assert_eq!(
            status_of!(
                app,
                TestRequest::post()
                    .uri(&format!("{}members/", event_uri(event)))
                    .insert_header(bearer(mayor))
                    .set_json(json!({ "user_id": user }))
            ),
            201
        );
    }
    assert_eq!(
        member_ids(&calendar!(app, uri, mayor), event),
        vec![mayor, colleague]
    );

    // DELETE /events/{id}/members/{member_id}
    assert_eq!(
        status_of!(
            app,
            TestRequest::delete()
                .uri(&format!("{}members/{colleague}/", event_uri(event)))
                .insert_header(bearer(mayor))
        ),
        204
    );
    assert_eq!(member_ids(&calendar!(app, uri, mayor), event), vec![mayor]);

    // PATCH /events/{id}: a recurrence added, then dropped (orphan rule cleanup).
    let patch = |body: Value| {
        TestRequest::patch()
            .uri(&event_uri(event))
            .insert_header(bearer(mayor))
            .set_json(body)
    };
    assert_eq!(
        status_of!(
            app,
            patch(json!({ "recurrence": { "frequency": "weekly", "interval": 1 } }))
        ),
        204
    );
    let recurring = calendar!(app, uri, mayor);
    assert_eq!(
        listed(&recurring, event).unwrap()["recurrence"]["frequency"],
        "weekly"
    );
    assert_eq!(status_of!(app, patch(json!({ "recurrence": null }))), 204);
    assert_eq!(
        listed(&calendar!(app, uri, mayor), event).unwrap()["recurrence"],
        Value::Null
    );

    // DELETE /events/{id}
    assert_eq!(
        status_of!(
            app,
            TestRequest::delete()
                .uri(&event_uri(event))
                .insert_header(bearer(mayor))
        ),
        204
    );
    assert!(listed(&calendar!(app, uri, mayor), event).is_none());
}

#[tokio::test]
#[serial]
async fn mair_524_an_approval_invalidates_every_callers_calendar() {
    let ctx = TestContext::with_redis().await;
    let creator = create_user(&ctx.db, "Creator", Some("User")).await;
    let responsable = create_user(&ctx.db, "Responsable", Some("Responsable")).await;
    create_group(&ctx.db, creator, &[creator, responsable]).await;
    let event = ctx.event(creator, "Public").await;
    add_member(&ctx.db, event, creator).await;
    add_member(&ctx.db, event, responsable).await;
    let app = init_app!(ctx);
    let uri = next_week();

    let statuses = |calendar: &Value| -> Vec<String> {
        listed(calendar, event).expect("listed")["members"]
            .as_array()
            .unwrap()
            .iter()
            .map(|member| member["validation_status"].as_str().unwrap().to_string())
            .collect()
    };
    for user in [creator, responsable] {
        assert_eq!(statuses(&calendar!(app, uri, user)), ["pending", "pending"]);
    }

    assert_eq!(
        status_of!(
            app,
            TestRequest::patch()
                .uri(&format!("/api/v1/events/{event}/validation"))
                .insert_header(bearer(responsable))
                .set_json(json!({ "status": "approved" }))
        ),
        204
    );
    // Both entries (the decider's and the creator's) are invalidated.
    for user in [creator, responsable] {
        assert_eq!(
            statuses(&calendar!(app, uri, user)),
            ["validated", "validated"]
        );
    }
}

#[tokio::test]
#[serial]
async fn mair_524_two_callers_never_share_a_cached_page() {
    let ctx = TestContext::with_redis().await;
    let alice = create_user(&ctx.db, "Alice", Some("User")).await;
    let bob = create_user(&ctx.db, "Bob", Some("User")).await;
    let alices = ctx.event(alice, "Private").await;
    let bobs = ctx.event(bob, "Private").await;
    let app = init_app!(ctx);
    let uri = next_week();

    // Same period, both orders: each reads their own page.
    let as_alice = calendar!(app, uri, alice);
    let as_bob = calendar!(app, uri, bob);
    assert!(listed(&as_alice, alices).is_some());
    assert!(listed(&as_alice, bobs).is_none());
    assert!(listed(&as_bob, bobs).is_some());
    assert!(listed(&as_bob, alices).is_none());
    // Cached now: still their own.
    assert_eq!(calendar!(app, uri, alice), as_alice);
    assert_eq!(calendar!(app, uri, bob), as_bob);
}
