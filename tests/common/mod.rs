use std::sync::atomic::{AtomicU64, Ordering};

use calendar_api::database::event::create::view::CreateEventQueryView;
use calendar_api::database::event::model::{EventCategory, EventInput, EventVisibility};
use chrono::{DateTime, Utc};
use mairie360_api_lib::database::db_interface::{ApiRequestDto, Database, QueryParam};
use mairie360_api_lib::test_setup::queries_setup::{get_shared_db, get_shared_db_as};

/// Postgres role Calendar connects with in every deployment (Database `api_roles.sql`).
pub const CALENDAR_DB_ROLE: &str = "calendar_api";

/// URL of the shared test database as [`CALENDAR_DB_ROLE`] (MAIR-556): the application code
/// under test (`AppState`, query views) goes through it, so a query the role is not granted
/// fails here with `permission denied` instead of in staging.
pub async fn calendar_db_url() -> String {
    get_shared_db_as(CALENDAR_DB_ROLE).await
}

/// Client of the shared test database connected as [`CALENDAR_DB_ROLE`], for the query views
/// under test.
pub async fn api_db() -> Database {
    Database::new(&calendar_db_url().await).await
}

/// Client of the shared test database connected as the superuser of `get_shared_db()`, for the
/// fixtures only (users, roles, groups... that Calendar's role may not write). Never run the
/// code under test through it: a superuser ignores the grants.
pub async fn fixtures_db() -> Database {
    let (_container, superuser_url) = get_shared_db().await;
    Database::new(superuser_url).await
}

/// Minimal data of a public event, without metadata or recurrence.
pub fn event_input(
    name: &str,
    description: Option<&str>,
    start: DateTime<Utc>,
    end: DateTime<Utc>,
) -> EventInput {
    EventInput {
        name: name.to_string(),
        description: description.map(str::to_string),
        start,
        end,
        visibility: EventVisibility::Public,
        category: EventCategory::Other,
        service: None,
        location: None,
        recurrence: None,
    }
}

pub async fn create_event(db: &Database, creator_id: u64, input: &EventInput) -> u64 {
    db.fetch_scalar::<i32, _>(&CreateEventQueryView::new(creator_id, input))
        .await
        .unwrap() as u64
}

/// Test-only setup SQL statement (users, roles, groups), without parameters.
#[derive(serde::Serialize, serde::Deserialize)]
struct FixtureSql {
    #[serde(skip)]
    sql: &'static str,
    params: Vec<QueryParam>,
}

impl ApiRequestDto for FixtureSql {
    fn query_sql(&self) -> &'static str {
        self.sql
    }

    fn query_params(&self) -> &[QueryParam] {
        &self.params
    }
}

fn fixture(sql: String) -> FixtureSql {
    FixtureSql {
        sql: Box::leak(sql.into_boxed_str()),
        params: Vec::new(),
    }
}

/// Dummy argon2id PHC string: the users table only accepts hashed passwords (`chk_users_password_hashed`).
const TEST_PASSWORD_HASH: &str =
    "$argon2id$v=19$m=19456,t=2,p=1$c29tZXNhbHRzb21lc2FsdA$c29tZWhhc2hzb21laGFzaHNvbWVoYXNoc29tZWhhc2g";

fn unique_suffix() -> String {
    static COUNTER: AtomicU64 = AtomicU64::new(0);
    let nanos = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    format!("{nanos}{}", COUNTER.fetch_add(1, Ordering::Relaxed))
}

/// Creates a user (with a role when `role` is given) and returns its id.
pub async fn create_user(db: &Database, first_name: &str, role: Option<&str>) -> u64 {
    let id: i32 = db
        .fetch_scalar(&fixture(format!(
            "INSERT INTO users (first_name, last_name, email, password) \
             VALUES ('{first_name}', 'Test', '{}.{}@calendar-api.test', '{TEST_PASSWORD_HASH}') RETURNING id",
            first_name.to_lowercase(),
            unique_suffix()
        )))
        .await
        .unwrap();
    if let Some(role) = role {
        db.execute(&fixture(format!(
            "INSERT INTO user_roles (user_id, role_id) SELECT {id}, id FROM roles WHERE name = '{role}'"
        )))
        .await
        .unwrap();
    }
    id as u64
}

/// Creates a group containing `members`.
pub async fn create_group(db: &Database, owner_id: u64, members: &[u64]) {
    let id: i32 = db
        .fetch_scalar(&fixture(format!(
            "INSERT INTO groups (owner_id, name) VALUES ({owner_id}, 'Groupe {}') RETURNING id",
            unique_suffix()
        )))
        .await
        .unwrap();
    for member in members {
        db.execute(&fixture(format!(
            "INSERT INTO group_members (group_id, user_id) VALUES ({id}, {member}) ON CONFLICT DO NOTHING"
        )))
        .await
        .unwrap();
    }
}

/// Number of recurrence rules with the given id (0 or 1).
pub async fn recurrence_rule_count(db: &Database, rule_id: i32) -> i64 {
    db.fetch_scalar(&fixture(format!(
        "SELECT COUNT(*) FROM recurrence_rules WHERE id = {rule_id}"
    )))
    .await
    .unwrap()
}

/// Approval stored on the event: (`approval_status`, `approval_decided_by`, decision date set).
pub async fn approval_decision(db: &Database, event_id: u64) -> (String, Option<i32>, bool) {
    let row: serde_json::Value = db
        .fetch_one(&fixture(format!(
            "SELECT jsonb_build_object('status', approval_status, 'by', approval_decided_by, \
             'at', approval_decided_at IS NOT NULL) FROM events WHERE id = {event_id}"
        )))
        .await
        .unwrap();
    (
        row["status"].as_str().unwrap().to_string(),
        row["by"].as_i64().map(|id| id as i32),
        row["at"].as_bool().unwrap(),
    )
}

/// Archives a user: its tokens are then refused by `JwtMiddleware`.
pub async fn archive_user(db: &Database, user_id: u64) {
    db.execute(&fixture(format!(
        "UPDATE users SET is_archived = TRUE, status = 'archived' WHERE id = {user_id}"
    )))
    .await
    .unwrap();
}

/// Assigns `user_id` to `event_id` directly in the database.
pub async fn add_member(db: &Database, event_id: u64, user_id: u64) {
    db.execute(&fixture(format!(
        "INSERT INTO event_members (event_id, user_id) VALUES ({event_id}, {user_id})"
    )))
    .await
    .unwrap();
}

/// Runs a fixture statement directly in the database, behind the API's back (no cache
/// invalidation): the cache tests use it to tell a cached page from a fresh one.
pub async fn raw_sql(db: &Database, sql: String) {
    db.execute(&fixture(sql)).await.unwrap();
}

/// Number of events created by `creator_id`.
pub async fn created_event_count(db: &Database, creator_id: u64) -> i64 {
    db.fetch_scalar(&fixture(format!(
        "SELECT COUNT(*) FROM events WHERE created_by = {creator_id}"
    )))
    .await
    .unwrap()
}

/// Ids of the users assigned to `event_id`, sorted.
pub async fn member_ids(db: &Database, event_id: u64) -> Vec<u64> {
    let ids: serde_json::Value = db
        .fetch_one(&fixture(format!(
            "SELECT COALESCE(jsonb_agg(user_id ORDER BY user_id), '[]'::jsonb) \
             FROM event_members WHERE event_id = {event_id}"
        )))
        .await
        .unwrap();
    serde_json::from_value(ids).unwrap()
}
