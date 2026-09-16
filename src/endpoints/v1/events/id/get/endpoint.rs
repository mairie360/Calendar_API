use actix_web::{get, web, HttpResponse, Responder};
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
        owner: event.owner_id().map(|id| id as u64),
        created_by: event.created_by().map(|id| id as u64),
        members: members
            .into_iter()
            .map(|member| MemberView {
                id: member.user_id() as u64,
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
    params(
        ("event_id" = u64, Path, description = "Event ID")
    ),
    responses(
        (status = 200, description = "Event details", body = GetEventResultView),
        (status = 403, description = "The caller is not assigned to the event"),
        (status = 404, description = "Unknown event"),
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
    auth_user: AuthenticatedUser,
    event_id: web::Path<u64>,
) -> Result<impl Responder, ApiError> {
    let event_id = event_id.into_inner();
    let access =
        require_event_access(&state, event_id, auth_user.id, EventAccess::can_view).await?;
    let event = load_event(&state, event_id, &access).await?;
    Ok(HttpResponse::Ok().json(event))
}
