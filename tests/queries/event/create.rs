use calendar_api::database::event::create::view::{
    CreateEventByGroupQueryView, CreateEventByUserQueryView, ReccurenceType, RecurrenceRule,
};
use chrono::Utc;
use mairie360_api_lib::database::db_interface::Database;
use mairie360_api_lib::test_setup::queries_setup::get_shared_db;
use serial_test::serial;

#[tokio::test]
#[serial]
async fn test_create_event_by_user_success() {
    let (_container, host) = get_shared_db().await;
    let db = Database::new(host).await;

    let start_date = Utc::now();
    let end_date = start_date + chrono::Duration::days(10);
    let view = CreateEventByUserQueryView::new(
        "Test Event",
        Some("Description"),
        start_date,
        end_date,
        1, // ID utilisateur admin (assure-toi qu'il existe en DB)
        None,
        1,
    );

    let result = db.fetch_scalar::<i32, _>(&view).await;

    assert!(result.is_ok());
    assert!(result.unwrap() > 0);
}

#[tokio::test]
#[serial]
async fn test_create_recurrent_event_by_user_success() {
    let (_container, host) = get_shared_db().await;
    let db = Database::new(host).await;

    let start_date = Utc::now();
    let end_date = start_date + chrono::Duration::days(10);
    let view = CreateEventByUserQueryView::new(
        "Test Event",
        Some("Description"),
        start_date,
        end_date,
        1,
        Some(RecurrenceRule::new(ReccurenceType::Daily, Some(1), None)),
        1,
    );

    let result = db.fetch_scalar::<i32, _>(&view).await;

    assert!(result.is_ok());
    assert!(result.unwrap() > 0);
}

#[tokio::test]
#[serial]
async fn test_create_event_by_group_success() {
    let (_container, host) = get_shared_db().await;
    let db = Database::new(host).await;

    let start_date = Utc::now();
    let end_date = start_date + chrono::Duration::days(10);
    let view = CreateEventByGroupQueryView::new(
        "Test Event".to_string(),
        Some("Description".to_string()),
        start_date,
        end_date,
        1,
        None,
        1,
    );

    let result = db.fetch_scalar::<i32, _>(&view).await;

    assert!(result.is_ok());
    assert!(result.unwrap() > 0);
}

#[tokio::test]
#[serial]
async fn test_create_recurrent_event_by_group() {
    let (_container, host) = get_shared_db().await;
    let db = Database::new(host).await;

    let start_date = Utc::now();
    let end_date = start_date + chrono::Duration::days(10);
    let view = CreateEventByGroupQueryView::new(
        "Test Event".to_string(),
        Some("Description".to_string()),
        start_date,
        end_date,
        1,
        Some(RecurrenceRule::new(ReccurenceType::Daily, Some(1), None)),
        1,
    );

    let result = db.fetch_scalar::<i32, _>(&view).await;

    assert!(result.is_ok());
    assert!(result.unwrap() > 0);
}
