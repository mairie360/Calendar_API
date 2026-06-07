use actix_web::http::StatusCode;
use actix_web::{post, web, HttpResponse, Responder, ResponseError};
use mairie360_api_lib::pool::AppState;
use mairie360_api_lib::security::AuthenticatedUser;

use crate::endpoints::v1::events::id::members::post::view::PostMemberView;

#[derive(Debug, Clone, PartialEq)]
pub enum AddMemberError {
    BadRequest,
    DatabaseError,
    UnknownEvent,
}

impl std::fmt::Display for AddMemberError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AddMemberError::BadRequest => {
                write!(f, "Bad request")
            }
            AddMemberError::DatabaseError => {
                write!(f, "An error occurred while accessing the database.")
            }
            AddMemberError::UnknownEvent => {
                write!(f, "Unknown event.")
            }
        }
    }
}

impl ResponseError for AddMemberError {
    fn status_code(&self) -> StatusCode {
        match self {
            AddMemberError::BadRequest => StatusCode::BAD_REQUEST,
            AddMemberError::DatabaseError => StatusCode::INTERNAL_SERVER_ERROR,
            AddMemberError::UnknownEvent => StatusCode::NOT_FOUND,
        }
    }

    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code()).body(self.to_string())
    }
}

async fn add_member(
    state: web::Data<AppState>,
    user_id: u64,
    view: PostMemberView,
) -> Result<(), AddMemberError> {
    let pool = match state.db_pool.clone() {
        Some(pool) => pool,
        None => return Err(AddMemberError::DatabaseError),
    };

    //query

    Ok(())
}

#[utoipa::path(
    post,
    path = "",
    params(
        ("event_id" = u64, Path, description = "Event ID")
    ),

    responses(
        (status = 200, description = "Member added successfully", body = PostMemberView),
        (status = 400, description = "Bad request"),
        (status = 404, description = "Unknown event"),
        (status = 500, description = "Internal server error")
    ),
    request_body = PostMemberView,
    security(
        ("jwt" = [])
    ),
    tag = "Events",
)]
#[post("/")]
pub async fn add_event_member(
    state: web::Data<AppState>,
    auth_user: AuthenticatedUser,
    request_view: web::Json<PostMemberView>,
) -> Result<impl Responder, AddMemberError> {
    let view = request_view
        .try_into()
        .map_err(|_| AddMemberError::BadRequest)?;
    let calendar = add_member(state, auth_user.id, view).await?;
    Ok(HttpResponse::Ok().json(calendar))
}
