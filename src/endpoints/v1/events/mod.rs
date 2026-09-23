use actix_web::web;

use crate::database::event::model::EventInput;
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

/// A label displayed as-is by the fronts: at most `max` characters, no control character (Postgres
/// rejects NUL bytes) and no `<` / `>` (echoed back unescaped in the JSON responses).
fn is_valid_label(value: &str, max: usize) -> bool {
    value.chars().count() <= max
        && !value.chars().any(char::is_control)
        && !value.contains(['<', '>'])
}

/// A free-text description: like a label, but line breaks and tabs are allowed.
fn is_valid_description(value: &str) -> bool {
    value.chars().count() <= MAX_DESCRIPTION_LENGTH
        && !value
            .chars()
            .any(|c| c.is_control() && !matches!(c, '\n' | '\r' | '\t'))
        && !value.contains(['<', '>'])
}

/// Rules shared by event creation and update.
pub fn validate_event_input(input: &EventInput) -> Result<(), ApiError> {
    let valid = !input.name.trim().is_empty()
        && is_valid_label(&input.name, MAX_EVENT_NAME_LENGTH)
        && input.end > input.start
        && input
            .service
            .as_deref()
            .is_none_or(|service| is_valid_label(service, MAX_SERVICE_LENGTH))
        && input
            .location
            .as_deref()
            .is_none_or(|location| is_valid_label(location, MAX_LOCATION_LENGTH))
        && input
            .description
            .as_deref()
            .is_none_or(is_valid_description)
        && input
            .recurrence
            .as_ref()
            .is_none_or(|recurrence| recurrence.is_valid_for(input.start));
    if valid {
        Ok(())
    } else {
        Err(ApiError::BadRequest)
    }
}

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
    fn rejects_blank_long_nul_and_markup_names() {
        assert!(validate_event_input(&event("  ")).is_err());
        assert!(validate_event_input(&event(&"a".repeat(151))).is_err());
        assert!(validate_event_input(&event("Conseil\0")).is_err());
        assert!(validate_event_input(&event("<script>alert(1);</script>")).is_err());
    }

    #[test]
    fn rejects_markup_and_nul_in_optional_fields() {
        let mut input = event("Conseil municipal");
        input.location = Some("<script>alert(1);</script>".to_string());
        assert!(validate_event_input(&input).is_err());
        let mut input = event("Conseil municipal");
        input.service = Some("Secr\0".to_string());
        assert!(validate_event_input(&input).is_err());
        let mut input = event("Conseil municipal");
        input.description = Some("a\u{0}b".to_string());
        assert!(validate_event_input(&input).is_err());
    }
}
