use actix_web::test::{call_and_read_body_json, TestRequest};
use chrono::{Duration, SecondsFormat, Utc};
use serde_json::{json, Value};

use super::{bearer, TestContext};
use crate::common::{add_member, create_user};
use crate::{init_app, status_of};

fn next_week() -> String {
    let start = Utc::now();
    format!(
        "/api/v1/calendar?start={}&end={}",
        start.to_rfc3339_opts(SecondsFormat::Secs, true),
        (start + Duration::days(7)).to_rfc3339_opts(SecondsFormat::Secs, true)
    )
}

#[tokio::test]
async fn mair_1122_an_invited_member_is_pending_then_answers() {
    let ctx = TestContext::new().await;
    let creator = create_user(&ctx.db, "Creator", Some("User")).await;
    let member = create_user(&ctx.db, "Member", Some("User")).await;
    let event = ctx.event(creator, "Private").await;
    add_member(&ctx.db, event, member).await;
    let app = init_app!(ctx);
    let detail_uri = format!("/api/v1/events/{event}/");
    let answer = |status: &str| {
        TestRequest::put()
            .uri(&format!("/api/v1/events/{event}/participation"))
            .insert_header(bearer(member))
            .set_json(json!({ "status": status }))
    };

    let detail: Value = call_and_read_body_json(
        &app,
        TestRequest::get()
            .uri(&detail_uri)
            .insert_header(bearer(member))
            .to_request(),
    )
    .await;
    assert_eq!(detail["my_participation"], "pending");
    assert_eq!(detail["permissions"]["can_respond"], true);
    assert_eq!(detail["members"][0]["participation"], "pending");

    assert_eq!(status_of!(app, answer("declined")), 204);
    let calendar: Value = call_and_read_body_json(
        &app,
        TestRequest::get()
            .uri(&next_week())
            .insert_header(bearer(member))
            .to_request(),
    )
    .await;
    let listed = calendar["events"]
        .as_array()
        .unwrap()
        .iter()
        .find(|listed| listed["id"].as_u64() == Some(event))
        .cloned()
        .expect("the event is listed");
    assert_eq!(listed["my_participation"], "declined");
    assert_eq!(listed["members"][0]["participation"], "declined");

    assert_eq!(status_of!(app, answer("accepted")), 204);
    let detail: Value = call_and_read_body_json(
        &app,
        TestRequest::get()
            .uri(&detail_uri)
            .insert_header(bearer(member))
            .to_request(),
    )
    .await;
    assert_eq!(detail["my_participation"], "accepted");
}

#[tokio::test]
async fn mair_1122_only_an_assigned_member_answers() {
    let ctx = TestContext::new().await;
    let creator = create_user(&ctx.db, "Creator", Some("User")).await;
    let outsider = create_user(&ctx.db, "Outsider", Some("Admin")).await;
    let event = ctx.event(creator, "Public").await;
    let app = init_app!(ctx);
    let answer = |caller: u64, uri: String| {
        TestRequest::put()
            .uri(&uri)
            .insert_header(bearer(caller))
            .set_json(json!({ "status": "accepted" }))
    };
    let uri = format!("/api/v1/events/{event}/participation");

    // The creator who is not assigned and an Admin outsider cannot answer.
    assert_eq!(status_of!(app, answer(creator, uri.clone())), 403);
    assert_eq!(status_of!(app, answer(outsider, uri.clone())), 403);
    assert_eq!(
        status_of!(
            app,
            answer(outsider, "/api/v1/events/2147483000/participation".into())
        ),
        404
    );

    let detail: Value = call_and_read_body_json(
        &app,
        TestRequest::get()
            .uri(&format!("/api/v1/events/{event}/"))
            .insert_header(bearer(outsider))
            .to_request(),
    )
    .await;
    assert_eq!(detail["my_participation"], Value::Null);
    assert_eq!(detail["permissions"]["can_respond"], false);
}

#[tokio::test]
async fn mair_1122_an_unknown_answer_is_refused() {
    let ctx = TestContext::new().await;
    let creator = create_user(&ctx.db, "Creator", Some("User")).await;
    let member = create_user(&ctx.db, "Member", Some("User")).await;
    let event = ctx.event(creator, "Public").await;
    add_member(&ctx.db, event, member).await;
    let app = init_app!(ctx);
    let uri = format!("/api/v1/events/{event}/participation");

    for body in [
        json!({ "status": "maybe" }),
        json!({ "status": "pending" }),
        json!({ "status": "accepted", "extra": 1 }),
        json!({}),
    ] {
        assert_eq!(
            status_of!(
                app,
                TestRequest::put()
                    .uri(&uri)
                    .insert_header(bearer(member))
                    .set_json(&body)
            ),
            400,
            "{body}"
        );
    }
}

#[tokio::test]
async fn mair_1122_a_creator_assigning_themselves_takes_part() {
    let ctx = TestContext::new().await;
    let creator = create_user(&ctx.db, "Creator", Some("Admin")).await;
    let colleague = create_user(&ctx.db, "Colleague", Some("User")).await;
    let event = ctx.event(creator, "Public").await;
    let app = init_app!(ctx);
    let uri = format!("/api/v1/events/{event}/members/");
    for user_id in [creator, colleague] {
        assert_eq!(
            status_of!(
                app,
                TestRequest::post()
                    .uri(&uri)
                    .insert_header(bearer(creator))
                    .set_json(json!({ "user_id": user_id }))
            ),
            201
        );
    }

    let members: Value = call_and_read_body_json(
        &app,
        TestRequest::get()
            .uri(&uri)
            .insert_header(bearer(creator))
            .to_request(),
    )
    .await;
    let participation_of = |user: u64| {
        members["members"]
            .as_array()
            .unwrap()
            .iter()
            .find(|m| m["id"].as_u64() == Some(user))
            .map(|m| m["participation"].clone())
    };
    assert_eq!(participation_of(creator), Some(json!("accepted")));
    assert_eq!(participation_of(colleague), Some(json!("pending")));
}
