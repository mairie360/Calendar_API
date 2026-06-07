use actix_web::http::StatusCode;
use actix_web::{patch, web, HttpResponse, Responder, ResponseError};
use mairie360_api_lib::pool::AppState;
use mairie360_api_lib::security::AuthenticatedUser;

use crate::endpoints::v1::events::id::patch::view::{PatchEventParams, PatchEventView};

#[derive(Debug, Clone, PartialEq)]
pub enum PatchEventError {
    DatabaseError,
    BadParams,
    BadRequest,
    UnknownEvent,
}

impl std::fmt::Display for PatchEventError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PatchEventError::DatabaseError => {
                write!(f, "An error occurred while accessing the database.")
            }
            PatchEventError::BadParams => {
                write!(f, "Bad params.")
            }
            PatchEventError::BadRequest => {
                write!(f, "Bad request.")
            }
            PatchEventError::UnknownEvent => {
                write!(f, "Unknown event.")
            }
        }
    }
}

impl ResponseError for PatchEventError {
    fn status_code(&self) -> StatusCode {
        match self {
            PatchEventError::DatabaseError => StatusCode::INTERNAL_SERVER_ERROR,
            PatchEventError::BadParams => StatusCode::BAD_REQUEST,
            PatchEventError::BadRequest => StatusCode::BAD_REQUEST,
            PatchEventError::UnknownEvent => StatusCode::NOT_FOUND,
        }
    }

    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code()).body(self.to_string())
    }
}

async fn trigger_patch_event(
    state: web::Data<AppState>,
    user_id: u64,
    params: PatchEventParams,
    view: PatchEventView,
) -> Result<(), PatchEventError> {
    let pool = match state.db_pool.clone() {
        Some(pool) => pool,
        None => return Err(PatchEventError::DatabaseError),
    };

    //query

    // update cache

    Ok(())
}

#[utoipa::path(
    patch,
    path = "",
    params(
        ("event_id" = u64, Path, description = "Event ID"),
        ("reccurent" = bool, Query, description = "Whether the event is recurring")
    ),
    responses(
        (status = 200, description = "Event patched successfully"),
        (status = 400, description = "Bad request"),
        (status = 404, description = "Event not found"),
        (status = 500, description = "Internal server error")
    ),
    request_body = PatchEventView,
    tag = "Events",
    security(
        ("jwt" = [])
    )
)]
#[patch("/")]
pub async fn patch_event(
    state: web::Data<AppState>,
    auth_user: AuthenticatedUser,
    path_params: web::Query<PatchEventParams>,
    view: web::Json<PatchEventView>,
) -> Result<impl Responder, PatchEventError> {
    let param = path_params
        .try_into()
        .map_err(|_| PatchEventError::BadRequest)?;
    let view = view.try_into().map_err(|_| PatchEventError::BadRequest)?;
    let calendar = trigger_patch_event(state, auth_user.id, param, view).await?;
    Ok(HttpResponse::Ok().json(calendar))
}
