use crate::common::{approval_decision, create_event, create_group, create_user, event_input};
use calendar_api::database::event::access::view::{
    ApprovalStatus, EventAccess, EventAccessQueryView,
};
use calendar_api::database::event::add_member::view::AddUserToEventQueryView;
use calendar_api::database::event::edit::view::EditEventQueryView;
use calendar_api::database::event::get_event_members::view::{
    EventValidationStatus, GetEventMemberQueryView, Member,
};
use calendar_api::database::event::model::EventVisibility;
use calendar_api::database::event::remove_member::view::RemoveUserFromEventQueryView;
use calendar_api::database::event::validation::view::{
    CanAssignUserQueryView, SetEventApprovalQueryView,
};
use chrono::{Duration, Utc};
use mairie360_api_lib::database::db_interface::Database;
use mairie360_api_lib::database::error::DbError;
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
}

async fn remove_member(db: &Database, event_id: u64, user_id: u64) {
    db.fetch_scalar::<i32, _>(&RemoveUserFromEventQueryView::new(user_id, event_id))
        .await
        .unwrap();
}

/// Private event created by `creator`.
async fn private_event(db: &Database, creator: u64, name: &str) -> u64 {
    let start = Utc::now();
    let mut input = event_input(name, None, start, start + Duration::hours(8));
    input.visibility = EventVisibility::Private;
    create_event(db, creator, &input).await
}

/// A User, a Responsable sharing a group with them, a Responsable outside it, and an outsider.
async fn approval_circuit(db: &Database) -> (u64, u64, u64, u64) {
    let creator = create_user(db, "Agent", Some("User")).await;
    let responsable = create_user(db, "Chef", Some("Responsable")).await;
    let foreign_responsable = create_user(db, "Autre", Some("Responsable")).await;
    let outsider = create_user(db, "Externe", Some("User")).await;
    create_group(db, creator, &[creator, responsable]).await;
    (creator, responsable, foreign_responsable, outsider)
}

