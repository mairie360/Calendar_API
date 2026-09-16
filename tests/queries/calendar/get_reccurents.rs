use crate::common::{create_event, event_input};
use calendar_api::database::calendar::get::view::{Event, GetCalendarQueryView};
use calendar_api::database::event::add_member::view::AddUserToEventQueryView;
use calendar_api::database::event::model::{EventRecurrence, RecurrenceFrequency};
use chrono::{Duration, NaiveDate, TimeZone, Utc};
use mairie360_api_lib::database::db_interface::Database;
use mairie360_api_lib::test_setup::queries_setup::get_shared_db;
use serial_test::serial;

#[tokio::test]
#[serial]
async fn test_member_sees_event_without_owning_it() {
    let (_container, host) = get_shared_db().await;
    let db = Database::new(host).await;
    let start = Utc::now();
    let end = start + Duration::hours(1);

    // Propriété d'Alice (1), Bob (2) est ajouté comme membre.
    let id = create_event(&db, 1, &event_input("Shared Meeting", None, start, end)).await;
    db.execute(&AddUserToEventQueryView::new(2, id))
        .await
        .unwrap();

    let events: Vec<Event> = db
        .fetch_all(&GetCalendarQueryView::new(
            start - Duration::days(1),
            end + Duration::days(1),
            2,
        ))
        .await
        .unwrap();
    let owner_events: Vec<Event> = db
        .fetch_all(&GetCalendarQueryView::new(
            start - Duration::days(1),
            end + Duration::days(1),
            1,
        ))
        .await
        .unwrap();

    assert!(events.iter().any(|e| e.id() as u64 == id && e.is_member));
    assert!(owner_events
        .iter()
        .any(|e| e.id() as u64 == id && !e.is_member));
}

#[tokio::test]
#[serial]
async fn test_recurring_event_is_listed_while_its_rule_overlaps_the_window() {
    let (_container, host) = get_shared_db().await;
    let db = Database::new(host).await;
    let start = Utc.with_ymd_and_hms(2025, 1, 6, 9, 0, 0).unwrap();
    let mut input = event_input("Permanence", None, start, start + Duration::hours(2));
    input.recurrence = Some(EventRecurrence {
        frequency: RecurrenceFrequency::Weekly,
        interval: 1,
        days_of_week: Some(vec![1]),
        ends_on: Some(NaiveDate::from_ymd_opt(2025, 3, 31).unwrap()),
    });
    let id = create_event(&db, 1, &input).await;
    let listed = |from: (i32, u32, u32), to: (i32, u32, u32)| {
        GetCalendarQueryView::new(
            Utc.with_ymd_and_hms(from.0, from.1, from.2, 0, 0, 0)
                .unwrap(),
            Utc.with_ymd_and_hms(to.0, to.1, to.2, 23, 59, 59).unwrap(),
            1,
        )
    };

    let during: Vec<Event> = db
        .fetch_all(&listed((2025, 3, 1), (2025, 3, 31)))
        .await
        .unwrap();
    let last_day: Vec<Event> = db
        .fetch_all(&listed((2025, 3, 31), (2025, 3, 31)))
        .await
        .unwrap();
    let after: Vec<Event> = db
        .fetch_all(&listed((2025, 4, 1), (2025, 4, 30)))
        .await
        .unwrap();

    let found = during
        .iter()
        .find(|e| e.id() as u64 == id)
        .expect("listé pendant la règle");
    assert_eq!(found.recurrence, input.recurrence);
    assert!(last_day.iter().any(|e| e.id() as u64 == id));
    assert!(!after.iter().any(|e| e.id() as u64 == id));
}
