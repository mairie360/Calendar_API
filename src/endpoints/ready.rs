use actix_web::{get, web, HttpResponse, Responder};
use mairie360_api_lib::state::AppState;
use utoipa::OpenApi;

use crate::database::service::readiness::check_dependencies;

#[utoipa::path(
    get,
    path = "ready",
    summary = "Readiness probe",
    description = "Answers `READY` when the service can serve requests: it runs `SELECT 1` on \
                   PostgreSQL and reads a key from Redis (the revocation list of session tokens). \
                   Unauthenticated route, meant for the Kubernetes readiness probe and the \
                   readiness sidecars of the Docker stacks: a `503` takes the instance out of the \
                   load balancing without restarting it. `GET /health` stays the liveness probe \
                   (process up, dependencies not checked).",
    responses(
        (
            status = 200,
            description = "PostgreSQL and Redis both answer.",
            body = String,
            content_type = "text/plain",
            example = json!("READY")
        ),
        (
            status = 503,
            description = "PostgreSQL or Redis does not answer; the body names the first \
                           unreachable dependency.",
            body = String,
            content_type = "text/plain",
            example = json!("PostgreSQL unavailable")
        )
    ),
    tag = "Service"
)]
#[get("/ready")]
pub async fn ready(state: web::Data<AppState>) -> impl Responder {
    match check_dependencies(&state).await {
        Ok(()) => HttpResponse::Ok().body("READY"),
        Err(dependency) => {
            HttpResponse::ServiceUnavailable().body(format!("{dependency} unavailable"))
        }
    }
}

#[derive(OpenApi)]
#[openapi(paths(ready,))]
pub struct ReadyDoc;
