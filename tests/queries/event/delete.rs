use calendar_api::database::event::{
    create::view::CreateEventByUserQueryView,
    delete::view::DeleteEventQueryView,
    get::view::{GetEventQueryResultView, GetEventQueryView},
};
use chrono::Utc;
use mairie360_api_lib::database::db_interface::Database;
use mairie360_api_lib::database::error::DbError;
use mairie360_api_lib::test_setup::queries_setup::get_shared_db;
use serial_test::serial;

#[tokio::test]
#[serial]
async fn test_delete_event_success() {
    let (_container, host) = get_shared_db().await;
    let db = Database::new(host).await;

    let start = Utc::now();
    let end = start + chrono::Duration::hours(1);

    let view =
        CreateEventByUserQueryView::new("Test Event", Some("Description"), start, end, 1, None, 1);
    let id = db.fetch_scalar::<i32, _>(&view).await.unwrap() as u64;

    let view = DeleteEventQueryView::new(id);
    assert!(db.execute(&view).await.is_ok());

    let view = GetEventQueryView::new(id);
    let result = db.fetch_one::<GetEventQueryResultView, _>(&view).await;
    assert!(matches!(result, Err(DbError::NotFound)));
}

#[tokio::test]
#[serial]
async fn test_delete_event_not_found() {
    let (_container, host) = get_shared_db().await;
    let db = Database::new(host).await;

    // Supprimer un événement inexistant est un no-op sans erreur.
    let view = DeleteEventQueryView::new(99999);
    assert!(db.execute(&view).await.is_ok());

    let view = GetEventQueryView::new(99999);
    let result = db.fetch_one::<GetEventQueryResultView, _>(&view).await;
    assert!(matches!(result, Err(DbError::NotFound)));
}
