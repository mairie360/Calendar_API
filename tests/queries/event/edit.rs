use calendar_api::database::event::edit::view::EditEventQueryView;
use chrono::{Duration, Utc};
use mairie360_api_lib::database::db_interface::ApiRequestDto;

#[test]
fn test_edit_view_getters_with_optional_fields() {
    let start = Utc::now();
    let end = start + Duration::hours(1);
    let view = EditEventQueryView::new(
        12,
        "New title".to_string(),
        Some("New description".to_string()),
        start,
        end,
        Some("Room 4".to_string()),
    );

    assert_eq!(view.id(), 12);
    assert_eq!(view.title(), "New title");
    assert_eq!(view.description(), "New description");
    assert_eq!(view.location(), "Room 4");
    assert_eq!(
        view.start_date().timestamp_millis(),
        start.timestamp_millis()
    );
    assert_eq!(view.end_date().timestamp_millis(), end.timestamp_millis());
}

#[test]
fn test_edit_view_defaults_missing_optionals_to_empty() {
    let start = Utc::now();
    let end = start + Duration::hours(1);
    let view = EditEventQueryView::new(1, "t".to_string(), None, start, end, None);

    assert_eq!(view.description(), "");
    assert_eq!(view.location(), "");
}

#[test]
fn test_edit_view_sql_and_params() {
    let start = Utc::now();
    let view = EditEventQueryView::new(
        1,
        "t".to_string(),
        None,
        start,
        start + Duration::hours(1),
        None,
    );

    let sql = view.query_sql();
    assert!(sql.contains("UPDATE events"));
    assert!(sql.contains("NULLIF($2, '')"));
    assert!(sql.contains("NULLIF($5, '')"));
    assert!(sql.contains("WHERE id = $6"));
    assert_eq!(view.query_params().len(), 6);
}
