use calendar_api::database::calendar::get::view::{Event, GetCalendarQueryView};
use calendar_api::database::event::add_member::view::AddUserToEventQueryView;
use calendar_api::database::event::create::view::CreateEventByUserQueryView;
use chrono::{Duration, Utc};
use mairie360_api_lib::database::db_interface::Database;
use mairie360_api_lib::test_setup::queries_setup::get_shared_db;
use serial_test::serial;

#[tokio::test]
#[serial]
async fn test_member_sees_event_without_owning_it() {
    let (_container, host) = get_shared_db().await;
    let db = Database::new(host).await;

    let start = Utc::now();
    let end = start + Duration::hours(1);

    // Owned by Alice (1), Bob (2) is added as a member.
    let create = CreateEventByUserQueryView::new("Shared Meeting", None, start, end, 1, None, 1);
    let id = db.fetch_scalar::<i32, _>(&create).await.unwrap();

    let add = AddUserToEventQueryView::new(2, id as u64);
    db.execute(&add).await.unwrap();

    let view = GetCalendarQueryView::new(start - Duration::days(1), end + Duration::days(1), 2);
    let events = db.fetch_all::<Event, _>(&view).await.unwrap();

    assert!(events.iter().any(|e| e.id() == id));
}

#[tokio::test]
#[serial]
async fn test_owner_and_member_event_is_not_duplicated() {
    let (_container, host) = get_shared_db().await;
    let db = Database::new(host).await;

    let start = Utc::now();
    let end = start + Duration::hours(1);

    let create = CreateEventByUserQueryView::new("Self Member", None, start, end, 1, None, 1);
    let id = db.fetch_scalar::<i32, _>(&create).await.unwrap();

    // Alice is both owner and an explicit member.
    db.execute(&AddUserToEventQueryView::new(1, id as u64))
        .await
        .unwrap();

    let view = GetCalendarQueryView::new(start - Duration::days(1), end + Duration::days(1), 1);
    let events = db.fetch_all::<Event, _>(&view).await.unwrap();

    assert_eq!(events.iter().filter(|e| e.id() == id).count(), 1);
}
