use actix_web::test::{call_and_read_body_json, TestRequest};
use chrono::{Duration, SecondsFormat, Utc};
use serde_json::Value;
use serial_test::serial;

use super::{bearer, TestContext};
use crate::common::create_user;
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
