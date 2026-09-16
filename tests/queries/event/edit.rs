use crate::common::{create_event, event_input, recurrence_rule_count};
use calendar_api::database::event::edit::view::{
    DeleteOrphanRecurrenceQueryView, EditEventQueryView,
};
use calendar_api::database::event::get::view::{GetEventQueryResultView, GetEventQueryView};
use calendar_api::database::event::model::{EventCategory, EventRecurrence, RecurrenceFrequency};
use chrono::{Duration, TimeZone, Utc};
use mairie360_api_lib::database::db_interface::{ApiRequestDto, Database};
use mairie360_api_lib::test_setup::queries_setup::get_shared_db;
use serial_test::serial;

async fn read(db: &Database, id: u64) -> GetEventQueryResultView {
    db.fetch_one(&GetEventQueryView::new(id)).await.unwrap()
}

#[test]
fn test_edit_view_params() {
    let start = Utc::now();
    let view = EditEventQueryView::new(
        12,
        &event_input("Titre", None, start, start + Duration::hours(1)),
    );

    assert_eq!(view.id(), 12);
    assert_eq!(view.name(), "Titre");
    assert_eq!(view.query_params().len(), 14);
    assert!(view.query_sql().contains("UPDATE events"));
}

#[tokio::test]
#[serial]
async fn test_edit_event_updates_fields_and_recurrence_lifecycle() {
    let (_container, host) = get_shared_db().await;
    let db = Database::new(host).await;
    let start = Utc.with_ymd_and_hms(2026, 11, 2, 14, 0, 0).unwrap();
    let id = create_event(
        &db,
        1,
        &event_input("Réunion", Some("v1"), start, start + Duration::hours(1)),
    )
    .await;

    // Ajout d'une répétition et de métadonnées.
    let mut input = event_input("Réunion d'équipe", None, start, start + Duration::hours(2));
    input.category = EventCategory::Meeting;
    input.location = Some("Mairie annexe".to_string());
    input.recurrence = Some(EventRecurrence {
        frequency: RecurrenceFrequency::Daily,
        interval: 1,
        days_of_week: None,
        ends_on: None,
    });
    let updated: bool = db
        .fetch_scalar(&EditEventQueryView::new(id, &input))
        .await
        .unwrap();
    assert!(updated);
    let event = read(&db, id).await;
    let rule_id = event.recurrence_id().expect("règle créée");
    assert_eq!(event.to_input(), input);

    // Modification de la répétition : la même règle est mise à jour.
    input.recurrence = Some(EventRecurrence {
        frequency: RecurrenceFrequency::Monthly,
        interval: 3,
        days_of_week: None,
        ends_on: chrono::NaiveDate::from_ymd_opt(2027, 6, 30),
    });
    assert!(db
        .fetch_scalar::<bool, _>(&EditEventQueryView::new(id, &input))
        .await
        .unwrap());
    let event = read(&db, id).await;
    assert_eq!(event.recurrence_id(), Some(rule_id));
    assert_eq!(event.to_input().recurrence, input.recurrence);

    // Retrait : l'événement est détaché puis la règle orpheline supprimée.
    input.recurrence = None;
    assert!(db
        .fetch_scalar::<bool, _>(&EditEventQueryView::new(id, &input))
        .await
        .unwrap());
    assert_eq!(read(&db, id).await.recurrence_id(), None);
    assert_eq!(recurrence_rule_count(&db, rule_id).await, 1);
    db.execute(&DeleteOrphanRecurrenceQueryView::new(rule_id as u64))
        .await
        .unwrap();
    assert_eq!(recurrence_rule_count(&db, rule_id).await, 0);
}

#[tokio::test]
#[serial]
async fn test_orphan_cleanup_keeps_attached_rules() {
    let (_container, host) = get_shared_db().await;
    let db = Database::new(host).await;
    let start = Utc::now();
    let mut input = event_input("Récurrent", None, start, start + Duration::hours(1));
    input.recurrence = Some(EventRecurrence {
        frequency: RecurrenceFrequency::Weekly,
        interval: 1,
        days_of_week: None,
        ends_on: None,
    });
    let id = create_event(&db, 1, &input).await;
    let rule_id = read(&db, id).await.recurrence_id().unwrap();

    db.execute(&DeleteOrphanRecurrenceQueryView::new(rule_id as u64))
        .await
        .unwrap();

    assert_eq!(recurrence_rule_count(&db, rule_id).await, 1);
}

#[tokio::test]
#[serial]
async fn test_edit_unknown_event() {
    let (_container, host) = get_shared_db().await;
    let db = Database::new(host).await;
    let start = Utc::now();

    let updated: bool = db
        .fetch_scalar(&EditEventQueryView::new(
            999_999,
            &event_input("Inconnu", None, start, start + Duration::hours(1)),
        ))
        .await
        .unwrap();

    assert!(!updated);
}
