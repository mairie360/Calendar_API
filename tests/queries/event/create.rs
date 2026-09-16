use crate::common::{create_event, event_input};
use calendar_api::database::event::create::view::CreateEventQueryView;
use calendar_api::database::event::get::view::{GetEventQueryResultView, GetEventQueryView};
use calendar_api::database::event::model::{
    EventCategory, EventRecurrence, EventVisibility, RecurrenceFrequency,
};
use chrono::{Duration, NaiveDate, TimeZone, Utc};
use mairie360_api_lib::database::db_interface::{ApiRequestDto, Database};
use mairie360_api_lib::test_setup::queries_setup::get_shared_db;
use serial_test::serial;

#[test]
fn test_create_view_params_and_display() {
    let start = Utc::now();
    let view = CreateEventQueryView::new(
        3,
        &event_input("Party", Some("fun"), start, start + Duration::hours(1)),
    );

    assert_eq!(view.creator_id(), 3);
    assert_eq!(view.name(), "Party");
    assert_eq!(view.query_params().len(), 14);
    assert!(view.query_sql().contains("INSERT INTO events"));
    assert!(format!("{view}").contains("Party"));
}

#[tokio::test]
#[serial]
async fn test_create_event_stores_metadata_without_recurrence() {
    let (_container, host) = get_shared_db().await;
    let db = Database::new(host).await;
    let start = Utc.with_ymd_and_hms(2026, 10, 5, 9, 0, 0).unwrap();
    let mut input = event_input(
        "Conseil",
        Some("Ordre du jour"),
        start,
        start + Duration::hours(2),
    );
    input.visibility = EventVisibility::Private;
    input.category = EventCategory::Ceremony;
    input.service = Some("Urbanisme".to_string());
    input.location = Some("Salle du conseil".to_string());

    let id = create_event(&db, 1, &input).await;
    let event: GetEventQueryResultView = db.fetch_one(&GetEventQueryView::new(id)).await.unwrap();

    assert_eq!(event.to_input(), input);
    assert_eq!(event.created_by(), Some(1));
    assert_eq!(event.owner_id(), Some(1));
    assert_eq!(event.recurrence_id(), None);
}

#[tokio::test]
#[serial]
async fn test_create_event_with_weekly_recurrence() {
    let (_container, host) = get_shared_db().await;
    let db = Database::new(host).await;
    let start = Utc.with_ymd_and_hms(2026, 9, 7, 8, 30, 0).unwrap();
    let mut input = event_input("Point hebdo", None, start, start + Duration::minutes(45));
    input.recurrence = Some(EventRecurrence {
        frequency: RecurrenceFrequency::Weekly,
        interval: 2,
        days_of_week: Some(vec![1, 3]),
        ends_on: Some(NaiveDate::from_ymd_opt(2026, 12, 18).unwrap()),
    });

    let id = create_event(&db, 1, &input).await;
    let event: GetEventQueryResultView = db.fetch_one(&GetEventQueryView::new(id)).await.unwrap();

    assert!(event.recurrence_id().is_some());
    assert_eq!(event.to_input().recurrence, input.recurrence);
}

#[tokio::test]
#[serial]
async fn test_create_event_rejects_end_before_start() {
    let (_container, host) = get_shared_db().await;
    let db = Database::new(host).await;
    let start = Utc::now();

    let result = db
        .fetch_scalar::<i32, _>(&CreateEventQueryView::new(
            1,
            &event_input("Inversé", None, start, start - Duration::hours(1)),
        ))
        .await;

    assert!(result.is_err());
}
