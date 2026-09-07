use calendar_api::database::calendar::get::view::{Event, GetCalendarQueryView};
use calendar_api::database::event::create::view::CreateEventByUserQueryView;
use chrono::{Duration, Utc};
use mairie360_api_lib::database::db_interface::{ApiRequestDto, Database};
use mairie360_api_lib::test_setup::queries_setup::get_shared_db;
use serial_test::serial;

#[test]
fn test_query_view_getters() {
    let start = Utc::now();
    let end = start + Duration::days(2);
    let view = GetCalendarQueryView::new(start, end, 42);

    assert_eq!(view.start().timestamp_millis(), start.timestamp_millis());
    assert_eq!(view.end().timestamp_millis(), end.timestamp_millis());
    assert_eq!(view.user_id(), 42);
    assert_eq!(view.query_params().len(), 3);
    assert!(view.query_sql().contains("FROM events"));
    assert!(format!("{view}").contains("user_id=42"));
}

#[test]
fn test_event_getters_and_display() {
    let start = Utc::now();
    let end = start + Duration::hours(3);
    let event = Event::new(7, "Stand-up", start, end);

    assert_eq!(event.id(), 7);
    assert_eq!(event.name(), "Stand-up");
    assert_eq!(*event.start_date(), start);
    assert_eq!(*event.end_date(), end);
    assert!(format!("{event}").contains("Stand-up"));
}

#[tokio::test]
#[serial]
async fn test_get_calendar_returns_owned_event() {
    let (_container, host) = get_shared_db().await;
    let db = Database::new(host).await;

    let start = Utc::now();
    let end = start + Duration::hours(2);

    let create =
        CreateEventByUserQueryView::new("Calendar Event", Some("desc"), start, end, 1, None, 1);
    let id = db.fetch_scalar::<i32, _>(&create).await.unwrap();

    let view = GetCalendarQueryView::new(start - Duration::days(1), end + Duration::days(1), 1);
    let events = db.fetch_all::<Event, _>(&view).await.expect("query ok");

    assert!(events
        .iter()
        .any(|e| e.id() == id && e.name() == "Calendar Event"));
}

#[tokio::test]
#[serial]
async fn test_get_calendar_excludes_out_of_window_event() {
    let (_container, host) = get_shared_db().await;
    let db = Database::new(host).await;

    let start = Utc::now() + Duration::days(30);
    let end = start + Duration::hours(1);

    let create = CreateEventByUserQueryView::new("Far Away", None, start, end, 1, None, 1);
    let id = db.fetch_scalar::<i32, _>(&create).await.unwrap();

    let view = GetCalendarQueryView::new(Utc::now() - Duration::days(1), Utc::now(), 1);
    let events = db.fetch_all::<Event, _>(&view).await.expect("query ok");

    assert!(!events.iter().any(|e| e.id() == id));
}
