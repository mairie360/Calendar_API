use actix_web::web;

use crate::database::event::model::EventInput;
use crate::endpoints::error::ApiError;

pub mod doc;
pub mod id;
pub mod post;

pub const MAX_EVENT_NAME_LENGTH: usize = 150;
pub const MAX_SERVICE_LENGTH: usize = 128;

/// Règles communes à la création et à la modification d'un événement.
pub fn validate_event_input(input: &EventInput) -> Result<(), ApiError> {
    let name_length = input.name.trim().chars().count();
    let valid = name_length > 0
        && name_length <= MAX_EVENT_NAME_LENGTH
        && input.end > input.start
        && input
            .service
            .as_ref()
            .is_none_or(|service| service.chars().count() <= MAX_SERVICE_LENGTH)
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
