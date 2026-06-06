pub mod doc;
pub mod events;
pub mod get;
pub mod params;

use actix_web::web;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/v1")
            .configure(events::config)
            .service(get::endpoint::get)
            .configure(params::config),
    );
}
