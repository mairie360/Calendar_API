use actix_web::test::TestRequest;
use serde_json::json;
use serial_test::serial;

use super::{bearer, TestContext};
use crate::common::{add_member, create_group, create_user};
use crate::{init_app, status_of};

/// A pending event of a plain User, with a Responsable sharing a group with the creator, both
/// assigned. Returns (event, creator, responsable).
async fn pending_event(ctx: &TestContext) -> (u64, u64, u64) {
    let creator = create_user(&ctx.db, "Creator", Some("User")).await;
    let responsable = create_user(&ctx.db, "Responsable", Some("Responsable")).await;
    create_group(&ctx.db, creator, &[creator, responsable]).await;
    let event = ctx.event(creator, "Public").await;
    add_member(&ctx.db, event, creator).await;
    add_member(&ctx.db, event, responsable).await;
    (event, creator, responsable)
}

fn decide(event: u64, caller: u64, status: &str) -> TestRequest {
    TestRequest::patch()
        .uri(&format!("/api/v1/events/{event}/validation"))
        .insert_header(bearer(caller))
        .set_json(json!({ "status": status }))
}

#[tokio::test]
#[serial]
async fn the_creator_cannot_approve_their_own_event() {
    let ctx = TestContext::new().await;
    let (event, creator, _) = pending_event(&ctx).await;
    let app = init_app!(ctx);

    assert_eq!(status_of!(app, decide(event, creator, "approved")), 403);
}

#[tokio::test]
#[serial]
async fn only_an_assigned_responsable_of_the_creator_group_decides() {
    let ctx = TestContext::new().await;
    let (event, creator, _) = pending_event(&ctx).await;
    // Same group as the creator, but not assigned.
    let unassigned = create_user(&ctx.db, "Unassigned", Some("Responsable")).await;
    create_group(&ctx.db, creator, &[creator, unassigned]).await;
    // Assigned, but sharing no group with the creator.
    let foreign = create_user(&ctx.db, "Foreign", Some("Responsable")).await;
    add_member(&ctx.db, event, foreign).await;
    let app = init_app!(ctx);

    assert_eq!(status_of!(app, decide(event, unassigned, "approved")), 403);
    assert_eq!(status_of!(app, decide(event, foreign, "approved")), 403);
}

#[tokio::test]
#[serial]
async fn a_decision_is_taken_once() {
    let ctx = TestContext::new().await;
    let (event, _, responsable) = pending_event(&ctx).await;
    let app = init_app!(ctx);

    assert_eq!(status_of!(app, decide(event, responsable, "rejected")), 204);
    assert_eq!(status_of!(app, decide(event, responsable, "approved")), 403);
}

#[tokio::test]
#[serial]
async fn an_event_that_needs_no_approval_cannot_be_decided() {
    let ctx = TestContext::new().await;
    let mayor = create_user(&ctx.db, "Mayor", Some("Maire")).await;
    let responsable = create_user(&ctx.db, "Responsable", Some("Responsable")).await;
    create_group(&ctx.db, mayor, &[mayor, responsable]).await;
    let event = ctx.event(mayor, "Public").await;
    add_member(&ctx.db, event, responsable).await;
    let app = init_app!(ctx);

    assert_eq!(status_of!(app, decide(event, responsable, "approved")), 403);
}