#[tokio::test]
#[serial]
async fn test_event_requiring_approval_is_validated_by_a_responsable_of_the_creator_group() {
    let (_container, host) = get_shared_db().await;
    let db = Database::new(host).await;
    let (creator, responsable, foreign_responsable, outsider) = approval_circuit(&db).await;
    let event_id = private_event(&db, creator, "Congés").await;

    // Pending from creation, whoever is assigned.
    add_member(&db, event_id, creator).await;
    assert_eq!(
        access(&db, event_id, creator).await.approval_status,
        ApprovalStatus::Pending
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

    db.fetch_scalar::<i32, _>(&SetEventApprovalQueryView::new(
        event_id,
        responsable,
        ApprovalStatus::Approved,
    ))
    .await
    .unwrap();
    assert_eq!(
        approval_decision(&db, event_id).await,
        ("validated".to_string(), Some(responsable as i32), true)
    );
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

    // A second decision finds no pending event.
    let second = db
        .fetch_scalar::<i32, _>(&SetEventApprovalQueryView::new(
            event_id,
            responsable,
            ApprovalStatus::Rejected,
        ))
        .await;
    assert!(matches!(second, Err(DbError::NotFound)));
}

#[tokio::test]
#[serial]
async fn test_member_changes_never_overwrite_the_approval() {
    let (_container, host) = get_shared_db().await;
    let db = Database::new(host).await;
    let (creator, responsable, _, outsider) = approval_circuit(&db).await;

    // Never inviting a Responsable leaves the event pending.
    let never_reviewed = private_event(&db, creator, "Sans responsable").await;
    add_member(&db, never_reviewed, creator).await;
    add_member(&db, never_reviewed, outsider).await;
    assert_eq!(
        access(&db, never_reviewed, creator).await.approval_status,
        ApprovalStatus::Pending
    );

    // Removing the Responsable who rejected the event keeps it rejected.
    let rejected = private_event(&db, creator, "Refusé").await;
    add_member(&db, rejected, creator).await;
    add_member(&db, rejected, responsable).await;
    db.fetch_scalar::<i32, _>(&SetEventApprovalQueryView::new(
        rejected,
        responsable,
        ApprovalStatus::Rejected,
    ))
    .await
    .unwrap();
    remove_member(&db, rejected, responsable).await;
    add_member(&db, rejected, outsider).await;
    assert_eq!(
        access(&db, rejected, creator).await.approval_status,
        ApprovalStatus::Rejected
    );
    assert_eq!(
        approval_decision(&db, rejected).await,
        ("refused".to_string(), Some(responsable as i32), true)
    );
}

#[tokio::test]
#[serial]
async fn test_edit_resets_the_approval_only_when_asked() {
    let (_container, host) = get_shared_db().await;
    let db = Database::new(host).await;
    let (creator, responsable, _, _) = approval_circuit(&db).await;
    let start = Utc::now();
    let input = event_input("Permanence", None, start, start + Duration::hours(2));
    let event_id = create_event(&db, creator, &input).await;
    add_member(&db, event_id, creator).await;
    add_member(&db, event_id, responsable).await;
    db.fetch_scalar::<i32, _>(&SetEventApprovalQueryView::new(
        event_id,
        responsable,
        ApprovalStatus::Approved,
    ))
    .await
    .unwrap();

    let creator_access = access(&db, event_id, creator).await;
    let responsable_access = access(&db, event_id, responsable).await;
    assert!(creator_access.edit_needs_new_approval());
    assert!(!responsable_access.edit_needs_new_approval());

    let mut renamed = input.clone();
    renamed.name = "Permanence du samedi".to_string();
    assert!(!renamed.changes_schedule(&input));
    assert!(db
        .fetch_scalar::<bool, _>(&EditEventQueryView::new(event_id, &renamed, false))
        .await
        .unwrap());
    assert_eq!(approval_decision(&db, event_id).await.0, "validated");

    let mut moved = renamed.clone();
    moved.start += Duration::days(1);
    moved.end += Duration::days(1);
    assert!(moved.changes_schedule(&renamed));
    assert!(db
        .fetch_scalar::<bool, _>(&EditEventQueryView::new(event_id, &moved, true))
        .await
        .unwrap());
    assert_eq!(
        approval_decision(&db, event_id).await,
        ("pending".to_string(), None, false)
    );
    assert!(access(&db, event_id, responsable).await.can_validate());
}

#[tokio::test]
#[serial]
async fn test_event_created_by_a_responsable_never_requires_approval() {
    let (_container, host) = get_shared_db().await;
    let db = Database::new(host).await;
    let creator = create_user(&db, "Chef", Some("Responsable")).await;
    let colleague = create_user(&db, "Collègue", Some("Responsable")).await;
    create_group(&db, creator, &[creator, colleague]).await;
    let event_id = private_event(&db, creator, "Réunion").await;

    add_member(&db, event_id, creator).await;
    add_member(&db, event_id, colleague).await;

    let colleague_access = access(&db, event_id, colleague).await;
    assert!(!colleague_access.requires_approval);
    assert_eq!(colleague_access.approval_status, ApprovalStatus::Approved);
    assert!(colleague_access.can_edit() && !colleague_access.can_delete());
}

#[tokio::test]
#[serial]
async fn test_visibility_and_creator_read_access() {
    let (_container, host) = get_shared_db().await;
    let db = Database::new(host).await;
    let creator = create_user(&db, "Agent", Some("User")).await;
    let outsider = create_user(&db, "Externe", Some("User")).await;
    let start = Utc::now();
    let public_event = create_event(
        &db,
        creator,
        &event_input("Fête de quartier", None, start, start + Duration::hours(3)),
    )
    .await;
    let private = private_event(&db, creator, "Entretien").await;

    // The creator reads their event without being a member.
    let creator_access = access(&db, private, creator).await;
    assert!(!creator_access.is_member && creator_access.can_view());
    assert!(!creator_access.can_edit() && creator_access.can_delete());

    // Public: readable by anyone, but nothing more.
    let outsider_public = access(&db, public_event, outsider).await;
    assert!(outsider_public.is_public && outsider_public.can_view());
    assert!(!outsider_public.can_edit() && !outsider_public.can_manage_members());
    assert!(!access(&db, private, outsider).await.can_view());
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
