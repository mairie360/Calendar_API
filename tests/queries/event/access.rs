use crate::common::{create_event, create_group, create_user, event_input};
use calendar_api::database::event::access::view::{
    ApprovalStatus, EventAccess, EventAccessQueryView,
};
use calendar_api::database::event::add_member::view::AddUserToEventQueryView;
use calendar_api::database::event::get_event_members::view::{
    EventValidationStatus, GetEventMemberQueryView, Member,
};
use calendar_api::database::event::validation::view::{
    CanAssignUserQueryView, RefreshEventValidationQueryView, SetEventValidationQueryView,
};
use chrono::{Duration, Utc};
use mairie360_api_lib::database::db_interface::Database;
use mairie360_api_lib::test_setup::queries_setup::get_shared_db;
use serial_test::serial;

async fn access(db: &Database, event_id: u64, user_id: u64) -> EventAccess {
    let mut access: EventAccess = db
        .fetch_one(&EventAccessQueryView::new(event_id, user_id))
        .await
        .unwrap();
    access.caller_id = user_id as i32;
    access
}

async fn add_member(db: &Database, event_id: u64, user_id: u64) {
    db.execute(&AddUserToEventQueryView::new(user_id, event_id))
        .await
        .unwrap();
    db.execute(&RefreshEventValidationQueryView::new(event_id))
        .await
        .unwrap();
}

#[tokio::test]
#[serial]
async fn test_event_requiring_approval_is_validated_by_a_responsable_of_the_creator_group() {
    let (_container, host) = get_shared_db().await;
    let db = Database::new(host).await;
    let creator = create_user(&db, "Agent", Some("User")).await;
    let responsable = create_user(&db, "Chef", Some("Responsable")).await;
    let foreign_responsable = create_user(&db, "Autre", Some("Responsable")).await;
    let outsider = create_user(&db, "Externe", Some("User")).await;
    create_group(&db, creator, &[creator, responsable]).await;
    let start = Utc::now();
    let event_id = create_event(
        &db,
        creator,
        &event_input("Congés", None, start, start + Duration::hours(8)),
    )
    .await;

    add_member(&db, event_id, creator).await;
    assert_eq!(
        access(&db, event_id, creator).await.approval_status,
        ApprovalStatus::Approved
    );

    add_member(&db, event_id, responsable).await;
    add_member(&db, event_id, foreign_responsable).await;

    let creator_access = access(&db, event_id, creator).await;
    let responsable_access = access(&db, event_id, responsable).await;
    let foreign_access = access(&db, event_id, foreign_responsable).await;
    let outsider_access = access(&db, event_id, outsider).await;

    assert!(creator_access.exists && creator_access.requires_approval);
    assert_eq!(creator_access.approval_status, ApprovalStatus::Pending);
    assert!(
        creator_access.can_edit() && creator_access.can_delete() && !creator_access.can_validate()
    );
    assert!(
        responsable_access.can_validate()
            && responsable_access.can_edit()
            && !responsable_access.can_delete()
    );
    assert!(foreign_access.can_view() && !foreign_access.can_validate());
    assert!(
        !outsider_access.can_view()
            && !outsider_access.can_edit()
            && !outsider_access.can_manage_members()
    );

    db.execute(&SetEventValidationQueryView::new(
        event_id,
        EventValidationStatus::Validated,
    ))
    .await
    .unwrap();
    let members: Vec<Member> = db
        .fetch_all(&GetEventMemberQueryView::new(event_id))
        .await
        .unwrap();
    assert!(members
        .iter()
        .all(|member| member.validation_status() == EventValidationStatus::Validated));
    assert_eq!(
        access(&db, event_id, responsable).await.approval_status,
        ApprovalStatus::Approved
    );
    assert!(!access(&db, event_id, responsable).await.can_validate());
}

#[tokio::test]
#[serial]
async fn test_event_created_by_a_responsable_never_requires_approval() {
    let (_container, host) = get_shared_db().await;
    let db = Database::new(host).await;
    let creator = create_user(&db, "Chef", Some("Responsable")).await;
    let colleague = create_user(&db, "Collègue", Some("Responsable")).await;
    create_group(&db, creator, &[creator, colleague]).await;
    let start = Utc::now();
    let event_id = create_event(
        &db,
        creator,
        &event_input("Réunion", None, start, start + Duration::hours(1)),
    )
    .await;

    add_member(&db, event_id, creator).await;
    add_member(&db, event_id, colleague).await;

    let colleague_access = access(&db, event_id, colleague).await;
    assert!(!colleague_access.requires_approval);
    assert_eq!(colleague_access.approval_status, ApprovalStatus::Approved);
    assert!(colleague_access.can_edit() && !colleague_access.can_delete());
}

#[tokio::test]
#[serial]
async fn test_unknown_event_access() {
    let (_container, host) = get_shared_db().await;
    let db = Database::new(host).await;

    let unknown = access(&db, 999_999, 1).await;

    assert!(!unknown.exists && !unknown.can_view());
}

#[tokio::test]
#[serial]
async fn test_assignment_scope_follows_roles_and_groups() {
    let (_container, host) = get_shared_db().await;
    let db = Database::new(host).await;
    let agent = create_user(&db, "Agent", Some("User")).await;
    let teammate = create_user(&db, "Equipier", Some("User")).await;
    let stranger = create_user(&db, "Inconnu", Some("User")).await;
    let maire = create_user(&db, "Maire", Some("Maire")).await;
    create_group(&db, agent, &[agent, teammate]).await;

    for (caller, user, expected) in [
        (agent, agent, true),
        (agent, teammate, true),
        (agent, stranger, false),
        (maire, stranger, true),
        (agent, 999_999, false),
    ] {
        let assignable: bool = db
            .fetch_scalar(&CanAssignUserQueryView::new(caller, user))
            .await
            .unwrap();
        assert_eq!(assignable, expected, "{caller} assigne {user}");
    }
}
