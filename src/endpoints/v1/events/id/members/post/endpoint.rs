use actix_web::http::StatusCode;
use actix_web::{post, web, HttpResponse, Responder, ResponseError};
use mairie360_api_lib::database::error::DbError;
use mairie360_api_lib::error::ApiLibError;
use mairie360_api_lib::security::AuthenticatedUser;
use mairie360_api_lib::state::AppState;

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

impl From<ApiLibError> for AddMemberError {
    fn from(err: ApiLibError) -> Self {
        match err {
            // Un event_id inconnu déclenche une violation de clé étrangère.
            ApiLibError::Database(DbError::ForeignKeyViolation(_)) => AddMemberError::UnknownEvent,
            ApiLibError::Serialization(_) => AddMemberError::BadRequest,
            _ => AddMemberError::DatabaseError,
        }
    }
}

async fn add_member(
    state: web::Data<AppState>,
    view: PostMemberView,
    project_id: u64,
) -> Result<(), AddMemberError> {
    let query = AddUserToEventQueryView::new(view.user_id(), project_id);
    state.get_smart_db().execute(query).await?;
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
    path_params: web::Path<u64>,
) -> Result<impl Responder, AddMemberError> {
    let view = request_view.try_into()?;
    add_member(state, view, path_params.into_inner()).await?;
    Ok(HttpResponse::Ok().finish())
}
