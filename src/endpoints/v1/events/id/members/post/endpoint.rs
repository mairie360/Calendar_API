use actix_web::{post, web, HttpResponse, Responder};
use mairie360_api_lib::security::AuthenticatedUser;
use mairie360_api_lib::state::AppState;

use crate::database::event::access::view::EventAccess;
use crate::database::event::add_member::view::AddUserToEventQueryView;
use crate::database::event::validation::view::{
    CanAssignUserQueryView, RefreshEventValidationQueryView,
};
use crate::endpoints::error::{database_error, require_event_access, ApiError};
use crate::endpoints::v1::events::id::members::post::view::PostMemberView;

#[utoipa::path(
    post,
    path = "",
    params(
        ("event_id" = u64, Path, description = "Event ID")
    ),
    request_body = PostMemberView,
    responses(
        (status = 201, description = "Member added; the event validation is recomputed"),
        (status = 403, description = "The caller cannot manage the members, or the user is outside their assignment scope"),
        (status = 404, description = "Unknown event"),
        (status = 409, description = "The user is already a member"),
        (status = 500, description = "Internal server error")
    ),
    tag = "Events",
    security(
        ("jwt" = [])
    )
)]
#[post("/")]
pub async fn add_event_member(
    state: web::Data<AppState>,
    auth_user: AuthenticatedUser,
    event_id: web::Path<u64>,
    view: web::Json<PostMemberView>,
) -> Result<impl Responder, ApiError> {
    let event_id = event_id.into_inner();
    require_event_access(
        &state,
        event_id,
        auth_user.id,
        EventAccess::can_manage_members,
    )
    .await?;

    let db = state.get_smart_db();
    let assignable: bool = db
        .fetch_scalar(&CanAssignUserQueryView::new(auth_user.id, view.user_id))
        .await
        .map_err(database_error)?;
    if !assignable {
        return Err(ApiError::Forbidden);
    }
    // Index unique (event_id, user_id) : un doublon échoue à l'insertion.
    db.execute(AddUserToEventQueryView::new(view.user_id, event_id))
        .await
        .map_err(|_| ApiError::Conflict)?;
    db.execute(RefreshEventValidationQueryView::new(event_id))
        .await
        .map_err(database_error)?;

    Ok(HttpResponse::Created().finish())
}
