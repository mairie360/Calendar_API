use calendar_api::database::event::{
    create::view::CreateEventByUserQueryView,
    get::view::{GetEventQueryResultView, GetEventQueryView},
};
use chrono::Utc;
use mairie360_api_lib::database::db_interface::Database;
use mairie360_api_lib::database::error::DbError;
use mairie360_api_lib::test_setup::queries_setup::get_shared_db;
use serial_test::serial;

#[tokio::test]
#[serial]
async fn test_get_event_success() {
    let (_container, host) = get_shared_db().await;
    let db = Database::new(host).await;

    let start = Utc::now();
    let end = start + chrono::Duration::days(1);

    let view =
        CreateEventByUserQueryView::new("Test Event", Some("Description"), start, end, 1, None, 1);

    let id = db.fetch_scalar::<i32, _>(&view).await.unwrap();

    let view = GetEventQueryView::new(id as u64);
    let event = db
        .fetch_one::<GetEventQueryResultView, _>(&view)
        .await
        .expect("event should exist");

    assert_eq!(event.name(), "Test Event");
    assert_eq!(event.description(), Some("Description"));
    assert_eq!(event.created_by(), Some(1));
    assert_eq!(event.recurrence_id(), None);
    assert_eq!(
        event.start_date().timestamp_millis(),
        start.timestamp_millis()
    );
    assert_eq!(event.end_date().timestamp_millis(), end.timestamp_millis());
}

#[tokio::test]
#[serial]
async fn test_get_event_not_found() {
    let (_container, host) = get_shared_db().await;
    let db = Database::new(host).await;

    let view = GetEventQueryView::new(99999);
    let result = db.fetch_one::<GetEventQueryResultView, _>(&view).await;

    assert!(matches!(result, Err(DbError::NotFound)));
}
