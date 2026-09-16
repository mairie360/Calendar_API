use actix_web::{patch, web, HttpResponse, Responder};
use mairie360_api_lib::security::AuthenticatedUser;
use mairie360_api_lib::state::AppState;

use crate::database::event::access::view::EventAccess;
use crate::database::event::edit::view::{DeleteOrphanRecurrenceQueryView, EditEventQueryView};
use crate::database::event::get::view::{GetEventQueryResultView, GetEventQueryView};
use crate::endpoints::error::{database_error, require_event_access, ApiError};
use crate::endpoints::v1::events::id::patch::view::PatchEventView;
use crate::endpoints::v1::events::validate_event_input;

#[utoipa::path(
    patch,
    path = "",
    params(
        ("event_id" = u64, Path, description = "Event ID")
    ),
    request_body = PatchEventView,
    responses(
        (status = 204, description = "Event updated"),
        (status = 400, description = "Bad request"),
        (status = 403, description = "Only an assigned creator, Responsable, Maire or Admin can edit the event"),
        (status = 404, description = "Unknown event"),
        (status = 500, description = "Internal server error")
    ),
    tag = "Events",
    security(
        ("jwt" = [])
    )
)]
#[patch("/")]
pub async fn patch_event(
    state: web::Data<AppState>,
    auth_user: AuthenticatedUser,
    event_id: web::Path<u64>,
    view: web::Json<PatchEventView>,
) -> Result<impl Responder, ApiError> {
    let event_id = event_id.into_inner();
    require_event_access(&state, event_id, auth_user.id, EventAccess::can_edit).await?;

    let db = state.get_smart_db();
    let current: Vec<GetEventQueryResultView> = db
        .fetch_all(&GetEventQueryView::new(event_id))
        .await
        .map_err(database_error)?;
    let current = current.into_iter().next().ok_or(ApiError::NotFound)?;
    let input = view.into_inner().apply_to(current.to_input());
    validate_event_input(&input)?;

    let updated: bool = db
        .fetch_scalar(&EditEventQueryView::new(event_id, &input))
        .await
        .map_err(|_| ApiError::BadRequest)?;
    if !updated {
        return Err(ApiError::NotFound);
    }
    if let (Some(rule_id), None) = (current.recurrence_id(), &input.recurrence) {
        db.execute(DeleteOrphanRecurrenceQueryView::new(rule_id as u64))
            .await
            .map_err(database_error)?;
    }

    Ok(HttpResponse::NoContent().finish())
}
