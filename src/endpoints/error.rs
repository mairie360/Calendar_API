use actix_web::http::StatusCode;
use actix_web::{HttpResponse, ResponseError};

use mairie360_api_lib::smart_db::SmartTransaction;

use crate::database::event::access::view::{EventAccess, EventAccessQueryView, LockEventQueryView};

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

/// Rejects the caller unless `allowed` accepts its rights: 404 for an unknown event, 403 otherwise.
fn check_event_access(
    mut access: EventAccess,
    user_id: u64,
    allowed: impl Fn(&EventAccess) -> bool,
) -> Result<EventAccess, ApiError> {
    access.caller_id = user_id as i32;
    if !access.exists {
        return Err(ApiError::NotFound);
    }
    if !allowed(&access) {
        return Err(ApiError::Forbidden);
    }
    Ok(access)
}

/// Loads the caller's rights on the event for a read: 404 if it does not exist, 403 if `allowed`
/// refuses.
pub async fn require_event_access(
    state: &actix_web::web::Data<mairie360_api_lib::state::AppState>,
    event_id: u64,
    user_id: u64,
    allowed: impl Fn(&EventAccess) -> bool,
) -> Result<EventAccess, ApiError> {
    let access: EventAccess = state
        .get_smart_db()
        .fetch_one(&EventAccessQueryView::new(event_id, user_id))
        .await
        .map_err(database_error)?;
    check_event_access(access, user_id, allowed)
}

/// Same as [`require_event_access`] inside the transaction of a write: the event row is locked
/// first, so the rights cannot change between this check and the write (MAIR-420).
pub async fn require_event_access_in(
    tx: &mut SmartTransaction,
    event_id: u64,
    user_id: u64,
    allowed: impl Fn(&EventAccess) -> bool,
) -> Result<EventAccess, ApiError> {
    tx.execute(&LockEventQueryView::new(event_id))
        .await
        .map_err(database_error)?;
    let access: EventAccess = tx
        .fetch_one(&EventAccessQueryView::new(event_id, user_id))
        .await
        .map_err(database_error)?;
    check_event_access(access, user_id, allowed)
}
