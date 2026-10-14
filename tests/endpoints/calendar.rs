use actix_web::test::{call_and_read_body_json, TestRequest};
use chrono::{Duration, SecondsFormat, Utc};
use serde_json::Value;
use serial_test::serial;

use super::{bearer, TestContext};
use crate::common::{add_member, create_user, raw_sql};
use crate::{init_app, status_of};

fn period(start_in_days: i64, length_days: i64) -> String {
    let start = Utc::now() + Duration::days(start_in_days);
    let end = start + Duration::days(length_days);
    format!(
        "/api/v1/calendar?start={}&end={}",
        start.to_rfc3339_opts(SecondsFormat::Secs, true),
        end.to_rfc3339_opts(SecondsFormat::Secs, true)
    )
}

#[tokio::test]
#[serial]
async fn an_invalid_period_is_refused() {
    let ctx = TestContext::new().await;
    let user = create_user(&ctx.db, "Reader", Some("User")).await;
    let app = init_app!(ctx);

    for uri in [
        period(0, -1),
        period(0, 367),
        "/api/v1/calendar".to_string(),
        "/api/v1/calendar?start=demain&end=apres-demain".to_string(),
    ] {
        assert_eq!(
            status_of!(
                app,
                TestRequest::get().uri(&uri).insert_header(bearer(user))
            ),
            400,
            "{uri}"
        );
    }
}

#[tokio::test]
#[serial]
async fn private_events_of_others_are_not_listed() {
    let ctx = TestContext::new().await;
    let creator = create_user(&ctx.db, "Creator", Some("User")).await;
    let outsider = create_user(&ctx.db, "Outsider", Some("User")).await;
    let public = ctx.event(creator, "Public").await;
    let private = ctx.event(creator, "Private").await;
    let app = init_app!(ctx);

    let listed = |body: Value| -> Vec<u64> {
        body["events"]
            .as_array()
            .unwrap()
            .iter()
            .map(|event| event["id"].as_u64().unwrap())
            .collect()
    };
    let as_outsider = listed(
        call_and_read_body_json(
            &app,
            TestRequest::get()
                .uri(&period(0, 7))
                .insert_header(bearer(outsider))
                .to_request(),
        )
        .await,
    );
    assert!(as_outsider.contains(&public));
    assert!(!as_outsider.contains(&private));

    let as_creator = listed(
        call_and_read_body_json(
            &app,
            TestRequest::get()
                .uri(&period(0, 7))
                .insert_header(bearer(creator))
                .to_request(),
        )
        .await,
    );
    assert!(as_creator.contains(&private));
}

/// The calendar of `user` over the next week, as JSON.
macro_rules! calendar_of {
    ($app:expr, $user:expr) => {{
        let calendar: Value = call_and_read_body_json(
            &$app,
            TestRequest::get()
                .uri(&period(0, 7))
                .insert_header(bearer($user))
                .to_request(),
        )
        .await;
        calendar
    }};
}

/// The listed event `event`, if any.
fn listed_event(calendar: &Value, event: u64) -> Option<Value> {
    calendar["events"]
        .as_array()
        .unwrap()
        .iter()
        .find(|listed| listed["id"].as_u64() == Some(event))
        .cloned()
}

#[tokio::test]
#[serial]
async fn mair_524_calendar_lists_members_like_the_event_detail() {
    let ctx = TestContext::new().await;
    // A plain User creates a pending event, a Maire a validated one.
    let creator = create_user(&ctx.db, "Creator", Some("User")).await;
    let mayor = create_user(&ctx.db, "Mayor", Some("Maire")).await;
    let first = create_user(&ctx.db, "First", Some("User")).await;
    let second = create_user(&ctx.db, "Second", Some("User")).await;
    let pending = ctx.event(creator, "Public").await;
    let validated = ctx.event(mayor, "Public").await;
    let empty = ctx.event(mayor, "Public").await;
    // Assigned in reverse id order: the list is sorted by id, not by insertion.
    for event in [pending, validated] {
        add_member(&ctx.db, event, second).await;
        add_member(&ctx.db, event, first).await;
    }
    let app = init_app!(ctx);

    let calendar = calendar_of!(app, creator);
    for (event, status) in [(pending, "pending"), (validated, "validated")] {
        let listed = listed_event(&calendar, event).expect("event listed");
        assert_eq!(
            listed["members"],
            serde_json::json!([
                { "id": first, "validation_status": status },
                { "id": second, "validation_status": status },
            ]),
            "{event}"
        );
        let detail: Value = call_and_read_body_json(
            &app,
            TestRequest::get()
                .uri(&format!("/api/v1/events/{event}/"))
                .insert_header(bearer(creator))
                .to_request(),
        )
        .await;
        assert_eq!(listed["members"], detail["members"], "{event}");
    }
    // Nobody assigned: an empty list, never null.
    assert_eq!(
        listed_event(&calendar, empty).expect("event listed")["members"],
        serde_json::json!([])
    );
    // `is_member` still reflects the caller.
    let as_first = calendar_of!(app, first);
    assert_eq!(
        listed_event(&as_first, pending).unwrap()["is_member"],
        Value::Bool(true)
    );
    assert_eq!(
        listed_event(&calendar, pending).unwrap()["is_member"],
        Value::Bool(false)
    );
}

#[tokio::test]
#[serial]
async fn mair_524_calendar_does_not_leak_members_the_detail_route_refuses() {
    let ctx = TestContext::new().await;
    let creator = create_user(&ctx.db, "Creator", Some("User")).await;
    let member = create_user(&ctx.db, "Member", Some("User")).await;
    let owner = create_user(&ctx.db, "Owner", Some("User")).await;
    let outsider = create_user(&ctx.db, "Outsider", Some("User")).await;
    let private = ctx.event(creator, "Private").await;
    add_member(&ctx.db, private, member).await;
    let app = init_app!(ctx);
    let expected = serde_json::json!([{ "id": member, "validation_status": "pending" }]);
    for allowed in [creator, member] {
        let listed = listed_event(&calendar_of!(app, allowed), private).expect("listed");
        assert_eq!(listed["members"], expected, "{allowed}");
    }

    // Owned by `owner` without being created by them nor assigned to them: listed in their
    // calendar (owner_id), but `GET /events/{id}` answers them 403.
    raw_sql(
        &ctx.db,
        format!("UPDATE events SET owner_id = {owner} WHERE id = {private}"),
    )
    .await;
    let listed = listed_event(&calendar_of!(app, member), private).expect("member lists it");
    assert_eq!(listed["members"], expected);

    assert!(listed_event(&calendar_of!(app, outsider), private).is_none());

    let as_owner = listed_event(&calendar_of!(app, owner), private).expect("owner lists it");
    assert_eq!(as_owner["members"], serde_json::json!([]));
    assert_eq!(
        status_of!(
            app,
            TestRequest::get()
                .uri(&format!("/api/v1/events/{private}/"))
                .insert_header(bearer(owner))
        ),
        403
    );
}
