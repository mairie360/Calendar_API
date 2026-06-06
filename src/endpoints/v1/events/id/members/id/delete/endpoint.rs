use actix_web::http::StatusCode;
use actix_web::{delete, web, HttpResponse, Responder, ResponseError};
use mairie360_api_lib::pool::AppState;
use mairie360_api_lib::security::AuthenticatedUser;

use crate::endpoints::v1::events::id::members::id::delete::view::DeleteMemberParams;

#[derive(Debug, Clone, PartialEq)]
pub enum RemoveMemberError {
    DatabaseError,
    UnknownEvent,
    UnknownMember,
}

impl std::fmt::Display for RemoveMemberError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RemoveMemberError::DatabaseError => {
                write!(f, "An error occurred while accessing the database.")
            }
            RemoveMemberError::UnknownEvent => {
                write!(f, "Unknown event.")
            }
            RemoveMemberError::UnknownMember => {
                write!(f, "Unknown member.")
            }
        }
    }
}

impl ResponseError for RemoveMemberError {
    fn status_code(&self) -> StatusCode {
        match self {
            RemoveMemberError::DatabaseError => StatusCode::INTERNAL_SERVER_ERROR,
            RemoveMemberError::UnknownEvent => StatusCode::NOT_FOUND,
            RemoveMemberError::UnknownMember => StatusCode::NOT_FOUND,
        }
    }

    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code()).body(self.to_string())
    }
}

async fn delete_event(
    state: web::Data<AppState>,
    params: DeleteMemberParams,
) -> Result<(), RemoveMemberError> {
    let pool = match state.db_pool.clone() {
        Some(pool) => pool,
        None => return Err(RemoveMemberError::DatabaseError),
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
        DeleteMemberParams
    ),
    security(
        ("jwt" = [])
    )
)]
#[delete("/")]
pub async fn delete(
    state: web::Data<AppState>,
    _: AuthenticatedUser,
    params: web::Query<DeleteMemberParams>,
) -> Result<impl Responder, RemoveMemberError> {
    let params = params.try_into()?;
    delete_event(state, params).await?;
    Ok(HttpResponse::NoContent().finish())
}
