//! Règles de validation et modification partielle des événements (sans base de données).

use calendar_api::database::event::model::{
    EventCategory, EventInput, EventRecurrence, EventVisibility, RecurrenceFrequency,
};
use calendar_api::endpoints::v1::events::id::patch::view::PatchEventView;
use calendar_api::endpoints::v1::events::validate_event_input;
use chrono::{Duration, NaiveDate, TimeZone, Utc};

fn input() -> EventInput {
    let start = Utc.with_ymd_and_hms(2026, 9, 14, 10, 0, 0).unwrap();
    EventInput {
        name: "Conseil".to_string(),
        description: Some("Ordre du jour".to_string()),
        start,
        end: start + Duration::hours(1),
        visibility: EventVisibility::Public,
        category: EventCategory::Meeting,
        service: Some("Urbanisme".to_string()),
        location: Some("Salle A".to_string()),
        recurrence: None,
    }
}

fn weekly(
    days: Option<Vec<u8>>,
    ends_on: Option<NaiveDate>,
    interval: u32,
) -> Option<EventRecurrence> {
    Some(EventRecurrence {
        frequency: RecurrenceFrequency::Weekly,
        interval,
        days_of_week: days,
        ends_on,
    })
}

#[test]
fn event_input_validation() {
    assert!(validate_event_input(&input()).is_ok());

    let mut blank = input();
    blank.name = "  ".to_string();
    let mut reversed = input();
    reversed.end = reversed.start;
    let mut long_service = input();
    long_service.service = Some("x".repeat(129));
    assert!(validate_event_input(&blank).is_err());
    assert!(validate_event_input(&reversed).is_err());
    assert!(validate_event_input(&long_service).is_err());

    for (recurrence, valid) in [
        (
            weekly(Some(vec![1, 3]), NaiveDate::from_ymd_opt(2026, 9, 14), 1),
            true,
        ),
        (weekly(None, None, 365), true),
        (weekly(Some(vec![]), None, 1), false),
        (weekly(Some(vec![7]), None, 1), false),
        (weekly(Some(vec![2, 2]), None, 1), false),
        (weekly(None, None, 0), false),
        (weekly(None, NaiveDate::from_ymd_opt(2026, 9, 13), 1), false),
    ] {
        let mut event = input();
        event.recurrence = recurrence.clone();
        assert_eq!(
            validate_event_input(&event).is_ok(),
            valid,
            "{recurrence:?}"
        );
    }
}

#[test]
fn patch_keeps_missing_fields_and_clears_explicit_nulls() {
    let recurring = PatchEventView {
        recurrence: Some(weekly(Some(vec![1]), None, 2)),
        ..PatchEventView::default()
    }
    .apply_to(input());
    assert_eq!(recurring.recurrence, weekly(Some(vec![1]), None, 2));
    assert_eq!(recurring.location.as_deref(), Some("Salle A"));

    let patch: PatchEventView = serde_json::from_str(
        r#"{"name":" Conseil municipal ","location":null,"recurrence":null,"category":"ceremony"}"#,
    )
    .unwrap();
    let patched = patch.apply_to(recurring);

    assert_eq!(patched.name, "Conseil municipal");
    assert_eq!(patched.location, None);
    assert_eq!(patched.recurrence, None);
    assert_eq!(patched.category, EventCategory::Ceremony);
    assert_eq!(patched.description.as_deref(), Some("Ordre du jour"));
    assert_eq!(patched.service.as_deref(), Some("Urbanisme"));
}

#[test]
fn recurrence_json_contract() {
    let json = serde_json::to_value(weekly(
        Some(vec![1, 5]),
        NaiveDate::from_ymd_opt(2026, 12, 18),
        2,
    ))
    .unwrap();

    assert_eq!(
        json,
        serde_json::json!({ "frequency": "weekly", "interval": 2, "days_of_week": [1, 5], "ends_on": "2026-12-18" })
    );
}
