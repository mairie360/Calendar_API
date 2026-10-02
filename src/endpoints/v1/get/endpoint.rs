use actix_web::{get, web, HttpResponse, Responder};
use mairie360_api_lib::security::AuthenticatedUser;
use mairie360_api_lib::state::AppState;

use crate::database::calendar::get::view::{Event, GetCalendarQueryView};
use crate::endpoints::error::{database_error, ApiError};
use crate::endpoints::v1::get::view::{GetCalendarParams, GetCalendarResultView};

#[utoipa::path(
    get,
    path = "calendar",
    summary = "Read the calendar over a period",
    description = "Returns the events overlapping the requested period that the JWT user can \
                   see: every `Public` event, and the `Private` events they own or are assigned \
                   to.\n\n\
                   Recurring events whose rule overlaps the period are included even when their \
                   first occurrence is earlier: a week or a month can be displayed without \
                   replaying the recurrences client side.\n\n\
                   Both bounds are inclusive. `end` must not be before `start` and the period is \
                   at most 366 days wide, otherwise the response is `400`. No pagination.\n\n\
                   List view: neither the members nor the approval status are included, call \
                   `GET /api/v1/events/{event_id}/` for them.",
    params(GetCalendarParams),
    responses(
        (
            status = 200,
            description = "Events visible to the caller over the period, overlapping recurrences included.",
            body = GetCalendarResultView,
            example = json!({
                "events": [
                    {
                        "id": 21,
                        "name": "Conseil municipal",
                        "start": "2026-10-05T18:00:00Z",
                        "end": "2026-10-05T20:00:00Z",
                        "is_member": true,
                        "visibility": "Public",
                        "category": "meeting",
                        "service": "Secrétariat général",
                        "location": "Salle du conseil",
                        "recurrence": { "frequency": "monthly", "interval": 1, "days_of_week": null, "ends_on": "2027-06-30" }
                    }
                ]
            })
        ),
        (
            status = 400,
            description = "`start` or `end` missing or malformed, `end` before `start`, or a period wider than 366 days.",
            body = String,
            content_type = "text/plain",
            example = json!("Bad request.")
        ),
        (
            status = 401,
            description = "Missing `Authorization` header, invalid or expired JWT, or revoked session.",
            body = String,
            content_type = "text/plain",
            example = json!("Jeton expiré")
        ),
        (
            status = 500,
            description = "Database error.",
            body = String,
            content_type = "text/plain",
            example = json!("An error occurred while accessing the database.")
        ),
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
    if !params.is_valid() {
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
