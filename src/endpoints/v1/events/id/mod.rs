pub mod delete;
pub mod get;
pub mod members;
pub mod patch;
pub mod validation;

use actix_web::web;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/{event_id}")
            .service(get::endpoint::get_event)
            .service(patch::endpoint::patch_event)
            .service(delete::endpoint::delete_event)
            .service(validation::endpoint::update_event_validation)
            .configure(members::config),
    );
}
