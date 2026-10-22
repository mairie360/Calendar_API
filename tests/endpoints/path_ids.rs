//! A path id that is not an integer answers the documented `400 text/plain` on every operation
//! (MAIR-262): the routes answered actix's default `404` while the contract documents `400`.
//!
//! Driven by the published contract (`ApiDoc`), so a new operation with a path id is covered as
//! soon as it is documented.

use actix_web::http::header::CONTENT_TYPE;
use actix_web::http::Method;
use actix_web::test::{try_call_service, TestRequest};
use calendar_api::endpoints::swagger::ApiDoc;
use serde_json::json;
use serial_test::serial;
use utoipa::OpenApi;

use super::{bearer, TestContext};
use crate::common::create_user;
use crate::init_app;

/// Every published operation with a path parameter, as `(method, uri, documented statuses)`,
/// each path parameter replaced by `abc`.
fn operations_with_a_path_id() -> Vec<(Method, String, Vec<String>)> {
    let document = serde_json::to_value(ApiDoc::openapi()).unwrap();
    let mut operations = Vec::new();
    for (template, methods) in document["paths"].as_object().unwrap() {
        if !template.contains('{') {
            continue;
        }
        let uri = template
            .split('/')
            .map(|segment| {
                if segment.starts_with('{') {
                    "abc"
                } else {
                    segment
                }
            })
            .collect::<Vec<_>>()
            .join("/");
        for (method, operation) in methods.as_object().unwrap() {
            let statuses = operation["responses"]
                .as_object()
                .map(|responses| responses.keys().cloned().collect())
                .unwrap_or_default();
            operations.push((
                Method::from_bytes(method.to_uppercase().as_bytes()).unwrap(),
                uri.clone(),
                statuses,
            ));
        }
    }
    operations
}

#[actix_web::test]
#[serial]
async fn a_path_id_that_is_not_an_integer_answers_the_documented_400() {
    let ctx = TestContext::new().await;
    let app = init_app!(ctx);
    let caller = create_user(&ctx.db, "Path", None).await;

    let operations = operations_with_a_path_id();
    assert!(operations.len() >= 6, "{operations:?}");
    let mut failures = Vec::new();
    for (method, uri, documented) in operations {
        if !documented.iter().any(|status| status == "400") {
            failures.push(format!("{method} {uri}: 400 is not documented"));
        }
        // A valid body, so that only the path can be refused.
        let req = TestRequest::default()
            .method(method.clone())
            .uri(&uri)
            .insert_header(bearer(caller))
            .set_json(json!({ "name": "abc", "user_id": caller, "status": "approved" }))
            .to_request();
        let (status, content_type, body) = match try_call_service(&app, req).await {
            Ok(resp) => {
                let content_type = resp.headers().get(CONTENT_TYPE).cloned();
                let status = resp.status();
                let body = actix_web::test::read_body(resp).await;
                (
                    status,
                    content_type,
                    String::from_utf8_lossy(&body).to_string(),
                )
            }
            Err(err) => {
                let resp = err.error_response();
                let content_type = resp.headers().get(CONTENT_TYPE).cloned();
                (
                    err.as_response_error().status_code(),
                    content_type,
                    String::new(),
                )
            }
        };
        let content_type = content_type
            .map(|v| v.to_str().unwrap().to_string())
            .unwrap_or_default();
        if status.as_u16() != 400 || !content_type.starts_with("text/plain") {
            failures.push(format!("{method} {uri}: {status} {content_type} {body}"));
        } else if body.contains("abc") {
            failures.push(format!("{method} {uri}: the value is echoed: {body}"));
        }
    }
    assert!(failures.is_empty(), "{}", failures.join("\n"));
}
