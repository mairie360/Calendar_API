//! `POST /events` with `members` (MAIR-562): the event and its members are written in one
//! transaction, so a refused assignment leaves no orphan event behind.

use actix_web::test::{call_service, read_body_json, TestRequest};
use serde_json::{json, Value};
use serial_test::serial;

use super::{bearer, new_event_body, TestContext};
use crate::common::{create_group, create_user, created_event_count, member_ids};
use crate::{init_app, status_of};

fn with_members(members: Value) -> Value {
    let mut body = new_event_body("Public");
    body["members"] = members;
    body
}

fn create(caller: u64, body: Value) -> TestRequest {
    TestRequest::post()
        .uri("/api/v1/events/")
        .insert_header(bearer(caller))
        .set_json(body)
}

#[tokio::test]
#[serial]
async fn an_event_is_created_with_its_members_in_one_call() {
    let ctx = TestContext::new().await;
    let creator = create_user(&ctx.db, "Creator", Some("User")).await;
    let colleague = create_user(&ctx.db, "Colleague", Some("User")).await;
    create_group(&ctx.db, creator, &[creator, colleague]).await;
    let app = init_app!(ctx);

    let response = call_service(
        &app,
        create(creator, with_members(json!([colleague, creator]))).to_request(),
    )
    .await;
    assert_eq!(response.status().as_u16(), 201);
    let body: Value = read_body_json(response).await;
    let event = body["event_id"].as_u64().unwrap();

    let mut expected = vec![creator, colleague];
    expected.sort_unstable();
    assert_eq!(member_ids(&ctx.db, event).await, expected);
}

#[tokio::test]
#[serial]
async fn a_body_without_members_still_creates_a_memberless_event() {
    let ctx = TestContext::new().await;
    let creator = create_user(&ctx.db, "Creator", Some("User")).await;
    let app = init_app!(ctx);

    for body in [new_event_body("Public"), with_members(json!([]))] {
        let response = call_service(&app, create(creator, body).to_request()).await;
        assert_eq!(response.status().as_u16(), 201);
        let body: Value = read_body_json(response).await;
        assert!(member_ids(&ctx.db, body["event_id"].as_u64().unwrap())
            .await
            .is_empty());
    }
}

#[tokio::test]
#[serial]
async fn mair_562_a_refused_member_leaves_no_event_behind() {
    let ctx = TestContext::new().await;
    let creator = create_user(&ctx.db, "Creator", Some("User")).await;
    let colleague = create_user(&ctx.db, "Colleague", Some("User")).await;
    let stranger = create_user(&ctx.db, "Stranger", Some("User")).await;
    create_group(&ctx.db, creator, &[creator, colleague]).await;
    let app = init_app!(ctx);

    // The refused id comes after an accepted one: its assignment was already written when the
    // refusal happens, and must be rolled back with the event.
    for members in [
        json!([colleague, stranger]),
        json!([creator, 2_147_483_000_u64]),
    ] {
        assert_eq!(status_of!(app, create(creator, with_members(members))), 403);
    }
    assert_eq!(created_event_count(&ctx.db, creator).await, 0);
}

#[tokio::test]
#[serial]
async fn invalid_member_lists_are_refused_before_any_write() {
    let ctx = TestContext::new().await;
    let creator = create_user(&ctx.db, "Creator", Some("Admin")).await;
    let app = init_app!(ctx);

    let cases = [
        (json!([creator, creator]), "members[1]"),
        (json!((1..=501).collect::<Vec<u64>>()), "members"),
        (json!(["12"]), "members[0]"),
        (json!(12), "members"),
    ];
    for (members, field) in cases {
        let response = call_service(
            &app,
            create(creator, with_members(members.clone())).to_request(),
        )
        .await;
        assert_eq!(response.status().as_u16(), 400, "{members}");
        let text = String::from_utf8(actix_web::test::read_body(response).await.to_vec()).unwrap();
        assert!(
            text.starts_with(&format!("Invalid `{field}`")),
            "{members}: {text}"
        );
    }
    assert_eq!(created_event_count(&ctx.db, creator).await, 0);
}
