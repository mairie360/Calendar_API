use actix_web::http::StatusCode;
use actix_web::{get, web, HttpResponse, Responder, ResponseError};
use mairie360_api_lib::database::error::DbError;
use mairie360_api_lib::error::ApiLibError;
use mairie360_api_lib::security::AuthenticatedUser;
use mairie360_api_lib::state::AppState;

use crate::database::event::get::view::{GetEventQueryResultView, GetEventQueryView};
use crate::database::event::get_event_members::view::{GetEventMemberQueryView, Member};
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

impl From<ApiLibError> for GetEventError {
    fn from(err: ApiLibError) -> Self {
        match err {
            ApiLibError::Database(DbError::NotFound) => GetEventError::UnknownEvent,
            _ => GetEventError::DatabaseError,
        }
    }
}

async fn trigger_get_event(
    state: web::Data<AppState>,
    event_id: u64,
) -> Result<GetEventResultView, GetEventError> {
    let db = state.get_smart_db();

    let event_view = GetEventQueryView::new(event_id);
    let result = db
        .fetch_one::<GetEventQueryResultView, _>(&event_view)
        .await?;

    let members_view = GetEventMemberQueryView::new(event_id);
    let members: Vec<Member> = db
        .fetch_all::<Member, _>(&members_view)
        .await
        .map_err(|_| GetEventError::DatabaseError)?;

    Ok(GetEventResultView::new(
        event_id,
        result.recurrence_id().map(|id| id as u64),
        result.start_date(),
        result.end_date(),
        result.name(),
        result.description(),
        None,
        result.owner_id().unwrap_or_default() as u64,
        members
            .into_iter()
            .map(|member| {
                crate::endpoints::v1::events::id::get::view::Member::new(
                    member.user_id() as u64,
                    member.validation_status().to_string().into(),
                )
            })
            .collect(),
    ))
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
    _: AuthenticatedUser,
    path_params: web::Path<u64>,
) -> Result<impl Responder, GetEventError> {
    let params = path_params.into_inner();
    let calendar = trigger_get_event(state, params).await?;
    Ok(HttpResponse::Ok().json(calendar))
}
