use actix_web::http::StatusCode;
use actix_web::{HttpResponse, ResponseError};

use mairie360_api_lib::database::error::DbError;
use mairie360_api_lib::error::ApiLibError;
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

/// Maps a database error to the status it deserves, and logs it (MAIR-421):
///
/// - unique or foreign-key violation: `409`, logged as a warning (a concurrent write won);
/// - no row where one was expected: `404`, logged at debug level;
/// - anything else (connection, SQL, mapping): `500`, logged as an error with its full cause.
///
/// The response body stays generic: Postgres messages name tables and constraints.
pub fn database_error(error: ApiLibError) -> ApiError {
    match &error {
        ApiLibError::Database(DbError::UniqueViolation(_) | DbError::ForeignKeyViolation(_)) => {
            log::warn!(target: "calendar_api::database", "constraint violation: {error:?}");
            ApiError::Conflict
        }
        ApiLibError::Database(DbError::NotFound) => {
            log::debug!(target: "calendar_api::database", "no matching row: {error:?}");
            ApiError::NotFound
        }
        _ => {
            log::error!(target: "calendar_api::database", "database failure: {error:?}");
            ApiError::DatabaseError
        }
    }
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

#[cfg(test)]
mod tests {
    use super::*;

    fn db(error: DbError) -> ApiLibError {
        ApiLibError::Database(error)
    }

    #[test]
    fn database_errors_are_mapped_by_kind() {
        assert_eq!(
            database_error(db(DbError::UniqueViolation("events_pkey".into()))),
            ApiError::Conflict
        );
        assert_eq!(
            database_error(db(DbError::ForeignKeyViolation("fk_user".into()))),
            ApiError::Conflict
        );
        assert_eq!(database_error(db(DbError::NotFound)), ApiError::NotFound);
        assert_eq!(
            database_error(db(DbError::Internal("pool closed".into()))),
            ApiError::DatabaseError
        );
        assert_eq!(
            database_error(db(DbError::MappingError("bad row".into()))),
            ApiError::DatabaseError
        );
    }

    #[actix_web::test]
    async fn error_bodies_never_leak_the_cause() {
        let response =
            database_error(db(DbError::Internal("relation \"events\"".into()))).error_response();
        assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
        let body = actix_web::body::to_bytes(response.into_body())
            .await
            .unwrap();
        assert_eq!(body, "An error occurred while accessing the database.");
    }
}
