use actix_web::{delete, web, HttpResponse, Responder};
use mairie360_api_lib::security::AuthenticatedUser;
use mairie360_api_lib::state::AppState;

use crate::database::event::access::view::EventAccess;
use crate::database::event::remove_member::view::RemoveUserFromEventQueryView;
use crate::database::event::validation::view::RefreshEventValidationQueryView;
use crate::endpoints::error::{database_error, require_event_access, ApiError};

#[utoipa::path(
    delete,
    path = "",
    params(
        ("event_id" = u64, Path, description = "Event ID"),
        ("member_id" = u64, Path, description = "User ID of the member")
    ),
    responses(
        (status = 204, description = "Member removed; the event validation is recomputed"),
        (status = 403, description = "The caller cannot manage the members"),
        (status = 404, description = "Unknown event or member"),
        (status = 500, description = "Internal server error")
    ),
    tag = "Events",
    security(
        ("jwt" = [])
    )
)]
#[delete("/")]
pub async fn remove_event_member(
    state: web::Data<AppState>,
    auth_user: AuthenticatedUser,
    path: web::Path<(u64, u64)>,
) -> Result<impl Responder, ApiError> {
    let (event_id, member_id) = path.into_inner();
    require_event_access(
        &state,
        event_id,
        auth_user.id,
        EventAccess::can_manage_members,
    )
    .await?;

    let db = state.get_smart_db();
    let removed: Vec<i32> = db
        .fetch_all(&RemoveUserFromEventQueryView::new(member_id, event_id))
        .await
        .map_err(database_error)?;
    if removed.is_empty() {
        return Err(ApiError::NotFound);
    }
    db.execute(RefreshEventValidationQueryView::new(event_id))
        .await
        .map_err(database_error)?;

    Ok(HttpResponse::NoContent().finish())
}
