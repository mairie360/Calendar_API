use actix_web::{get, HttpResponse, Responder};
use utoipa::OpenApi;

#[utoipa::path(
    get,
    path = "health",
    summary = "Liveness probe",
    description = "Answers `OK` as soon as the process accepts connections. Unauthenticated \
                   route, meant for the Kubernetes liveness probe. It checks neither PostgreSQL \
                   nor Redis, so that an outage of a dependency does not restart every instance: \
                   use `GET /ready` to know whether the service can serve requests.",
    responses(
        (
            status = 200,
            description = "The process accepts connections.",
            body = String,
            content_type = "text/plain",
            example = json!("OK")
        )
    ),
    tag = "Service"
)]
#[get("/health")]
pub async fn health() -> impl Responder {
    HttpResponse::Ok().body("OK")
}

#[derive(OpenApi)]
#[openapi(paths(health,))]
pub struct HealthDoc;
