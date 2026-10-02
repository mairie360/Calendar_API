use actix_web::{get, web, HttpResponse, Responder};
use mairie360_api_lib::database::db_interface::id_from_sql;
use mairie360_api_lib::security::AuthenticatedUser;
use mairie360_api_lib::state::AppState;

use crate::database::event::access::view::EventAccess;
use crate::database::event::get::view::{GetEventQueryResultView, GetEventQueryView};
use crate::database::event::get_event_members::view::{GetEventMemberQueryView, Member};
use crate::endpoints::error::{database_error, require_event_access, ApiError};
use crate::endpoints::v1::events::id::get::view::{
    EventPermissionsView, GetEventResultView, Member as MemberView,
};

pub async fn load_event(
    state: &web::Data<AppState>,
    event_id: u64,
    access: &EventAccess,
) -> Result<GetEventResultView, ApiError> {
    let db = state.get_smart_db();
    let event_view = GetEventQueryView::new(event_id);
    let members_view = GetEventMemberQueryView::new(event_id);
    let (events, members) = futures_util::try_join!(
        db.fetch_all::<GetEventQueryResultView, _>(&event_view),
        db.fetch_all::<Member, _>(&members_view),
    )
    .map_err(database_error)?;
    let event = events.into_iter().next().ok_or(ApiError::NotFound)?;
    let input = event.to_input();

    Ok(GetEventResultView {
        id: event_id,
        name: input.name,
        description: input.description,
        events_start_time: input.start,
        events_end_time: input.end,
        visibility: input.visibility,
        category: input.category,
        service: input.service,
        location: input.location,
        recurrence: input.recurrence,
        owner: event.owner_id().map(id_from_sql),
        created_by: event.created_by().map(id_from_sql),
        members: members
            .into_iter()
            .map(|member| MemberView {
                id: id_from_sql(member.user_id()),
                validation_status: member.validation_status(),
            })
            .collect(),
        approval_status: access.approval_status,
        permissions: EventPermissionsView {
            can_edit: access.can_edit(),
            can_delete: access.can_delete(),
            can_validate: access.can_validate(),
        },
    })
}

#[utoipa::path(
    get,
    path = "",
    summary = "Read the detail of an event",
    description = "Returns a full event: description, visibility, category, location, \
                   recurrence, members, approval status, and the **caller's rights** on the \
                   event.\n\n\
                   The `permissions` field saves the client from re-deriving the access rules: \
                   hide the buttons whose flag is `false`.\n\n\
                   A `Public` event is readable by every authenticated user. A `Private` event \
                   only by its members and its creator: anyone else gets `403`, not `404`.",
    params(
        ("event_id" = u64, Path, description = "Event id.", example = 21)
    ),
    responses(
        (
            status = 200,
            description = "Detail of the event and rights of the caller.",
            body = GetEventResultView,
            example = json!({
                "id": 21,
                "name": "Conseil municipal",
                "description": "Ordre du jour envoyé une semaine avant",
                "events_start_time": "2026-10-05T18:00:00Z",
                "events_end_time": "2026-10-05T20:00:00Z",
                "visibility": "Public",
                "category": "meeting",
                "service": "Secrétariat général",
                "location": "Salle du conseil",
                "recurrence": null,
                "owner": 42,
                "created_by": 42,
                "members": [
                    { "id": 42, "validation_status": "pending" },
                    { "id": 51, "validation_status": "pending" }
                ],
                "approval_status": "pending",
                "permissions": { "can_edit": true, "can_delete": true, "can_validate": false }
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
pub async fn get_event(
    state: web::Data<AppState>,
    auth_user: AuthenticatedUser,
    event_id: web::Path<u64>,
) -> Result<impl Responder, ApiError> {
    let event_id = event_id.into_inner();
    let access =
        require_event_access(&state, event_id, auth_user.id, EventAccess::can_view).await?;
    let event = load_event(&state, event_id, &access).await?;
    Ok(HttpResponse::Ok().json(event))
}
