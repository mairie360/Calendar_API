use actix_web::http::StatusCode;
use actix_web::{post, web, HttpResponse, Responder, ResponseError};
use mairie360_api_lib::pool::AppState;
use mairie360_api_lib::security::AuthenticatedUser;

use crate::database::event::add_member::query::add_user_to_event_query;
use crate::database::event::add_member::view::AddUserToEventQueryView;
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
    view: PostMemberView,
    project_id: u64,
) -> Result<(), AddMemberError> {
    let pool = match state.db_pool.clone() {
        Some(pool) => pool,
        None => return Err(AddMemberError::DatabaseError),
    };

    let view = AddUserToEventQueryView::new(view.user_id(), project_id);
    let result = add_user_to_event_query(view, pool)
        .await
        .map_err(|_| AddMemberError::DatabaseError)?;

    if result != 1 {
        return Err(AddMemberError::UnknownEvent);
    }
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
    _: AuthenticatedUser,
    request_view: web::Json<PostMemberView>,
    path_params: web::Query<u64>,
) -> Result<impl Responder, AddMemberError> {
    let calendar = add_member(state, request_view.into_inner(), path_params.into_inner()).await?;
    Ok(HttpResponse::Ok().json(calendar))
}
