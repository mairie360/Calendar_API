use actix_web::http::StatusCode;
use actix_web::{get, web, HttpResponse, Responder, ResponseError};
use mairie360_api_lib::pool::AppState;
use mairie360_api_lib::security::AuthenticatedUser;

use crate::endpoints::v1::get::view::{GetCalendarParams, GetCalendarResultView};

#[derive(Debug, Clone, PartialEq)]
pub enum GetCalendarError {
    BadParams,
    DatabaseError,
}

impl std::fmt::Display for GetCalendarError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GetCalendarError::BadParams => {
                write!(f, "Bad parameters")
            }
            GetCalendarError::DatabaseError => {
                write!(f, "An error occurred while accessing the database.")
            }
        }
    }
}

impl ResponseError for GetCalendarError {
    fn status_code(&self) -> StatusCode {
        match self {
            GetCalendarError::BadParams => StatusCode::BAD_REQUEST,
            GetCalendarError::DatabaseError => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code()).body(self.to_string())
    }
}

async fn trigger_get_calendar(
    state: web::Data<AppState>,
    user_id: u64,
    params: GetCalendarParams,
) -> Result<GetCalendarResultView, GetCalendarError> {
    //get_cache
    let pool = match state.db_pool.clone() {
        Some(pool) => pool,
        None => return Err(GetCalendarError::DatabaseError),
    };

    //query

    // update cache

    Ok(GetCalendarResultView::new(vec![]))
}

#[utoipa::path(
    get,
    path = "calendar",
    params(GetCalendarParams),
    responses(
        (status = 200, description = "Calendar in the specified time range", body = GetCalendarResultView),
        (status = 400, description = "Bad request"),
        (status = 500, description = "Internal server error")
    ),
    tag = "Calendar",
    security(
        ("jwt" = [])
    )
)]
#[get("/calendar")]
pub async fn get_calendar(
    state: web::Data<AppState>,
    auth_user: AuthenticatedUser,
    path_params: web::Query<GetCalendarParams>,
) -> Result<impl Responder, GetCalendarError> {
    let params = path_params.try_into()?;
    let calendar = trigger_get_calendar(state, auth_user.id, params).await?;
    Ok(HttpResponse::Ok().json(calendar))
}
