use calendar_api::database::event::{
    add_member::view::AddUserToEventQueryView,
    create::view::CreateEventByUserQueryView,
    get_event_members::view::{GetEventMemberQueryView, Member},
};
use chrono::Utc;
use mairie360_api_lib::database::db_interface::Database;
use mairie360_api_lib::test_setup::queries_setup::get_shared_db;
use serial_test::serial;

#[tokio::test]
#[serial]
async fn test_get_event_members_success() {
    let (_container, host) = get_shared_db().await;
    let db = Database::new(host).await;

    let start = Utc::now();
    let end = start + chrono::Duration::hours(1);

    let view =
        CreateEventByUserQueryView::new("Test Event", Some("Description"), start, end, 1, None, 1);
    let id = db.fetch_scalar::<i32, _>(&view).await.unwrap() as u64;

    let view = AddUserToEventQueryView::new(2, id);
    assert!(db.execute(&view).await.is_ok());

    let view = GetEventMemberQueryView::new(id);
    let members = db.fetch_all::<Member, _>(&view).await.unwrap();

    assert!(!members.is_empty());
    assert_eq!(members[0].user_id(), 2);
}

#[tokio::test]
#[serial]
async fn test_get_members_event_not_found() {
    let (_container, host) = get_shared_db().await;
    let db = Database::new(host).await;

    let view = GetEventMemberQueryView::new(99999);
    let members = db.fetch_all::<Member, _>(&view).await.unwrap();

    assert!(members.is_empty());
}
