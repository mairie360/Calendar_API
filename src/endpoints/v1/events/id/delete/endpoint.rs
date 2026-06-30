use actix_web::http::StatusCode;
use actix_web::{delete, web, HttpResponse, Responder, ResponseError};
use mairie360_api_lib::pool::AppState;
use mairie360_api_lib::security::AuthenticatedUser;

#[derive(Debug, Clone, PartialEq)]
pub enum DeleteEventError {
    DatabaseError,
    UnknownEvent,
}

impl std::fmt::Display for DeleteEventError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DeleteEventError::DatabaseError => {
                write!(f, "An error occurred while accessing the database.")
            }
            DeleteEventError::UnknownEvent => {
                write!(f, "Unknown event.")
            }
        }
    }
}

impl ResponseError for DeleteEventError {
    fn status_code(&self) -> StatusCode {
        match self {
            DeleteEventError::DatabaseError => StatusCode::INTERNAL_SERVER_ERROR,
            DeleteEventError::UnknownEvent => StatusCode::BAD_REQUEST,
        }
    }

    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code()).body(self.to_string())
    }
}

async fn trigger_delete_event(
    state: web::Data<AppState>,
    event_id: u64,
) -> Result<(), DeleteEventError> {
    let pool = match state.db_pool.clone() {
        Some(pool) => pool,
        None => return Err(DeleteEventError::DatabaseError),
    };

    //query

    // update cache

    Ok(())
}

#[utoipa::path(
    delete,
    path = "",
    responses(
        (status = 204, description = "Event deleted successfully"),
        (status = 400, description = "Bad request"),
        (status = 500, description = "Internal server error")
    ),
    tag = "Events",
    params(
        ("event_id" = u64, Path, description = "Event ID")
    ),
    security(
        ("jwt" = [])
    )
)]
#[delete("/")]
pub async fn delete_event(
    state: web::Data<AppState>,
    _: AuthenticatedUser,
    id: web::Path<u64>,
) -> Result<impl Responder, DeleteEventError> {
    let event_id = id.into_inner();
    trigger_delete_event(state, event_id).await?;
    Ok(HttpResponse::NoContent().finish())
}
