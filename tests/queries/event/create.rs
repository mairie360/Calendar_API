use calendar_api::database::event::create::view::{
    CreateEventByGroupQueryView, CreateEventByUserQueryView, ReccurenceType, RecurrenceRule,
};
use chrono::{Duration, Utc};
use mairie360_api_lib::database::db_interface::{ApiRequestDto, Database};
use mairie360_api_lib::test_setup::queries_setup::get_shared_db;
use serial_test::serial;

#[test]
fn test_recurrence_type_roundtrip_and_display() {
    for (variant, s) in [
        (ReccurenceType::Daily, "Daily"),
        (ReccurenceType::Weekly, "Weekly"),
        (ReccurenceType::Monthly, "Monthly"),
        (ReccurenceType::Error, "Error"),
    ] {
        assert_eq!(variant.as_str(), s);
        assert_eq!(format!("{variant}"), s);
        assert_eq!(ReccurenceType::from(s.to_string()), variant);
    }
    assert_eq!(
        ReccurenceType::from("nonsense".to_string()),
        ReccurenceType::Error
    );
}

#[test]
fn test_recurrence_rule_getters_and_display() {
    let end = Utc::now() + Duration::days(5);
    let rule = RecurrenceRule::new(ReccurenceType::Weekly, Some(2), Some(end));

    assert_eq!(*rule.type_recurrence(), ReccurenceType::Weekly);
    assert_eq!(rule.intervalle(), Some(2));
    assert_eq!(rule.date_fin(), Some(end));
    assert!(format!("{rule}").contains("Weekly"));

    let bare = RecurrenceRule::new(ReccurenceType::Daily, None, None);
    assert_eq!(bare.intervalle(), None);
    assert_eq!(bare.date_fin(), None);
}

#[test]
fn test_create_by_user_view_getters_and_display() {
    let start = Utc::now();
    let end = start + Duration::days(1);
    let view = CreateEventByUserQueryView::new("Party", Some("fun"), start, end, 3, None, 9);

    assert_eq!(view.name(), "Party");
    assert_eq!(view.description(), "fun");
    assert_eq!(
        view.start_date().timestamp_millis(),
        start.timestamp_millis()
    );
    assert_eq!(view.end_date().timestamp_millis(), end.timestamp_millis());
    assert_eq!(view.created_by(), 3);
    assert_eq!(view.owner_id(), 9);
    assert!(view.query_sql().contains("INSERT INTO events"));
    assert_eq!(view.query_params().len(), 6);
    assert!(format!("{view}").contains("Party"));
}

#[test]
fn test_create_by_group_view_getters_and_display() {
    let start = Utc::now();
    let end = start + Duration::days(1);
    let view =
        CreateEventByGroupQueryView::new("Team Event".to_string(), None, start, end, 4, None, 7);

    assert_eq!(view.name(), "Team Event");
    assert_eq!(view.description(), "");
    assert_eq!(
        view.start_date().timestamp_millis(),
        start.timestamp_millis()
    );
    assert_eq!(view.end_date().timestamp_millis(), end.timestamp_millis());
    assert_eq!(view.created_by(), 4);
    assert_eq!(view.owner_id(), 7);
    assert!(view.query_sql().contains("INSERT INTO events"));
    assert_eq!(view.query_params().len(), 6);
    assert!(format!("{view}").contains("Team Event"));
}

#[tokio::test]
#[serial]
async fn test_create_event_by_user_success() {
    let (_container, host) = get_shared_db().await;
    let db = Database::new(host).await;

    let start_date = Utc::now();
    let end_date = start_date + chrono::Duration::days(10);
    let view = CreateEventByUserQueryView::new(
        "Test Event",
        Some("Description"),
        start_date,
        end_date,
        1, // ID utilisateur admin (assure-toi qu'il existe en DB)
        None,
        1,
    );

    let result = db.fetch_scalar::<i32, _>(&view).await;

    assert!(result.is_ok());
    assert!(result.unwrap() > 0);
}

#[tokio::test]
#[serial]
async fn test_create_recurrent_event_by_user_success() {
    let (_container, host) = get_shared_db().await;
    let db = Database::new(host).await;

    let start_date = Utc::now();
    let end_date = start_date + chrono::Duration::days(10);
    let view = CreateEventByUserQueryView::new(
        "Test Event",
        Some("Description"),
        start_date,
        end_date,
        1,
        Some(RecurrenceRule::new(ReccurenceType::Daily, Some(1), None)),
        1,
    );

    let result = db.fetch_scalar::<i32, _>(&view).await;

    assert!(result.is_ok());
    assert!(result.unwrap() > 0);
}

#[tokio::test]
#[serial]
async fn test_create_event_by_group_success() {
    let (_container, host) = get_shared_db().await;
    let db = Database::new(host).await;

    let start_date = Utc::now();
    let end_date = start_date + chrono::Duration::days(10);
    let view = CreateEventByGroupQueryView::new(
        "Test Event".to_string(),
        Some("Description".to_string()),
        start_date,
        end_date,
        1,
        None,
        1,
    );

    let result = db.fetch_scalar::<i32, _>(&view).await;

    assert!(result.is_ok());
    assert!(result.unwrap() > 0);
}

#[tokio::test]
#[serial]
async fn test_create_recurrent_event_by_group() {
    let (_container, host) = get_shared_db().await;
    let db = Database::new(host).await;

    let start_date = Utc::now();
    let end_date = start_date + chrono::Duration::days(10);
    let view = CreateEventByGroupQueryView::new(
        "Test Event".to_string(),
        Some("Description".to_string()),
        start_date,
        end_date,
        1,
        Some(RecurrenceRule::new(ReccurenceType::Daily, Some(1), None)),
        1,
    );

    let result = db.fetch_scalar::<i32, _>(&view).await;

    assert!(result.is_ok());
    assert!(result.unwrap() > 0);
}
