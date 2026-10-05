use actix_web::{get, web, HttpResponse, Responder};
use mairie360_api_lib::database::db_interface::id_from_sql;
use mairie360_api_lib::security::AuthenticatedUser;
use mairie360_api_lib::state::AppState;

use crate::database::event::access::view::EventAccess;
use crate::database::event::get_event_members::view::{GetEventMemberQueryView, Member};
use crate::endpoints::error::{database_error, require_event_access, ApiError};
use crate::endpoints::v1::events::id::get::view::Member as MemberView;
use crate::endpoints::v1::events::id::members::get::view::GetMembersResultView;

#[utoipa::path(
    get,
    path = "",
    summary = "List the members of an event",
    description = "Returns the users assigned to the event. Their `validation_status` is the \
                   approval status of the event, the same for every member: the decision is taken \
                   for the whole event. Readable by whoever can read the event (anyone for a \
                   `Public` event, its members and creator for a `Private` one).\n\n\
                   Only Core API ids are returned: pass them to Core API's \
                   `GET /api/v1/user/?ids=1,2,3` for the names.\n\n\
                   `GET /api/v1/events/{event_id}/` already returns this list with the event \
                   detail: this endpoint refreshes it alone.",
    params(
        ("event_id" = u64, Path, description = "Event id.", example = 21)
    ),
    responses(
        (
            status = 200,
            description = "Members of the event, sorted by id.",
            body = GetMembersResultView,
            example = json!({
                "members": [
                    { "id": 42, "validation_status": "pending" },
                    { "id": 51, "validation_status": "pending" }
                ]
            })
        ),
        (
            status = 400,
            description = "A path segment is not an integer.",
            body = String,
            content_type = "text/plain",
            example = json!("Path deserialize error: can not parse `abc` to a u64")
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
            status = 403,
            description = "The event is `Private` and the caller is neither one of its members nor its creator.",
            body = String,
            content_type = "text/plain",
            example = json!("Forbidden.")
        ),
        (
            status = 404,
            description = "No event has this id.",
            body = String,
            content_type = "text/plain",
            example = json!("Unknown event.")
        ),
        (
            status = 500,
            description = "Database error.",
            body = String,
            content_type = "text/plain",
            example = json!("An error occurred while accessing the database.")
        ),
    ),
    tag = "Events",
    security(
        ("jwt" = [])
    )
)]
#[get("/")]
pub async fn get_event_members(
    state: web::Data<AppState>,
    auth_user: AuthenticatedUser,
    event_id: web::Path<u64>,
) -> Result<impl Responder, ApiError> {
    let event_id = event_id.into_inner();
    require_event_access(&state, event_id, auth_user.id, EventAccess::can_view).await?;

    let members: Vec<Member> = state
        .get_smart_db()
        .fetch_all(&GetEventMemberQueryView::new(event_id))
        .await
        .map_err(database_error)?;

    Ok(HttpResponse::Ok().json(GetMembersResultView {
        members: members
            .into_iter()
            .map(|member| MemberView {
                id: id_from_sql(member.user_id()),
                validation_status: member.validation_status(),
            })
            .collect(),
    }))
}
