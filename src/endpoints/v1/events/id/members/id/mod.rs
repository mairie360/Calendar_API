pub mod delete;
pub mod doc;

use actix_web::web;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/{member_id}").service(delete::endpoint::remove_event_member));
}
