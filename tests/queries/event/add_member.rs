use crate::common::{api_db, event_input};
use calendar_api::database::event::{
    add_member::view::AddUserToEventQueryView,
    create::view::CreateEventQueryView,
    get::view::{GetEventQueryResultView, GetEventQueryView},
};
use chrono::Utc;
use mairie360_api_lib::database::error::DbError;
use serial_test::serial;

#[tokio::test]
#[serial]
async fn test_add_member_success() {
    let db = api_db().await;

    let start = Utc::now();
    let end = start + chrono::Duration::hours(1);

    let view = CreateEventQueryView::new(
        1,
        &event_input("Test Event", Some("Description"), start, end),
    );
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
    let db = api_db().await;

    let view = AddUserToEventQueryView::new(1, 99999);
    let result = db.execute(&view).await;

    assert!(matches!(result, Err(DbError::ForeignKeyViolation(_))));
}
