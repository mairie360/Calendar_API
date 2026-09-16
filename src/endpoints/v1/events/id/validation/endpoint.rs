use actix_web::{patch, web, HttpResponse, Responder};
use mairie360_api_lib::security::AuthenticatedUser;
use mairie360_api_lib::state::AppState;

use crate::database::event::access::view::{ApprovalStatus, EventAccess};
use crate::database::event::get_event_members::view::EventValidationStatus;
use crate::database::event::validation::view::SetEventValidationQueryView;
use crate::endpoints::error::{database_error, require_event_access, ApiError};
use crate::endpoints::v1::events::id::validation::view::UpdateEventValidationView;

#[utoipa::path(
    patch,
    path = "validation",
    params(
        ("event_id" = u64, Path, description = "Event ID")
    ),
    request_body = UpdateEventValidationView,
    responses(
        (status = 204, description = "Validation status applied to every member"),
        (status = 403, description = "Only an assigned Responsable sharing a group with the creator can validate a pending event"),
        (status = 404, description = "Unknown event"),
        (status = 500, description = "Internal server error")
    ),
    tag = "Events",
    security(
        ("jwt" = [])
    )
)]
#[patch("/validation")]
pub async fn update_event_validation(
    state: web::Data<AppState>,
    auth_user: AuthenticatedUser,
    event_id: web::Path<u64>,
    view: web::Json<UpdateEventValidationView>,
) -> Result<impl Responder, ApiError> {
    let event_id = event_id.into_inner();
    require_event_access(&state, event_id, auth_user.id, EventAccess::can_validate).await?;

    let status = match view.status {
        ApprovalStatus::Approved => EventValidationStatus::Validated,
        ApprovalStatus::Rejected => EventValidationStatus::Refused,
        ApprovalStatus::Pending => EventValidationStatus::Pending,
    };
    state
        .get_smart_db()
        .execute(SetEventValidationQueryView::new(event_id, status))
        .await
        .map_err(database_error)?;

    Ok(HttpResponse::NoContent().finish())
}
