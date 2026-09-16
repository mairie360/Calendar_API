use crate::common::event_input;
use calendar_api::database::calendar::get::view::{Event, GetCalendarQueryView};
use calendar_api::database::event::create::view::CreateEventQueryView;
use chrono::{Duration, Utc};
use mairie360_api_lib::database::db_interface::Database;
use mairie360_api_lib::test_setup::queries_setup::get_shared_db;
use serial_test::serial;

#[tokio::test]
#[serial]
async fn test_get_calendar_scoped_to_user() {
    let (_container, host) = get_shared_db().await;
    let db = Database::new(host).await;

    let start = Utc::now();
    let end = start + Duration::hours(1);

    let create = CreateEventQueryView::new(1, &event_input("Alice Only", None, start, end));
    let id = db.fetch_scalar::<i32, _>(&create).await.unwrap();

    let window_start = start - Duration::days(1);
    let window_end = end + Duration::days(1);

    let alice_view = GetCalendarQueryView::new(window_start, window_end, 1);
    let alice_events = db.fetch_all::<Event, _>(&alice_view).await.unwrap();
    assert!(alice_events.iter().any(|e| e.id() == id));

    let bob_view = GetCalendarQueryView::new(window_start, window_end, 2);
    let bob_events = db.fetch_all::<Event, _>(&bob_view).await.unwrap();
    assert!(!bob_events.iter().any(|e| e.id() == id));
}

#[tokio::test]
#[serial]
async fn test_get_calendar_empty_window() {
    let (_container, host) = get_shared_db().await;
    let db = Database::new(host).await;

    let far_past = Utc::now() - Duration::days(3650);
    let view = GetCalendarQueryView::new(far_past, far_past + Duration::hours(1), 1);
    let events = db.fetch_all::<Event, _>(&view).await.unwrap();

    assert!(events.is_empty());
}
