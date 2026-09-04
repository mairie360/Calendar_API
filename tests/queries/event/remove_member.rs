use calendar_api::database::event::{
    add_member::view::AddUserToEventQueryView, create::view::CreateEventByUserQueryView,
    remove_member::view::RemoveUserFromEventQueryView,
};
use chrono::Utc;
use mairie360_api_lib::database::db_interface::Database;
use mairie360_api_lib::database::error::DbError;
use mairie360_api_lib::test_setup::queries_setup::get_shared_db;
use serial_test::serial;

async fn create_event(db: &Database) -> u64 {
    let start = Utc::now();
    let end = start + chrono::Duration::hours(1);
    let view =
        CreateEventByUserQueryView::new("Test Event", Some("Description"), start, end, 1, None, 1);
    db.fetch_scalar::<i32, _>(&view).await.unwrap() as u64
}

#[tokio::test]
#[serial]
async fn test_remove_member_success() {
    let (_container, host) = get_shared_db().await;
    let db = Database::new(host).await;

    let id = create_event(&db).await;

    let view = AddUserToEventQueryView::new(2, id);
    assert!(db.execute(&view).await.is_ok());

    let view = RemoveUserFromEventQueryView::new(2, id);
    let removed = db.fetch_scalar::<i32, _>(&view).await.unwrap();
    assert_eq!(removed, 2);
}

#[tokio::test]
#[serial]
async fn test_remove_member_event_not_found() {
    let (_container, host) = get_shared_db().await;
    let db = Database::new(host).await;

    let view = RemoveUserFromEventQueryView::new(1, 99999);
    let result = db.fetch_scalar::<i32, _>(&view).await;
    assert!(matches!(result, Err(DbError::NotFound)));
}

#[tokio::test]
#[serial]
async fn test_remove_member_user_not_found() {
    let (_container, host) = get_shared_db().await;
    let db = Database::new(host).await;

    let id = create_event(&db).await;

    let view = RemoveUserFromEventQueryView::new(999, id);
    let result = db.fetch_scalar::<i32, _>(&view).await;
    assert!(matches!(result, Err(DbError::NotFound)));
}
