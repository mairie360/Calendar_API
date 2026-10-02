use crate::common::{create_event, event_input, recurrence_rule_count};
use calendar_api::database::event::{
    create::view::CreateEventQueryView,
    delete::view::DeleteEventQueryView,
    get::view::{GetEventQueryResultView, GetEventQueryView},
    model::{EventRecurrence, RecurrenceFrequency},
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

    let view = CreateEventQueryView::new(
        1,
        &event_input("Test Event", Some("Description"), start, end),
    );
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

    // Deleting an unknown event is a no-op without error.
    let view = DeleteEventQueryView::new(99999);
    assert!(db.execute(&view).await.is_ok());

    let view = GetEventQueryView::new(99999);
    let result = db.fetch_one::<GetEventQueryResultView, _>(&view).await;
    assert!(matches!(result, Err(DbError::NotFound)));
}

#[tokio::test]
#[serial]
async fn test_delete_event_removes_its_recurrence_rule_in_the_same_statement() {
    let (_container, host) = get_shared_db().await;
    let db = Database::new(host).await;
    let start = Utc::now();
    let mut input = event_input(
        "Permanence",
        None,
        start,
        start + chrono::Duration::hours(1),
    );
    input.recurrence = Some(EventRecurrence {
        frequency: RecurrenceFrequency::Weekly,
        interval: 1,
        days_of_week: Some(vec![1]),
        ends_on: None,
    });
    let id = create_event(&db, 1, &input).await;
    let rule_id = db
        .fetch_one::<GetEventQueryResultView, _>(&GetEventQueryView::new(id))
        .await
        .unwrap()
        .recurrence_id()
        .expect("rule created");

    db.execute(&DeleteEventQueryView::new(id)).await.unwrap();

    assert_eq!(recurrence_rule_count(&db, rule_id).await, 0);
}
