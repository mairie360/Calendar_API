use actix_web::{get, web, HttpResponse, Responder};
use mairie360_api_lib::security::AuthenticatedUser;
use mairie360_api_lib::state::AppState;

use crate::database::calendar::get::view::{Event, GetCalendarQueryView, CALENDAR_CACHE};
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
                   Each event carries its `members` (sorted by id) and their \
                   `validation_status` (the event's approval status, repeated on every member), \
                   exactly like `GET /api/v1/events/{event_id}/`: a period is displayed in one \
                   call, with no per-event request. The members are only exposed when the caller \
                   could read the event's detail (`Public` event, member or creator); otherwise \
                   the list is empty. The description, the `permissions` and the owner stay on \
                   the detail route.\n\n\
                   **Cache:** the page is cached in Redis for 30 seconds per caller and period \
                   (`start`, `end`). Every write of this API (event created, edited or deleted, \
                   member added or removed, approval decided) invalidates it once committed, so \
                   a client reads its own writes immediately. Changes made outside this API (a \
                   user deleted in Core removing their memberships) show up within those 30 \
                   seconds. When Redis is unreachable the page is read from the database.",
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
                        "recurrence": { "frequency": "monthly", "interval": 1, "days_of_week": null, "ends_on": "2027-06-30" },
                        "members": [
                            { "id": 42, "validation_status": "validated" },
                            { "id": 51, "validation_status": "validated" }
                        ]
                    },
                    {
                        "id": 34,
                        "name": "Permanence des élus",
                        "start": "2026-10-12T09:00:00Z",
                        "end": "2026-10-12T12:00:00Z",
                        "is_member": false,
                        "visibility": "Public",
                        "category": "other",
                        "service": null,
                        "location": "Mairie annexe",
                        "recurrence": null,
                        "members": []
                    }
                ]
            })
        ),
        (
            status = 400,
            description = "`start` or `end` missing or malformed, outside `1970-01-01T00:00:00Z` – `2999-12-31T23:59:59Z`, `end` before `start`, or a period wider than 366 days. The text body names the faulty parameter, except for a missing or malformed one (actix's `Query deserialize error`).",
            body = String,
            content_type = "text/plain",
            example = json!("Invalid `end`: must be at most 366 days after `start`.")
        ),
        (
            status = 401,
            description = "Missing `Authorization` header, invalid or expired JWT, or revoked session.",
            body = String,
            content_type = "text/plain",
            example = json!("Jeton expiré")
        ),
        (
            status = 429,
            description = "The caller exceeded their request quota (`RATE_LIMIT_PER_SECOND` per second on \
                           average, bursts of `RATE_LIMIT_BURST`, counted per user). The \
                           `Retry-After` header gives the seconds to wait.",
            body = String,
            content_type = "text/plain",
            example = json!("Too many requests, retry in 1s.")
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
    params.validate()?;
    let db = state.get_smart_db();
    // Keyed by the caller: the page depends on who asks (private events, members' visibility).
    let key = CALENDAR_CACHE.for_user(auth_user.id, &(params.start, params.end));
    let page = db
        .cached(&key, async || {
            let events: Vec<Event> = db
                .fetch_all(&GetCalendarQueryView::new(
                    params.start,
                    params.end,
                    auth_user.id,
                ))
                .await
                .map_err(database_error)?;
            Ok::<_, ApiError>(GetCalendarResultView {
                events: events.into_iter().map(Into::into).collect(),
            })
        })
        .await?;

    Ok(HttpResponse::Ok().json(page))
}
