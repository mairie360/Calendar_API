//! Invalid inputs found by fuzzing `POST` and `PATCH /events` (MAIR-481): each one is a `400`
//! naming the faulty field, never a `500`, and an accepted event stays readable.

use actix_web::test::{call_service, TestRequest};
use serde_json::{json, Value};
use serial_test::serial;

use super::{bearer, new_event_body, TestContext};
use crate::common::{add_member, create_user};
use crate::init_app;

/// A creation body with `field` replaced by `value`.
fn with(field: &str, value: Value) -> Value {
    let mut body = new_event_body("Public");
    body[field] = value;
    body
}

/// A body with these dates.
fn dated(start: &str, end: &str) -> Value {
    json!({ "name": "Conseil municipal", "events_start_time": start, "events_end_time": end })
}

/// Each invalid body and the field its `400` must name.
fn invalid_bodies() -> Vec<(Value, &'static str)> {
    let recurrence = |rule: Value| with("recurrence", rule);
    vec![
        // Outside the Postgres range: the write itself used to fail.
        (
            dated("-5000-01-01T00:00:00Z", "2026-01-01T00:00:00Z"),
            "events_start_time",
        ),
        // Stored, then unreadable (`GET` and `GET /calendar` answered 500).
        (
            dated("0000-01-01T00:00:00Z", "0001-01-01T00:00:00Z"),
            "events_start_time",
        ),
        (
            dated("+262000-01-01T00:00:00Z", "+262001-01-01T00:00:00Z"),
            "events_start_time",
        ),
        (
            dated("1969-12-31T23:59:59Z", "1970-01-01T01:00:00Z"),
            "events_start_time",
        ),
        (
            dated("2026-10-05T18:00:00Z", "+200000-01-01T00:00:00Z"),
            "events_end_time",
        ),
        (
            dated("2026-10-05T18:00:00Z", "3000-01-01T00:00:00Z"),
            "events_end_time",
        ),
        (
            dated("2026-10-05T18:00:00Z", "2026-10-05T18:00:00Z"),
            "events_end_time",
        ),
        (
            dated("not a date", "2026-10-05T18:00:00Z"),
            "events_start_time",
        ),
        (
            with("events_end_time", json!("2026-10-05T18:00:00")),
            "events_end_time",
        ),
        (
            recurrence(json!({ "frequency": "daily", "interval": 1, "ends_on": "+262000-01-01" })),
            "recurrence.ends_on",
        ),
        (
            recurrence(json!({ "frequency": "daily", "interval": 1, "ends_on": "3000-01-01" })),
            "recurrence.ends_on",
        ),
        (
            recurrence(json!({ "frequency": "daily", "interval": 1, "ends_on": "2020-01-01" })),
            "recurrence.ends_on",
        ),
        (
            recurrence(json!({ "frequency": "daily", "interval": 0 })),
            "recurrence.interval",
        ),
        (
            recurrence(json!({ "frequency": "daily", "interval": -1 })),
            "recurrence.interval",
        ),
        (
            recurrence(json!({ "frequency": "daily", "interval": 4_294_967_296_u64 })),
            "recurrence.interval",
        ),
        (
            recurrence(json!({ "frequency": "weekly", "interval": 1, "days_of_week": [7] })),
            "recurrence.days_of_week",
        ),
        (
            recurrence(json!({ "frequency": "weekly", "interval": 1, "days_of_week": [300] })),
            "recurrence.days_of_week[0]",
        ),
        (
            recurrence(json!({ "frequency": "weekly", "interval": 1, "days_of_week": [] })),
            "recurrence.days_of_week",
        ),
        (
            recurrence(json!({ "frequency": "weekly", "interval": 1, "days_of_week": [1, 1] })),
            "recurrence.days_of_week",
        ),
        (
            recurrence(json!({ "frequency": "yearly", "interval": 1 })),
            "recurrence.frequency",
        ),
        (
            recurrence(json!({ "frequency": "daily", "interval": 1, "colour": "red" })),
            "recurrence.colour",
        ),
        (recurrence(json!("weekly")), "recurrence"),
        (with("name", json!(12)), "name"),
        (with("name", json!("a".repeat(151))), "name"),
        (with("visibility", json!("public")), "visibility"),
        (with("category", json!("party")), "category"),
        (with("service", json!("a".repeat(129))), "service"),
        (with("location", json!("a".repeat(256))), "location"),
        (with("description", json!("a".repeat(5001))), "description"),
        (with("description", json!("a\u{0}b")), "description"),
        (json!(null), "body"),
    ]
}

/// Sends a request and returns its status and text body.
macro_rules! refusal {
    ($app:expr, $request:expr $(,)?) => {{
        let response = call_service(&$app, $request.to_request()).await;
        let status = response.status().as_u16();
        let body = actix_web::test::read_body(response).await;
        (status, String::from_utf8_lossy(&body).into_owned())
    }};
}

