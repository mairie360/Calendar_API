pub mod doc;
pub mod events;
pub mod get;
pub mod search;

use actix_web::web;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/v1")
            .configure(events::config)
            .service(web::scope("/search").configure(search::config))
            .service(get::endpoint::get_calendar),
    );
}
