use actix_web::web;

use crate::database::event::model::{is_event_date, EventInput};
use crate::endpoints::error::ApiError;

pub mod doc;
pub mod id;
pub mod post;

/// `events.name` is `VARCHAR(150)`.
pub const MAX_EVENT_NAME_LENGTH: usize = 150;
/// `events.service_label` is `VARCHAR(128)`.
pub const MAX_SERVICE_LENGTH: usize = 128;
/// `events.location` is `TEXT`, capped to keep the payloads reasonable.
pub const MAX_LOCATION_LENGTH: usize = 255;
/// `events.description` is `TEXT`, capped to keep the payloads reasonable.
pub const MAX_DESCRIPTION_LENGTH: usize = 5000;

/// A single-line label: at most `max` characters and no control character (Postgres rejects NUL
/// bytes). `<` and `>` are ordinary text (« budget > 10 000 € », « -> »): the API serves JSON with
/// `nosniff`, escaping is the job of the fronts that render it (MAIR-426).
fn is_valid_label(value: &str, max: usize) -> bool {
    value.chars().count() <= max && !value.chars().any(char::is_control)
}

/// A free-text description: like a label, but line breaks and tabs are allowed.
fn is_valid_description(value: &str) -> bool {
    value.chars().count() <= MAX_DESCRIPTION_LENGTH
        && !value
            .chars()
            .any(|c| c.is_control() && !matches!(c, '\n' | '\r' | '\t'))
}

/// `400` naming `field` when `valid` is false.
fn require(valid: bool, field: &str, rule: &str) -> Result<(), ApiError> {
    if valid {
        Ok(())
    } else {
        Err(ApiError::invalid_field(field, rule))
    }
}

/// Rules shared by event creation and update. The error names the first faulty field.
pub fn validate_event_input(input: &EventInput) -> Result<(), ApiError> {
    require(
        !input.name.trim().is_empty() && is_valid_label(&input.name, MAX_EVENT_NAME_LENGTH),
        "name",
        "must hold 1 to 150 characters once trimmed, without control characters",
    )?;
    require(is_event_date(input.start), "events_start_time", DATE_RULE)?;
    require(is_event_date(input.end), "events_end_time", DATE_RULE)?;
    require(
        input.end > input.start,
        "events_end_time",
        "must be strictly after `events_start_time`",
    )?;
    require(
        input
            .service
            .as_deref()
            .is_none_or(|service| is_valid_label(service, MAX_SERVICE_LENGTH)),
        "service",
        "must hold at most 128 characters, without control characters",
    )?;
    require(
        input
            .location
            .as_deref()
            .is_none_or(|location| is_valid_label(location, MAX_LOCATION_LENGTH)),
        "location",
        "must hold at most 255 characters, without control characters",
    )?;
    require(
        input
            .description
            .as_deref()
            .is_none_or(is_valid_description),
        "description",
        "must hold at most 5000 characters, without control characters other than line breaks and tabs",
    )?;
    match input
        .recurrence
        .as_ref()
        .and_then(|recurrence| recurrence.invalid_field(input.start))
    {
        Some((field, rule)) => Err(ApiError::invalid_field(field, rule)),
        None => Ok(()),
    }
}

/// Rule of every event date, shared by the error messages.
pub const DATE_RULE: &str = "must be between 1970-01-01T00:00:00Z and 2999-12-31T23:59:59Z";

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/events")
            .configure(id::config)
            .service(post::endpoint::create_event),
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{Duration, Utc};

    fn event(name: &str) -> EventInput {
        let start = Utc::now();
        EventInput {
            name: name.to_string(),
            description: None,
            start,
            end: start + Duration::hours(2),
            visibility: Default::default(),
            category: Default::default(),
            service: None,
            location: None,
            recurrence: None,
        }
    }

    #[test]
    fn accepts_a_plain_event() {
        let mut input = event("Conseil municipal");
        input.description = Some("Ordre du jour :\n- budget".to_string());
        input.location = Some("Salle du conseil".to_string());
        assert!(validate_event_input(&input).is_ok());
    }

    #[test]
    fn rejects_blank_long_and_nul_names() {
        assert!(validate_event_input(&event("  ")).is_err());
        assert!(validate_event_input(&event(&"a".repeat(151))).is_err());
        assert!(validate_event_input(&event("Conseil\0")).is_err());
    }

    #[test]
    fn accepts_angle_brackets_in_every_text_field() {
        let mut input = event("Budget > 10 000 € <3");
        input.service = Some("Urbanisme -> voirie".to_string());
        input.location = Some("Salle <A>".to_string());
        input.description = Some("Si recettes < dépenses :\n-> report".to_string());
        assert!(validate_event_input(&input).is_ok());
    }

    #[test]
    fn errors_name_the_faulty_field() {
        let message = |input: &EventInput| validate_event_input(input).unwrap_err().to_string();
        assert!(message(&event(" ")).starts_with("Invalid `name`"));
        let mut input = event("Conseil municipal");
        input.end = input.start;
        assert!(message(&input).starts_with("Invalid `events_end_time`"));
        let mut input = event("Conseil municipal");
        input.location = Some("a".repeat(256));
        assert!(message(&input).starts_with("Invalid `location`"));
    }

    #[test]
    fn rejects_dates_outside_the_calendar_window() {
        use chrono::TimeZone;
        let mut input = event("Conseil municipal");
        input.start = Utc.with_ymd_and_hms(1969, 12, 31, 23, 59, 59).unwrap();
        assert!(validate_event_input(&input)
            .unwrap_err()
            .to_string()
            .starts_with("Invalid `events_start_time`"));
        let mut input = event("Conseil municipal");
        input.end = Utc.with_ymd_and_hms(3000, 1, 1, 0, 0, 0).unwrap();
        assert!(validate_event_input(&input)
            .unwrap_err()
            .to_string()
            .starts_with("Invalid `events_end_time`"));
        let mut input = event("Conseil municipal");
        input.start = Utc.with_ymd_and_hms(1970, 1, 1, 0, 0, 0).unwrap();
        input.end = Utc.with_ymd_and_hms(2999, 12, 31, 23, 59, 59).unwrap();
        assert!(validate_event_input(&input).is_ok());
    }

    #[test]
    fn rejects_nul_in_optional_fields() {
        let mut input = event("Conseil municipal");
        input.service = Some("Secr\0".to_string());
        assert!(validate_event_input(&input).is_err());
        let mut input = event("Conseil municipal");
        input.description = Some("a\u{0}b".to_string());
        assert!(validate_event_input(&input).is_err());
    }
}
