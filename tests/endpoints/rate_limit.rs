use actix_governor::Governor;
use actix_web::test::{call_service, init_service, TestRequest};
use actix_web::{web, App};
use calendar_api::endpoints::rate_limit::RateLimit;
use serial_test::serial;

use super::{bearer, TestContext};
use crate::common::create_user;

#[tokio::test]
#[serial]
async fn each_user_has_their_own_quota() {
    let ctx = TestContext::new().await;
    let first = create_user(&ctx.db, "First", Some("User")).await;
    let second = create_user(&ctx.db, "Second", Some("User")).await;
    let event = ctx.event(first, "Public").await;
    let governor = RateLimit {
        per_second: 1,
        burst: 2,
    }
    .governor_config();
    // Same order as main.rs: JwtMiddleware (last wrap) runs before the limiter.
    let app = init_service(
        App::new().app_data(ctx.state.clone()).service(
            web::scope("/api")
                .wrap(Governor::new(&governor))
                .wrap(mairie360_api_lib::security::JwtMiddleware)
                .configure(calendar_api::endpoints::config),
        ),
    )
    .await;
    let read = |user: u64| {
        TestRequest::get()
            .uri(&format!("/api/v1/events/{event}/"))
            .insert_header(bearer(user))
            .to_request()
    };

    assert_eq!(call_service(&app, read(first)).await.status(), 200);
    assert_eq!(call_service(&app, read(first)).await.status(), 200);
    let limited = call_service(&app, read(first)).await;
    assert_eq!(limited.status(), 429);
    assert!(limited.headers().contains_key("retry-after"));

    // Another user is not affected by the first one's burst.
    assert_eq!(call_service(&app, read(second)).await.status(), 200);
}
