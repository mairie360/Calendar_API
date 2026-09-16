use actix_web::{delete, web, HttpResponse, Responder};
use mairie360_api_lib::security::AuthenticatedUser;
use mairie360_api_lib::state::AppState;

use crate::database::event::access::view::EventAccess;
use crate::database::event::delete::view::DeleteEventQueryView;
use crate::database::event::edit::view::DeleteOrphanRecurrenceQueryView;
use crate::database::event::get::view::{GetEventQueryResultView, GetEventQueryView};
use crate::endpoints::error::{database_error, require_event_access, ApiError};

#[utoipa::path(
    delete,
    path = "",
    params(
        ("event_id" = u64, Path, description = "Event ID")
    ),
    responses(
        (status = 204, description = "Event deleted"),
        (status = 403, description = "Only the creator can delete the event"),
        (status = 404, description = "Unknown event"),
        (status = 500, description = "Internal server error")
    ),
    tag = "Events",
    security(
        ("jwt" = [])
    )
)]
#[delete("/")]
pub async fn delete_event(
    state: web::Data<AppState>,
    auth_user: AuthenticatedUser,
    event_id: web::Path<u64>,
) -> Result<impl Responder, ApiError> {
    let event_id = event_id.into_inner();
    require_event_access(&state, event_id, auth_user.id, EventAccess::can_delete).await?;

    let db = state.get_smart_db();
    let events: Vec<GetEventQueryResultView> = db
        .fetch_all(&GetEventQueryView::new(event_id))
        .await
        .map_err(database_error)?;
    db.execute(DeleteEventQueryView::new(event_id))
        .await
        .map_err(database_error)?;
    if let Some(rule_id) = events
        .first()
        .and_then(GetEventQueryResultView::recurrence_id)
    {
        db.execute(DeleteOrphanRecurrenceQueryView::new(rule_id as u64))
            .await
            .map_err(database_error)?;
    }

    Ok(HttpResponse::NoContent().finish())
}
