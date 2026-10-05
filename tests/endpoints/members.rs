use actix_web::test::TestRequest;
use serde_json::json;
use serial_test::serial;

use super::{bearer, TestContext};
use crate::common::{add_member, archive_user, create_group, create_user};
use crate::{init_app, status_of};

#[tokio::test]
#[serial]
async fn only_the_creator_or_an_editor_manages_members() {
    let ctx = TestContext::new().await;
    let creator = create_user(&ctx.db, "Creator", Some("User")).await;
    let member = create_user(&ctx.db, "Member", Some("User")).await;
    let outsider = create_user(&ctx.db, "Outsider", Some("Admin")).await;
    let event = ctx.event(creator, "Public").await;
    add_member(&ctx.db, event, member).await;
    let app = init_app!(ctx);
    let uri = format!("/api/v1/events/{event}/members/");

    for caller in [member, outsider] {
        assert_eq!(
            status_of!(
                app,
                TestRequest::post()
                    .uri(&uri)
                    .insert_header(bearer(caller))
                    .set_json(json!({ "user_id": caller }))
            ),
            403
        );
        assert_eq!(
            status_of!(
                app,
                TestRequest::delete()
                    .uri(&format!("{uri}{member}/"))
                    .insert_header(bearer(caller))
            ),
            403
        );
    }
}

#[tokio::test]
#[serial]
async fn a_user_is_only_assigned_inside_the_caller_scope() {
    let ctx = TestContext::new().await;
    let creator = create_user(&ctx.db, "Creator", Some("User")).await;
    let colleague = create_user(&ctx.db, "Colleague", Some("User")).await;
    let stranger = create_user(&ctx.db, "Stranger", Some("User")).await;
    let archived = create_user(&ctx.db, "Archived", Some("User")).await;
    create_group(&ctx.db, creator, &[creator, colleague, archived]).await;
    archive_user(&ctx.db, archived).await;
    let event = ctx.event(creator, "Public").await;
    let app = init_app!(ctx);
    let uri = format!("/api/v1/events/{event}/members/");
    let assign = |user_id: u64| {
        TestRequest::post()
            .uri(&uri)
            .insert_header(bearer(creator))
            .set_json(json!({ "user_id": user_id }))
    };

    assert_eq!(status_of!(app, assign(stranger)), 403);
    assert_eq!(status_of!(app, assign(archived)), 403);
    assert_eq!(status_of!(app, assign(2_147_483_000)), 403);
    assert_eq!(status_of!(app, assign(creator)), 201);
    assert_eq!(status_of!(app, assign(colleague)), 201);
    assert_eq!(status_of!(app, assign(colleague)), 409);
}

#[tokio::test]
#[serial]
async fn removing_a_user_who_is_not_assigned_is_not_found() {
    let ctx = TestContext::new().await;
    let creator = create_user(&ctx.db, "Creator", Some("User")).await;
    let member = create_user(&ctx.db, "Member", Some("User")).await;
    let event = ctx.event(creator, "Public").await;
    add_member(&ctx.db, event, member).await;
    let app = init_app!(ctx);
    let remove = |user_id: u64| {
        TestRequest::delete()
            .uri(&format!("/api/v1/events/{event}/members/{user_id}/"))
            .insert_header(bearer(creator))
    };

    assert_eq!(status_of!(app, remove(member)), 204);
    assert_eq!(status_of!(app, remove(member)), 404);
}

#[tokio::test]
#[serial]
async fn members_of_a_private_event_are_hidden_from_outsiders() {
    let ctx = TestContext::new().await;
    let creator = create_user(&ctx.db, "Creator", Some("User")).await;
    let member = create_user(&ctx.db, "Member", Some("User")).await;
    let outsider = create_user(&ctx.db, "Outsider", Some("User")).await;
    let event = ctx.event(creator, "Private").await;
    add_member(&ctx.db, event, member).await;
    let app = init_app!(ctx);
    let uri = format!("/api/v1/events/{event}/members/");

    assert_eq!(
        status_of!(
            app,
            TestRequest::get().uri(&uri).insert_header(bearer(outsider))
        ),
        403
    );
    let members: serde_json::Value = actix_web::test::call_and_read_body_json(
        &app,
        TestRequest::get()
            .uri(&uri)
            .insert_header(bearer(member))
            .to_request(),
    )
    .await;
    assert_eq!(members["members"][0]["id"], json!(member));
}
