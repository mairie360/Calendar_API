use actix_governor::Governor;
use actix_web::test::{call_service, init_service, TestRequest};
use actix_web::{web, App};
use calendar_api::endpoints::rate_limit::{
    RateLimit, DEFAULT_RATE_LIMIT_BURST, DEFAULT_RATE_LIMIT_PER_SECOND,
};
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

/// The production budget (MAIR-474): `RateLimit::from_env()` without overrides, mounted like
/// `main.rs`. A user gets the whole burst, then `429` with a numeric `Retry-After` of at least one
/// second, while another user is still served.
#[tokio::test]
#[serial]
async fn the_production_budget_refuses_a_user_past_its_burst() {
    std::env::remove_var(calendar_api::endpoints::rate_limit::RATE_LIMIT_PER_SECOND_ENV);
    std::env::remove_var(calendar_api::endpoints::rate_limit::RATE_LIMIT_BURST_ENV);
    let quota = RateLimit::from_env().expect("the limiter is on by default");
    assert_eq!(
        quota,
        RateLimit {
            per_second: DEFAULT_RATE_LIMIT_PER_SECOND,
            burst: DEFAULT_RATE_LIMIT_BURST,
        }
    );
    let ctx = TestContext::new().await;
    let greedy = create_user(&ctx.db, "Greedy", Some("User")).await;
    let other = create_user(&ctx.db, "Other", Some("User")).await;
    let event = ctx.event(greedy, "Public").await;
    let governor = quota.governor_config();
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

    let started = std::time::Instant::now();
    let mut served: u64 = 0;
    let limited = loop {
        let response = call_service(&app, read(greedy)).await;
        if response.status() != 200 {
            break response;
        }
        served += 1;
        assert!(served < 10_000, "the limiter never refused");
    };
    // The bucket refills while the burst is sent.
    let refilled = u64::try_from(started.elapsed().as_millis()).unwrap()
        * DEFAULT_RATE_LIMIT_PER_SECOND
        / 1000
        + 1;
    let burst = u64::from(DEFAULT_RATE_LIMIT_BURST);
    assert!(
        served >= burst && served <= burst + refilled,
        "{served} requests served before the 429"
    );
    assert_eq!(limited.status(), 429);
    let retry_after: u64 = limited
        .headers()
        .get("retry-after")
        .unwrap()
        .to_str()
        .unwrap()
        .parse()
        .unwrap();
    assert!(retry_after >= 1, "Retry-After: {retry_after}");
    assert_eq!(
        limited.headers().get("x-ratelimit-after"),
        limited.headers().get("retry-after")
    );

    assert_eq!(call_service(&app, read(other)).await.status(), 200);
}
