pub mod error;
pub mod health;
pub mod rate_limit;
pub mod ready;
pub mod swagger;
pub mod v1;

use actix_web::web;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.configure(v1::config);
}
