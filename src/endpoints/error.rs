use actix_web::http::StatusCode;
use actix_web::{HttpResponse, ResponseError};

use crate::database::event::access::view::EventAccess;

/// Erreurs communes des endpoints du calendrier (corps texte, sans détail interne).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ApiError {
    BadRequest,
    Forbidden,
    NotFound,
    Conflict,
    DatabaseError,
}

impl std::fmt::Display for ApiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let message = match self {
            ApiError::BadRequest => "Bad request.",
            ApiError::Forbidden => "Forbidden.",
            ApiError::NotFound => "Unknown event.",
            ApiError::Conflict => "Conflict.",
            ApiError::DatabaseError => "An error occurred while accessing the database.",
        };
        f.write_str(message)
    }
}

impl ResponseError for ApiError {
    fn status_code(&self) -> StatusCode {
        match self {
            ApiError::BadRequest => StatusCode::BAD_REQUEST,
            ApiError::Forbidden => StatusCode::FORBIDDEN,
            ApiError::NotFound => StatusCode::NOT_FOUND,
            ApiError::Conflict => StatusCode::CONFLICT,
            ApiError::DatabaseError => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code()).body(self.to_string())
    }
}

/// Journalise l'erreur de base de données et renvoie une erreur générique.
pub fn database_error<E: std::fmt::Debug>(error: E) -> ApiError {
    eprintln!("{:?}", error);
    ApiError::DatabaseError
}

/// Charge les droits de l'appelant sur l'événement : 404 s'il n'existe pas, 403 si `allowed` refuse.
pub async fn require_event_access(
    state: &actix_web::web::Data<mairie360_api_lib::state::AppState>,
    event_id: u64,
    user_id: u64,
    allowed: impl Fn(&EventAccess) -> bool,
) -> Result<EventAccess, ApiError> {
    let mut access: EventAccess = state
        .get_smart_db()
        .fetch_one(
            &crate::database::event::access::view::EventAccessQueryView::new(event_id, user_id),
        )
        .await
        .map_err(database_error)?;
    access.caller_id = user_id as i32;

    if !access.exists {
        return Err(ApiError::NotFound);
    }
    if !allowed(&access) {
        return Err(ApiError::Forbidden);
    }
    Ok(access)
}
