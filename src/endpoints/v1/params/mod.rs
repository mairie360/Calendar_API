use actix_web::{web, HttpResponse, Responder};

pub mod doc;
// pub mod get;
// pub mod patch;

// Le handler qui génère l'erreur 501
async fn not_implemented_handler() -> impl Responder {
    HttpResponse::NotImplemented().body("This feature is not yet implemented.")
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/params")
            // Redirige toutes les requêtes (GET, POST, etc.) vers le handler 501
            .default_service(web::to(not_implemented_handler)),
    );
}
