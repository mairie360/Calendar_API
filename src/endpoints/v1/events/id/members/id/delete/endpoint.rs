use actix_web::http::StatusCode;
use actix_web::{delete, web, HttpResponse, Responder, ResponseError};
use mairie360_api_lib::database::error::DbError;
use mairie360_api_lib::error::ApiLibError;
use mairie360_api_lib::security::AuthenticatedUser;
use mairie360_api_lib::state::AppState;

use crate::database::event::remove_member::view::RemoveUserFromEventQueryView;
use crate::endpoints::v1::events::id::members::id::delete::view::DeleteMemberParams;

#[derive(Debug, Clone, PartialEq)]
pub enum RemoveMemberError {
    DatabaseError,
    UnknownEvent,
    UnknownMember,
    NothingToDelete,
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
            RemoveMemberError::NothingToDelete => {
                write!(f, "Nothing to delete.")
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
            RemoveMemberError::NothingToDelete => StatusCode::NOT_FOUND,
        }
    }

    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code()).body(self.to_string())
    }
}

impl From<ApiLibError> for RemoveMemberError {
    fn from(err: ApiLibError) -> Self {
        match err {
            // `DELETE ... RETURNING` ne renvoie aucune ligne : il n'y avait rien à supprimer.
            ApiLibError::Database(DbError::NotFound) => RemoveMemberError::NothingToDelete,
            _ => RemoveMemberError::DatabaseError,
        }
    }
}

async fn remove_member(
    state: web::Data<AppState>,
    params: DeleteMemberParams,
) -> Result<(), RemoveMemberError> {
    let view = RemoveUserFromEventQueryView::new(params.member_id, params.event_id);

    state.get_smart_db().fetch_scalar::<i32, _>(&view).await?;

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
pub async fn remove_event_member(
    state: web::Data<AppState>,
    _: AuthenticatedUser,
    params: web::Path<DeleteMemberParams>,
) -> Result<impl Responder, RemoveMemberError> {
    let params = params.try_into()?;
    remove_member(state, params).await?;
    Ok(HttpResponse::NoContent().finish())
}
