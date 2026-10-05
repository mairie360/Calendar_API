use actix_web::test::TestRequest;
use calendar_api::database::event::get::view::{GetEventQueryResultView, GetEventQueryView};
use serde_json::json;
use serial_test::serial;

use super::{bearer, TestContext};
use crate::common::{add_member, create_user, recurrence_rule_count};
use crate::{init_app, status_of};

async fn recurrence_id(ctx: &TestContext, event: u64) -> Option<i32> {
    let rows: Vec<GetEventQueryResultView> = ctx
        .db
        .fetch_all(&GetEventQueryView::new(event))
        .await
        .unwrap();
    rows[0].recurrence_id()
}

#[tokio::test]
#[serial]
async fn dropping_the_recurrence_deletes_the_rule_with_the_update() {
    let ctx = TestContext::new().await;
    let creator = create_user(&ctx.db, "Creator", Some("Maire")).await;
    let event = ctx.event(creator, "Public").await;
    add_member(&ctx.db, event, creator).await;
    let app = init_app!(ctx);
    let uri = format!("/api/v1/events/{event}/");
    let patch = |body| {
        TestRequest::patch()
            .uri(&uri)
            .insert_header(bearer(creator))
            .set_json(body)
    };

    assert_eq!(
        status_of!(
            app,
            patch(json!({ "recurrence": { "frequency": "weekly", "interval": 1 } }))
        ),
        204
    );
    let rule = recurrence_id(&ctx, event).await.expect("rule created");
    assert_eq!(recurrence_rule_count(&ctx.db, rule).await, 1);

    assert_eq!(status_of!(app, patch(json!({ "recurrence": null }))), 204);
    assert_eq!(recurrence_id(&ctx, event).await, None);
    assert_eq!(recurrence_rule_count(&ctx.db, rule).await, 0);
}

#[tokio::test]
#[serial]
async fn a_refused_write_changes_nothing() {
    let ctx = TestContext::new().await;
    let creator = create_user(&ctx.db, "Creator", Some("User")).await;
    let event = ctx.event(creator, "Public").await;
    add_member(&ctx.db, event, creator).await;
    let app = init_app!(ctx);

    // A duplicate assignment fails inside the transaction: it is rolled back and the event row
    // lock released, so the next write on the same event goes through.
    let assign = || {
        TestRequest::post()
            .uri(&format!("/api/v1/events/{event}/members/"))
            .insert_header(bearer(creator))
            .set_json(json!({ "user_id": creator }))
    };
    assert_eq!(status_of!(app, assign()), 409);
    assert_eq!(
        status_of!(
            app,
            TestRequest::delete()
                .uri(&format!("/api/v1/events/{event}/"))
                .insert_header(bearer(creator))
        ),
        204
    );
}
