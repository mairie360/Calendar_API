//! MAIR-288: `access-matrix.yaml` covers every operation of the `OpenAPI`, and the API answers as
//! it says. Each operation is called on a private event without a session, as a user of each role
//! who has no relation to it, and as its creator, a member and an assigned Responsable sharing a
//! group with the creator: an allowed caller never gets 401 / 403 / 404, any other caller gets 403.

use std::collections::{BTreeMap, BTreeSet};

use actix_web::test::TestRequest;
use calendar_api::endpoints::swagger::ApiDoc;
use serde::Deserialize;
use serde_json::{json, Value};
use serial_test::serial;
use utoipa::OpenApi;

use super::{bearer, TestContext};
use crate::common::{add_member, create_group, create_user};
use crate::{init_app, status_of};

const MATRIX: &str = include_str!("../../access-matrix.yaml");
const ROLES: [&str; 5] = ["Admin", "Maire", "Responsable", "User", "Guest"];
const RELATIONS: [&str; 3] = ["creator", "member", "responsable"];
/// The Admin seeded by the database migrations.
const ADMIN_ID: u64 = 1;

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct Matrix {
    version: u32,
    api: String,
    roles: Vec<String>,
    operations: BTreeMap<String, Operation>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct Operation {
    access: String,
    #[serde(default)]
    roles: Vec<String>,
    #[serde(default)]
    ownership: Vec<String>,
    #[serde(default)]
    personal: Vec<String>,
    #[allow(dead_code)]
    note: Option<String>,
}

fn matrix() -> Matrix {
    yaml_serde::from_str(MATRIX).expect("access-matrix.yaml is valid")
}

/// The spec the API serves (`ApiDoc`, the contract the BFF is generated from), as JSON.
fn spec() -> Value {
    serde_json::to_value(ApiDoc::openapi()).unwrap()
}

/// "METHOD /path" of every operation of the spec.
fn spec_operations(spec: &Value) -> BTreeSet<String> {
    let mut operations = BTreeSet::new();
    for (path, item) in spec["paths"].as_object().unwrap() {
        for method in ["get", "post", "put", "patch", "delete"] {
            if item.get(method).is_some() {
                operations.insert(format!("{} {path}", method.to_uppercase()));
            }
        }
    }
    operations
}

#[test]
fn the_matrix_covers_every_operation_of_the_spec() {
    let matrix = matrix();
    assert_eq!((matrix.version, matrix.api.as_str()), (1, "calendar"));
    assert_eq!(matrix.roles, ROLES);
    let spec = spec_operations(&spec());
    let listed: BTreeSet<String> = matrix.operations.keys().cloned().collect();
    let missing: Vec<_> = spec.difference(&listed).collect();
    let unknown: Vec<_> = listed.difference(&spec).collect();
    assert!(
        missing.is_empty(),
        "operations of the OpenAPI missing from access-matrix.yaml: {missing:?}"
    );
    assert!(
        unknown.is_empty(),
        "operations of access-matrix.yaml that the OpenAPI does not have: {unknown:?}"
    );
    for (name, op) in &matrix.operations {
        assert!(
            ["public", "authenticated"].contains(&op.access.as_str()),
            "{name}: access"
        );
        assert!(
            op.roles.iter().all(|r| ROLES.contains(&r.as_str())),
            "{name}: unknown role"
        );
        assert!(
            op.ownership.iter().all(|o| RELATIONS.contains(&o.as_str())),
            "{name}: ownership"
        );
        if op.access == "authenticated" {
            assert!(
                !op.roles.is_empty() || !op.ownership.is_empty(),
                "{name}: nobody may call it"
            );
        }
        assert!(
            op.personal.iter().all(|f| !f.is_empty()),
            "{name}: personal"
        );
    }
}

/// The people around one event, created once: the creator (a plain agent, so the event waits
/// for approval), a member, and a Responsable sharing a group with the creator. `outsider` is a
/// fresh account of that group, for the member additions.
struct People {
    creator: u64,
    member: u64,
    responsable: u64,
}

/// A fresh private event of the creator, with the member, the Responsable and an extra member.
struct Fixtures {
    event: u64,
    extra: u64,
}

async fn fixtures(ctx: &TestContext, people: &People) -> Fixtures {
    let event = ctx.event(people.creator, "Private").await;
    let extra = create_user(&ctx.db, "Extra", Some("User")).await;
    create_group(&ctx.db, people.creator, &[people.creator, extra]).await;
    for user in [people.member, people.responsable, extra] {
        add_member(&ctx.db, event, user).await;
    }
    Fixtures { event, extra }
}

/// The request of one operation on the fixtures: path and query parameters, and the example body
/// of the spec, made valid for the fixtures.
fn request(spec: &Value, name: &str, f: &Fixtures, joiner: u64) -> TestRequest {
    let (method, path) = name.split_once(' ').unwrap();
    let mut uri = path
        .replace("{event_id}", &f.event.to_string())
        .replace("{member_id}", &f.extra.to_string());
    if path == "/api/v1/calendar" {
        uri.push_str("?start=2026-01-01T00:00:00Z&end=2026-12-31T00:00:00Z");
    }
    let example = spec["paths"][path][method.to_lowercase()]["requestBody"]["content"]
        ["application/json"]["example"]
        .clone();
    let body = match name {
        "POST /api/v1/events/{event_id}/members/" => Some(json!({ "user_id": joiner })),
        _ if example.is_null() => None,
        _ => Some(example),
    };
    let request = match method {
        "GET" => TestRequest::get(),
        "POST" => TestRequest::post(),
        "PUT" => TestRequest::put(),
        "PATCH" => TestRequest::patch(),
        "DELETE" => TestRequest::delete(),
        other => panic!("{other}"),
    }
    .uri(&uri);
    match body {
        Some(body) => request.set_json(body),
        None => request,
    }
}

#[actix_web::test]
#[serial]
async fn every_role_and_relation_gets_what_the_matrix_says() {
    let ctx = TestContext::new().await;
    let app = init_app!(ctx);
    let spec = spec();
    let people = People {
        creator: create_user(&ctx.db, "Creator", Some("User")).await,
        member: create_user(&ctx.db, "Member", Some("User")).await,
        responsable: create_user(&ctx.db, "Responsable", Some("Responsable")).await,
    };
    create_group(
        &ctx.db,
        people.creator,
        &[people.creator, people.responsable],
    )
    .await;
    // Users of each role with no relation to the events (the schema also gives every account the
    // Guest role).
    let mut callers: Vec<(String, u64)> = vec![("Admin".to_string(), ADMIN_ID)];
    for role in &ROLES[1..] {
        let id = create_user(
            &ctx.db,
            &format!("Matrix{role}"),
            (*role != "Guest").then_some(*role),
        )
        .await;
        callers.push(((*role).to_string(), id));
    }

    let mut failures = Vec::new();
    for (name, op) in &matrix().operations {
        if op.access == "public" {
            continue;
        }
        let f = fixtures(&ctx, &people).await;
        let anonymous = status_of!(app, request(&spec, name, &f, f.extra));
        if anonymous != 401 {
            failures.push(format!(
                "{name}: got {anonymous} without a session, expected 401"
            ));
        }
        let mut cases: Vec<(String, u64, bool)> = callers
            .iter()
            .map(|(role, id)| (role.clone(), *id, op.roles.contains(role)))
            .collect();
        for (who, user, role) in [
            ("creator", people.creator, "User"),
            ("member", people.member, "User"),
            ("responsable", people.responsable, "Responsable"),
        ] {
            let allowed =
                op.roles.iter().any(|r| r == role) || op.ownership.iter().any(|o| o == who);
            cases.push((who.to_string(), user, allowed));
        }
        for (who, user, allowed) in cases {
            let f = fixtures(&ctx, &people).await;
            // A member addition needs a target inside the caller's scope: a fresh account of the
            // caller's own group with the creator.
            let joiner = create_user(&ctx.db, "Joiner", Some("User")).await;
            create_group(&ctx.db, people.creator, &[people.creator, user, joiner]).await;
            let got = status_of!(
                app,
                request(&spec, name, &f, joiner).insert_header(bearer(user))
            );
            let refused = got == 401 || got == 403 || got == 404;
            if allowed && refused {
                failures.push(format!(
                    "{name}: {who} is allowed by the matrix but got {got}"
                ));
            }
            if !allowed && got != 403 {
                failures.push(format!(
                    "{name}: {who} is not allowed by the matrix but got {got}, expected 403"
                ));
            }
        }
    }
    assert!(
        failures.is_empty(),
        "the API does not answer as access-matrix.yaml says:\n{}",
        failures.join("\n")
    );
}
