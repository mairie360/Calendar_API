pub mod doc;
pub mod get;
pub mod id;
pub mod post;

use actix_web::web;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/members")
            .service(get::endpoint::get)
            .service(post::endpoint::post)
            .configure(id::config),
    );
}
