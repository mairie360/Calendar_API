use actix_web::http::StatusCode;
use actix_web::{get, web, HttpResponse, Responder, ResponseError};
use mairie360_api_lib::pool::AppState;
use mairie360_api_lib::security::AuthenticatedUser;

use crate::endpoints::v1::events::id::get::view::GetEventResultView;

#[derive(Debug, Clone, PartialEq)]
pub enum GetEventError {
    DatabaseError,
    UnknownEvent,
}

impl std::fmt::Display for GetEventError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GetEventError::DatabaseError => {
                write!(f, "An error occurred while accessing the database.")
            }
            GetEventError::UnknownEvent => {
                write!(f, "Unknown event.")
            }
        }
    }
}

impl ResponseError for GetEventError {
    fn status_code(&self) -> StatusCode {
        match self {
            GetEventError::DatabaseError => StatusCode::INTERNAL_SERVER_ERROR,
            GetEventError::UnknownEvent => StatusCode::NOT_FOUND,
        }
    }

    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code()).body(self.to_string())
    }
}

async fn trigger_get_event(
    state: web::Data<AppState>,
    user_id: u64,
    event_id: u64,
) -> Result<(), GetEventError> {
    //get_cache
    let pool = match state.db_pool.clone() {
        Some(pool) => pool,
        None => return Err(GetEventError::DatabaseError),
    };

    //query

    // update cache

    // Ok(GetEventResultView::new())
    Ok(())
}

#[utoipa::path(
    get,
    path = "",
    params(
        ("event_id" = u64, Path, description = "Event ID")
    ),
    responses(
        (status = 200, description = "Event details", body = GetEventResultView),
        (status = 400, description = "Bad request"),
        (status = 500, description = "Internal server error")
    ),
    tag = "Events",
    security(
        ("jwt" = [])
    )
)]
#[get("/")]
pub async fn get_event(
    state: web::Data<AppState>,
    auth_user: AuthenticatedUser,
    path_params: web::Query<u64>,
) -> Result<impl Responder, GetEventError> {
    let params = path_params.into_inner();
    let calendar = trigger_get_event(state, auth_user.id, params).await?;
    Ok(HttpResponse::Ok().json(calendar))
}
