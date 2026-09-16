use actix_web::{get, web, HttpResponse, Responder};
use mairie360_api_lib::security::AuthenticatedUser;
use mairie360_api_lib::state::AppState;

use crate::database::calendar::get::view::{Event, GetCalendarQueryView};
use crate::endpoints::error::{database_error, ApiError};
use crate::endpoints::v1::get::view::{GetCalendarParams, GetCalendarResultView};

#[utoipa::path(
    get,
    path = "calendar",
    params(GetCalendarParams),
    responses(
        (status = 200, description = "Events owned by or assigned to the caller in the time range, including recurring events whose rule overlaps it", body = GetCalendarResultView),
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
    params: web::Query<GetCalendarParams>,
) -> Result<impl Responder, ApiError> {
    if params.end < params.start {
        return Err(ApiError::BadRequest);
    }
    let events: Vec<Event> = state
        .get_smart_db()
        .fetch_all(&GetCalendarQueryView::new(
            params.start,
            params.end,
            auth_user.id,
        ))
        .await
        .map_err(database_error)?;

    Ok(HttpResponse::Ok().json(GetCalendarResultView {
        events: events.into_iter().map(Into::into).collect(),
    }))
}
