use actix_web::test::TestRequest;
use serde_json::json;
use serial_test::serial;

use super::{bearer, new_event_body, TestContext};
use crate::common::{add_member, archive_user, create_user};
use crate::{init_app, status_of};

#[tokio::test]
#[serial]
async fn requests_without_a_valid_token_are_refused() {
    let ctx = TestContext::new().await;
    let creator = create_user(&ctx.db, "Creator", Some("User")).await;
    let event = ctx.event(creator, "Public").await;
    let app = init_app!(ctx);

    let uri = format!("/api/v1/events/{event}/");
    assert_eq!(status_of!(app, TestRequest::get().uri(&uri)), 401);
    assert_eq!(
        status_of!(
            app,
            TestRequest::get()
                .uri(&uri)
                .insert_header(("Authorization", "Bearer not-a-jwt"))
        ),
        401
    );
}

#[tokio::test]
#[serial]
async fn an_archived_account_is_refused() {
    let ctx = TestContext::new().await;
    let creator = create_user(&ctx.db, "Creator", Some("User")).await;
    let user = create_user(&ctx.db, "Archived", Some("Admin")).await;
    let event = ctx.event(creator, "Public").await;
    let app = init_app!(ctx);
    let read = || {
        TestRequest::get()
            .uri(&format!("/api/v1/events/{event}/"))
            .insert_header(bearer(user))
    };

    assert_eq!(status_of!(app, read()), 200);
    archive_user(&ctx.db, user).await;
    // API_lib answers `JWTCheckError::UnknownUser` (404) for a token whose account is archived.
    assert_eq!(status_of!(app, read()), 404);
}

#[tokio::test]
#[serial]
async fn an_unknown_event_is_not_found() {
    let ctx = TestContext::new().await;
    let user = create_user(&ctx.db, "Reader", Some("Admin")).await;
    let app = init_app!(ctx);

    for request in [
        TestRequest::get().uri("/api/v1/events/2147483000/"),
        TestRequest::patch()
            .uri("/api/v1/events/2147483000/")
            .set_json(json!({ "name": "Nouveau nom" })),
        TestRequest::delete().uri("/api/v1/events/2147483000/"),
        TestRequest::get().uri("/api/v1/events/2147483000/members/"),
    ] {
        assert_eq!(status_of!(app, request.insert_header(bearer(user))), 404);
    }
}

#[tokio::test]
#[serial]
async fn a_malformed_event_id_is_refused() {
    let ctx = TestContext::new().await;
    let user = create_user(&ctx.db, "Reader", Some("User")).await;
    let app = init_app!(ctx);

    for uri in [
        "/api/v1/events/abc/",
        "/api/v1/events/-1/",
        "/api/v1/events/%2e%2e/",
    ] {
        let status = status_of!(app, TestRequest::get().uri(uri).insert_header(bearer(user)));
        assert!(status == 400 || status == 404, "{uri} answered {status}");
    }
}

#[tokio::test]
#[serial]
async fn another_user_cannot_read_edit_nor_delete_a_private_event() {
    let ctx = TestContext::new().await;
    let creator = create_user(&ctx.db, "Creator", Some("User")).await;
    let outsider = create_user(&ctx.db, "Outsider", Some("Responsable")).await;
    let event = ctx.event(creator, "Private").await;
    let app = init_app!(ctx);
    let uri = format!("/api/v1/events/{event}/");

    for request in [
        TestRequest::get().uri(&uri),
        TestRequest::patch()
            .uri(&uri)
            .set_json(json!({ "name": "Renommé" })),
        TestRequest::delete().uri(&uri),
        TestRequest::get().uri(&format!("{uri}members/")),
    ] {
        assert_eq!(
            status_of!(app, request.insert_header(bearer(outsider))),
            403
        );
    }

    // The creator, not yet a member, still reads it.
    assert_eq!(
        status_of!(
            app,
            TestRequest::get().uri(&uri).insert_header(bearer(creator))
        ),
        200
    );
}

