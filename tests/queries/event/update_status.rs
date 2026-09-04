use calendar_api::database::event::{
    add_member::view::AddUserToEventQueryView,
    create::view::CreateEventByUserQueryView,
    get_event_members::view::{
        EventValidationStatus as MemberValidationStatus, GetEventMemberQueryView, Member,
    },
    update_user_status::view::{EventStatusUpdateQueryView, EventValidationStatus},
};
use chrono::Utc;
use mairie360_api_lib::database::db_interface::Database;
use mairie360_api_lib::test_setup::queries_setup::get_shared_db;
use serial_test::serial;

#[tokio::test]
#[serial]
async fn test_update_user_status_success() {
    let (_container, host) = get_shared_db().await;
    let db = Database::new(host).await;

    let start = Utc::now();
    let end = start + chrono::Duration::hours(1);
    let view =
        CreateEventByUserQueryView::new("Test Event", Some("Description"), start, end, 1, None, 1);
    let id = db.fetch_scalar::<i32, _>(&view).await.unwrap() as u64;

    let view = AddUserToEventQueryView::new(2, id);
    assert!(db.execute(&view).await.is_ok());

    let view = EventStatusUpdateQueryView::new(2, id, EventValidationStatus::Validated);
    assert!(db.execute(&view).await.is_ok());

    let view = GetEventMemberQueryView::new(id);
    let members = db.fetch_all::<Member, _>(&view).await.unwrap();
    let member = members
        .iter()
        .find(|m| m.user_id() == 2)
        .expect("member should exist");
    assert_eq!(
        member.validation_status(),
        MemberValidationStatus::Validated
    );
}

#[tokio::test]
#[serial]
async fn test_update_user_status_no_match_is_ok() {
    let (_container, host) = get_shared_db().await;
    let db = Database::new(host).await;

    // Aucune ligne ne correspond : l'UPDATE est un no-op sans erreur.
    let view = EventStatusUpdateQueryView::new(999, 99999, EventValidationStatus::Refused);
    assert!(db.execute(&view).await.is_ok());
}
