//! MAIR-524: the calendar query (members aggregate included) stays on indexes, and every write
//! view that changes a calendar page invalidates its read cache.

use calendar_api::database::calendar::get::view::{
    GetCalendarQueryView, CALENDAR_CACHE, CALENDAR_CACHE_NAMESPACE,
};
use calendar_api::database::event::access::view::ApprovalStatus;
use calendar_api::database::event::add_member::view::AddUserToEventQueryView;
use calendar_api::database::event::create::view::CreateEventQueryView;
use calendar_api::database::event::delete::view::DeleteEventQueryView;
use calendar_api::database::event::edit::view::{
    DeleteOrphanRecurrenceQueryView, EditEventQueryView,
};
use calendar_api::database::event::remove_member::view::RemoveUserFromEventQueryView;
use calendar_api::database::event::validation::view::SetEventApprovalQueryView;
use chrono::{Duration, Utc};
use mairie360_api_lib::database::db_interface::{ApiRequestDto, QueryParam};
use serde_json::Value;
use serial_test::serial;

use crate::common::{add_member, api_db, create_event, create_user, event_input, fixtures_db};

#[derive(serde::Serialize, serde::Deserialize)]
struct RawSql {
    #[serde(skip)]
    sql: &'static str,
    params: Vec<QueryParam>,
}

impl ApiRequestDto for RawSql {
    fn query_sql(&self) -> &'static str {
        self.sql
    }

    fn query_params(&self) -> &[QueryParam] {
        &self.params
    }
}

/// Every `(Node Type, Relation Name)` of an `EXPLAIN (FORMAT JSON)` plan.
fn scans(node: &Value, out: &mut Vec<(String, String)>) {
    if let Some(kind) = node["Node Type"].as_str() {
        out.push((
            kind.to_string(),
            node["Relation Name"]
                .as_str()
                .unwrap_or_default()
                .to_string(),
        ));
    }
    if let Some(children) = node["Plans"].as_array() {
        for child in children {
            scans(child, out);
        }
    }
}

#[tokio::test]
#[serial]
async fn mair_524_the_calendar_query_reads_events_and_members_through_indexes() {
    let db = api_db().await;
    let fixtures = fixtures_db().await;
    let user = create_user(&fixtures, "Planner", Some("User")).await;
    let start = Utc::now();
    let event = create_event(
        &db,
        user,
        &event_input("Bureau municipal", None, start, start + Duration::hours(1)),
    )
    .await;
    add_member(&fixtures, event, user).await;
    let view =
        GetCalendarQueryView::new(start - Duration::days(1), start + Duration::days(30), user);

    // The test tables are tiny: forbid sequential scans so the planner shows whether an index
    // can serve each lookup at all (a missing index leaves a Seq Scan even then).
    let mut tx = db.begin().await.unwrap();
    tx.execute(&RawSql {
        sql: "SET LOCAL enable_seqscan = off",
        params: Vec::new(),
    })
    .await
    .unwrap();
    let explain: Value = tx
        .fetch_one(&RawSql {
            sql: Box::leak(format!("EXPLAIN (FORMAT JSON) {}", view.query_sql()).into_boxed_str()),
            params: view.query_params().to_vec(),
        })
        .await
        .unwrap();
    tx.rollback().await.unwrap();

    let mut nodes = Vec::new();
    scans(&explain[0]["Plan"], &mut nodes);
    let indexed = |relation: &str| {
        nodes.iter().any(|(kind, rel)| {
            rel == relation
                && matches!(
                    kind.as_str(),
                    "Index Scan" | "Index Only Scan" | "Bitmap Heap Scan"
                )
        })
    };
    let seq_scanned = |relation: &str| {
        nodes
            .iter()
            .any(|(kind, rel)| rel == relation && kind == "Seq Scan")
    };
    for relation in ["events", "event_members"] {
        assert!(indexed(relation), "{relation} not read by index: {nodes:?}");
        assert!(!seq_scanned(relation), "{relation} scanned: {nodes:?}");
    }
}

#[test]
fn mair_524_every_calendar_write_invalidates_the_calendar_cache() {
    let input = event_input(
        "Vœux du maire",
        None,
        Utc::now(),
        Utc::now() + Duration::hours(2),
    );
    let writes: Vec<(&str, &'static [&'static str])> = vec![
        (
            "create",
            CreateEventQueryView::new(1, &input).invalidated_read_caches(),
        ),
        (
            "edit",
            EditEventQueryView::new(1, &input, false).invalidated_read_caches(),
        ),
        (
            "orphan recurrence",
            DeleteOrphanRecurrenceQueryView::new(1).invalidated_read_caches(),
        ),
        (
            "delete",
            DeleteEventQueryView::new(1).invalidated_read_caches(),
        ),
        (
            "add member",
            AddUserToEventQueryView::new(1, 1).invalidated_read_caches(),
        ),
        (
            "remove member",
            RemoveUserFromEventQueryView::new(1, 1).invalidated_read_caches(),
        ),
        (
            "approval",
            SetEventApprovalQueryView::new(1, 1, ApprovalStatus::Approved)
                .invalidated_read_caches(),
        ),
    ];
    for (name, namespaces) in writes {
        assert!(
            namespaces.contains(&CALENDAR_CACHE_NAMESPACE),
            "{name} does not invalidate the calendar"
        );
    }
    // The read itself invalidates nothing, and the cache is the documented one.
    assert!(GetCalendarQueryView::new(Utc::now(), Utc::now(), 1)
        .invalidated_read_caches()
        .is_empty());
    assert_eq!(CALENDAR_CACHE.namespace(), "calendar");
    assert_eq!(CALENDAR_CACHE.ttl(), 30);
}
