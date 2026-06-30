use actix_web::http::StatusCode;
use actix_web::{post, web, HttpResponse, Responder, ResponseError};
use mairie360_api_lib::pool::AppState;
use mairie360_api_lib::security::AuthenticatedUser;

use crate::endpoints::v1::events::post::view::{PostEventResultView, PostEventView};

#[derive(Debug, Clone, PartialEq)]
pub enum PostEventError {
    BadRequest,
    DatabaseError,
}

impl std::fmt::Display for PostEventError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PostEventError::BadRequest => {
                write!(f, "Bad request")
            }
            PostEventError::DatabaseError => {
                write!(f, "An error occurred while accessing the database.")
            }
        }
    }
}

impl ResponseError for PostEventError {
    fn status_code(&self) -> StatusCode {
        match self {
            PostEventError::BadRequest => StatusCode::BAD_REQUEST,
            PostEventError::DatabaseError => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code()).body(self.to_string())
    }
}

async fn trigger_create_event(
    state: web::Data<AppState>,
    user_id: u64,
    view: PostEventView,
) -> Result<PostEventResultView, PostEventError> {
    let pool = match state.db_pool.clone() {
        Some(pool) => pool,
        None => return Err(PostEventError::DatabaseError),
    };

    //query

    Ok(PostEventResultView::new(0))
}

#[utoipa::path(
    post,
    path = "",
    responses(
        (status = 200, description = "Event created successfully", body = PostEventResultView),
        (status = 400, description = "Bad request"),
        (status = 500, description = "Internal server error")
    ),
    tag = "Events",
    request_body = PostEventView,
    security(
        ("jwt" = [])
    )
)]
#[post("/")]
pub async fn create_event(
    state: web::Data<AppState>,
    auth_user: AuthenticatedUser,
    request_view: web::Json<PostEventView>,
) -> Result<impl Responder, PostEventError> {
    let view = match request_view.try_into() {
        Ok(view) => view,
        Err(_) => return Err(PostEventError::BadRequest),
    };
    let calendar = trigger_create_event(state, auth_user.id, view).await?;
    Ok(HttpResponse::Ok().json(calendar))
}
