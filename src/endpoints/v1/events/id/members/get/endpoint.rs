use actix_web::{get, web, HttpResponse, Responder};
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
    params(
        ("event_id" = u64, Path, description = "Event ID")
    ),
    responses(
        (status = 200, description = "Event members", body = GetMembersResultView),
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
                id: member.user_id() as u64,
                validation_status: member.validation_status(),
            })
            .collect(),
    }))
}