#[tokio::test]
#[serial]
async fn a_public_event_is_readable_but_not_editable_by_everyone() {
    let ctx = TestContext::new().await;
    let creator = create_user(&ctx.db, "Creator", Some("User")).await;
    let outsider = create_user(&ctx.db, "Outsider", Some("User")).await;
    let event = ctx.event(creator, "Public").await;
    let app = init_app!(ctx);
    let uri = format!("/api/v1/events/{event}/");

    assert_eq!(
        status_of!(
            app,
            TestRequest::get().uri(&uri).insert_header(bearer(outsider))
        ),
        200
    );
    assert_eq!(
        status_of!(
            app,
            TestRequest::patch()
                .uri(&uri)
                .insert_header(bearer(outsider))
                .set_json(json!({ "name": "Renommé" }))
        ),
        403
    );
    assert_eq!(
        status_of!(
            app,
            TestRequest::delete()
                .uri(&uri)
                .insert_header(bearer(outsider))
        ),
        403
    );
}

#[tokio::test]
#[serial]
async fn a_member_who_is_not_the_creator_cannot_edit_nor_delete() {
    let ctx = TestContext::new().await;
    let creator = create_user(&ctx.db, "Creator", Some("User")).await;
    let member = create_user(&ctx.db, "Member", Some("User")).await;
    let event = ctx.event(creator, "Private").await;
    add_member(&ctx.db, event, member).await;
    let app = init_app!(ctx);
    let uri = format!("/api/v1/events/{event}/");

    assert_eq!(
        status_of!(
            app,
            TestRequest::get().uri(&uri).insert_header(bearer(member))
        ),
        200
    );
    assert_eq!(
        status_of!(
            app,
            TestRequest::patch()
                .uri(&uri)
                .insert_header(bearer(member))
                .set_json(json!({ "name": "Renommé" }))
        ),
        403
    );
    assert_eq!(
        status_of!(
            app,
            TestRequest::delete()
                .uri(&uri)
                .insert_header(bearer(member))
        ),
        403
    );
}

#[tokio::test]
#[serial]
async fn a_manager_member_edits_but_only_the_creator_deletes() {
    let ctx = TestContext::new().await;
    let creator = create_user(&ctx.db, "Creator", Some("User")).await;
    let manager = create_user(&ctx.db, "Manager", Some("Maire")).await;
    let event = ctx.event(creator, "Private").await;
    add_member(&ctx.db, event, manager).await;
    add_member(&ctx.db, event, creator).await;
    let app = init_app!(ctx);
    let uri = format!("/api/v1/events/{event}/");

    assert_eq!(
        status_of!(
            app,
            TestRequest::patch()
                .uri(&uri)
                .insert_header(bearer(manager))
                .set_json(json!({ "location": "Salle des fêtes" }))
        ),
        204
    );
    assert_eq!(
        status_of!(
            app,
            TestRequest::delete()
                .uri(&uri)
                .insert_header(bearer(manager))
        ),
        403
    );
    assert_eq!(
        status_of!(
            app,
            TestRequest::delete()
                .uri(&uri)
                .insert_header(bearer(creator))
        ),
        204
    );
    assert_eq!(
        status_of!(
            app,
            TestRequest::get().uri(&uri).insert_header(bearer(creator))
        ),
        404
    );
}

#[tokio::test]
#[serial]
async fn write_bodies_are_validated() {
    let ctx = TestContext::new().await;
    let creator = create_user(&ctx.db, "Creator", Some("User")).await;
    let event = ctx.event(creator, "Public").await;
    add_member(&ctx.db, event, creator).await;
    let app = init_app!(ctx);
    let uri = format!("/api/v1/events/{event}/");

    let mut unknown_field = new_event_body("Public");
    unknown_field["colour"] = json!("red");
    let mut reversed = new_event_body("Public");
    reversed["events_end_time"] = reversed["events_start_time"].clone();
    for body in [unknown_field, reversed] {
        assert_eq!(
            status_of!(
                app,
                TestRequest::post()
                    .uri("/api/v1/events/")
                    .insert_header(bearer(creator))
                    .set_json(body)
            ),
            400
        );
    }
    for body in [
        json!({ "event_name": "Faute de frappe" }),
        json!({ "name": " " }),
    ] {
        assert_eq!(
            status_of!(
                app,
                TestRequest::patch()
                    .uri(&uri)
                    .insert_header(bearer(creator))
                    .set_json(body)
            ),
            400
        );
    }
}
