pub mod delete;
pub mod doc;
pub mod get;
pub mod members;
pub mod patch;

use actix_web::web;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/{id}")
            .service(get::endpoint::get)
            .service(patch::endpoint::patch)
            .service(delete::endpoint::delete)
            .configure(members::config),
    );
}
