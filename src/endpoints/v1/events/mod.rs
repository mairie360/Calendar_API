use actix_web::web;

pub mod doc;
pub mod id;
pub mod post;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/events")
            .configure(id::config)
            .service(post::endpoint::create_event),
    );
}
