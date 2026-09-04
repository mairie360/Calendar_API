use calendar_api::database::event::{
    add_member::view::AddUserToEventQueryView,
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
async fn test_add_member_success() {
    let (_container, host) = get_shared_db().await;
    let db = Database::new(host).await;

    let start = Utc::now();
    let end = start + chrono::Duration::hours(1);

    let view =
        CreateEventByUserQueryView::new("Test Event", Some("Description"), start, end, 1, None, 1);
    let id = db.fetch_scalar::<i32, _>(&view).await.unwrap() as u64;

    let view = AddUserToEventQueryView::new(2, id);
    assert!(db.execute(&view).await.is_ok());

    let view = GetEventQueryView::new(id);
    assert!(db
        .fetch_one::<GetEventQueryResultView, _>(&view)
        .await
        .is_ok());
}

#[tokio::test]
#[serial]
async fn test_add_member_event_not_found() {
    let (_container, host) = get_shared_db().await;
    let db = Database::new(host).await;

    let view = AddUserToEventQueryView::new(1, 99999);
    let result = db.execute(&view).await;

    assert!(matches!(result, Err(DbError::ForeignKeyViolation(_))));
}
