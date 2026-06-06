use actix_web::http::StatusCode;
use actix_web::{get, web, HttpResponse, Responder, ResponseError};
use mairie360_api_lib::pool::AppState;
use mairie360_api_lib::security::AuthenticatedUser;

use crate::endpoints::v1::events::id::members::get::view::GetMembersResultView;

#[derive(Debug, Clone, PartialEq)]
pub enum GetMembersError {
    BadRequest,
    DatabaseError,
    UnknownEvent,
}

impl std::fmt::Display for GetMembersError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GetMembersError::BadRequest => {
                write!(f, "Bad request.")
            }
            GetMembersError::DatabaseError => {
                write!(f, "An error occurred while accessing the database.")
            }
            GetMembersError::UnknownEvent => {
                write!(f, "Unknown event.")
            }
        }
    }
}

impl ResponseError for GetMembersError {
    fn status_code(&self) -> StatusCode {
        match self {
            GetMembersError::BadRequest => StatusCode::BAD_REQUEST,
            GetMembersError::DatabaseError => StatusCode::INTERNAL_SERVER_ERROR,
            GetMembersError::UnknownEvent => StatusCode::NOT_FOUND,
        }
    }

    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code()).body(self.to_string())
    }
}

async fn get_members(
    state: web::Data<AppState>,
    event_id: u64,
) -> Result<GetMembersResultView, GetMembersError> {
    //get_cache
    let pool = match state.db_pool.clone() {
        Some(pool) => pool,
        None => return Err(GetMembersError::DatabaseError),
    };

    //query

    // update cache

    Ok(GetMembersResultView::new(vec![]))
}

#[utoipa::path(
    get,
    path = "",
    params(
        ("id" = u64, Path, description = "Event ID")
    ),
    responses(
        (status = 200, description = "Event members", body = GetMembersResultView),
        (status = 400, description = "Bad request"),
        (status = 404, description = "Event not found"),
        (status = 500, description = "Internal server error")
    ),
    tag = "Events",
    security(
        ("jwt" = [])
    )
)]
#[get("/")]
pub async fn get(
    state: web::Data<AppState>,
    _: AuthenticatedUser,
    path_params: web::Query<Option<u64>>,
) -> Result<impl Responder, GetMembersError> {
    let params = match path_params.into_inner() {
        Some(params) => params,
        None => return Err(GetMembersError::BadRequest),
    };
    let members = get_members(state, params).await?;
    Ok(HttpResponse::Ok().json(members))
}
