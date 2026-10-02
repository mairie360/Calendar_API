use actix_web::test::{call_service, init_service, read_body, TestRequest};
use actix_web::{web, App};
use calendar_api::endpoints::ready::ready;
use mairie360_api_lib::state::AppState;
use mairie360_api_lib::test_setup::queries_setup::get_shared_db;
use mairie360_api_lib::test_setup::redis_setup::start_redis_container;
use serial_test::serial;

use super::{bearer, TestContext};
use crate::common::create_user;
use crate::{init_app, status_of};

/// Status and body of `GET /ready` for an app state on the given Redis and Postgres URLs.
async fn readiness(redis_url: &str, pg_url: &str) -> (u16, String) {
    let state = AppState::new(redis_url.to_string(), pg_url.to_string()).await;
    let app = init_service(App::new().app_data(web::Data::new(state)).service(ready)).await;
    let response = call_service(&app, TestRequest::get().uri("/ready").to_request()).await;
    let status = response.status().as_u16();
    let body = String::from_utf8(read_body(response).await.to_vec()).unwrap();
    (status, body)
}

#[tokio::test]
#[serial]
async fn ready_checks_postgres_and_redis() {
    let (_container, pg_url) = get_shared_db().await;
    let (_redis, redis) = start_redis_container().await;

    assert_eq!(
        readiness(&redis.url, pg_url).await,
        (200, "READY".to_string())
    );
    assert_eq!(
        readiness("redis://127.0.0.1:1", pg_url).await,
        (503, "Redis unavailable".to_string())
    );
    assert_eq!(
        readiness(
            &redis.url,
            "postgres://postgres:postgres@127.0.0.1:1/postgres"
        )
        .await,
        (503, "PostgreSQL unavailable".to_string())
    );
}

#[tokio::test]
#[serial]
async fn the_probes_are_not_mounted_under_api() {
    let ctx = TestContext::new().await;
    let user = create_user(&ctx.db, "Reader", Some("User")).await;
    let app = init_app!(ctx);

    for uri in ["/api/health", "/api/ready"] {
        assert_eq!(
            status_of!(app, TestRequest::get().uri(uri).insert_header(bearer(user))),
            404
        );
    }
}