#[tokio::test]
#[serial]
async fn invalid_bodies_are_400_naming_the_field() {
    let ctx = TestContext::new().await;
    let creator = create_user(&ctx.db, "Creator", Some("Maire")).await;
    let event = ctx.event(creator, "Public").await;
    add_member(&ctx.db, event, creator).await;
    let app = init_app!(ctx);

    for (body, field) in invalid_bodies() {
        for request in [
            TestRequest::post().uri("/api/v1/events/"),
            TestRequest::patch().uri(&format!("/api/v1/events/{event}/")),
        ] {
            let (status, text) =
                refusal!(app, request.insert_header(bearer(creator)).set_json(&body));
            assert_eq!(status, 400, "{body} → {text}");
            assert!(
                text.starts_with(&format!("Invalid `{field}`: ")),
                "{body} → {text}"
            );
        }
    }
    // The refused PATCHes left the event readable.
    let (status, _) = refusal!(
        app,
        TestRequest::get()
            .uri(&format!("/api/v1/events/{event}/"))
            .insert_header(bearer(creator)),
    );
    assert_eq!(status, 200);
}

#[tokio::test]
#[serial]
async fn a_missing_field_names_the_body() {
    let ctx = TestContext::new().await;
    let creator = create_user(&ctx.db, "Creator", Some("Maire")).await;
    let app = init_app!(ctx);

    let (status, text) = refusal!(
        app,
        TestRequest::post()
            .uri("/api/v1/events/")
            .insert_header(bearer(creator))
            .set_json(json!({ "name": "Conseil municipal" })),
    );
    assert_eq!(status, 400);
    assert_eq!(text, "Invalid `body`: missing field `events_start_time`.");
}

#[tokio::test]
#[serial]
async fn events_at_the_bounds_are_stored_and_readable() {
    let ctx = TestContext::new().await;
    let creator = create_user(&ctx.db, "Creator", Some("Maire")).await;
    let app = init_app!(ctx);
    let mut body = dated("1970-01-01T00:00:00Z", "2999-12-31T23:59:59Z");
    body["recurrence"] =
        json!({ "frequency": "monthly", "interval": 365, "ends_on": "2999-12-31" });

    let response = call_service(
        &app,
        TestRequest::post()
            .uri("/api/v1/events/")
            .insert_header(bearer(creator))
            .set_json(&body)
            .to_request(),
    )
    .await;
    assert_eq!(response.status().as_u16(), 201);
    let created: Value = actix_web::test::read_body_json(response).await;
    let event = created["event_id"].as_u64().unwrap();

    let read: Value = actix_web::test::call_and_read_body_json(
        &app,
        TestRequest::get()
            .uri(&format!("/api/v1/events/{event}/"))
            .insert_header(bearer(creator))
            .to_request(),
    )
    .await;
    assert_eq!(read["recurrence"]["ends_on"], "2999-12-31");
    for period in [
        "start=1970-01-01T00:00:00Z&end=1970-12-31T00:00:00Z",
        "start=2999-06-01T00:00:00Z&end=2999-12-31T23:59:59Z",
    ] {
        let (status, text) = refusal!(
            app,
            TestRequest::get()
                .uri(&format!("/api/v1/calendar?{period}"))
                .insert_header(bearer(creator)),
        );
        assert_eq!(status, 200, "{period} → {text}");
        assert!(
            text.contains(&format!("\"id\":{event}")),
            "{period} → {text}"
        );
    }
}

#[tokio::test]
#[serial]
async fn a_calendar_period_outside_the_window_is_400() {
    let ctx = TestContext::new().await;
    let user = create_user(&ctx.db, "Reader", Some("User")).await;
    let app = init_app!(ctx);

    for (period, field) in [
        (
            "start=-5000-01-01T00:00:00Z&end=-5000-06-01T00:00:00Z",
            "start",
        ),
        (
            "start=1969-06-01T00:00:00Z&end=1969-12-01T00:00:00Z",
            "start",
        ),
        ("start=2999-12-01T00:00:00Z&end=3000-01-01T00:00:00Z", "end"),
        ("start=2026-10-01T00:00:00Z&end=2026-09-01T00:00:00Z", "end"),
    ] {
        let (status, text) = refusal!(
            app,
            TestRequest::get()
                .uri(&format!("/api/v1/calendar?{}", period.replace('+', "%2B")))
                .insert_header(bearer(user)),
        );
        assert_eq!(status, 400, "{period} → {text}");
        assert!(
            text.starts_with(&format!("Invalid `{field}`: ")),
            "{period} → {text}"
        );
    }
}

#[tokio::test]
#[serial]
async fn member_and_validation_bodies_name_the_field() {
    let ctx = TestContext::new().await;
    let creator = create_user(&ctx.db, "Creator", Some("Maire")).await;
    let event = ctx.event(creator, "Public").await;
    let app = init_app!(ctx);

    for (uri, request, body, expected) in [
        (
            format!("/api/v1/events/{event}/members/"),
            TestRequest::post(),
            json!({ "user_id": "abc" }),
            "Invalid `user_id`: ",
        ),
        (
            format!("/api/v1/events/{event}/validation"),
            TestRequest::patch(),
            json!({ "status": "Maybe" }),
            "Invalid `status`: ",
        ),
    ] {
        let (status, text) = refusal!(
            app,
            request
                .uri(&uri)
                .insert_header(bearer(creator))
                .set_json(body),
        );
        assert_eq!(status, 400, "{uri} → {text}");
        assert!(text.starts_with(expected), "{uri} → {text}");
    }
}
