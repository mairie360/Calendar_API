use actix_web::{patch, web, HttpResponse, Responder};
use mairie360_api_lib::database::error::DbError;
use mairie360_api_lib::error::ApiLibError;
use mairie360_api_lib::security::AuthenticatedUser;
use mairie360_api_lib::state::AppState;

use crate::database::event::access::view::EventAccess;
use crate::database::event::validation::view::SetEventApprovalQueryView;
use crate::endpoints::error::{database_error, require_event_access_in, ApiError};
use crate::endpoints::json::JsonBody;
use crate::endpoints::v1::events::id::validation::view::UpdateEventValidationView;

#[utoipa::path(
    patch,
    path = "validation",
    summary = "Approve or reject an event",
    description = "Records the approval decision on the event itself, with who took it and \
                   when: `approved` approves it, `rejected` rejects it, `pending` leaves it \
                   waiting. The decision is not tied to the members: assigning or removing members \
                   afterwards does not change it. Only a later edit of the dates, recurrence or \
                   location by the creator sends the event back to `pending`.\n\n\
                   An event needs approval when its creator only has the User or Guest roles; it \
                   is created `pending`. All these conditions are required: the Responsable role, \
                   being assigned to the event, sharing a group with its creator, and the event \
                   still pending. Otherwise the response is `403`, including on an event already \
                   approved or rejected.\n\n\
                   The response has an empty body; read the event again to see its new status.",
    params(
        ("event_id" = u64, Path, description = "Event id.", example = 21)
    ),
    request_body(
        content = UpdateEventValidationView,
        description = "Decision to record on the event.",
        example = json!({ "status": "approved" })
    ),
    responses(
        (
            status = 204,
            description = "Decision recorded. Empty body.",
        ),
        (
            status = 400,
            description = "Malformed JSON body, unknown field, `event_id` not an integer, or unknown status. A body error names the faulty field (`body` for the whole object).",
            body = String,
            content_type = "text/plain",
            example = json!("Invalid `status`: unknown variant `Maybe`, expected one of `pending`, `approved`, `rejected`.")
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
            description = "The caller is not an assigned Responsable sharing a group with the creator, or the event does not need approval or is no longer pending.",
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
            status = 409,
            description = "Another Responsable decided between the access check and the update: the event is no longer pending.",
            body = String,
            content_type = "text/plain",
            example = json!("Conflict.")
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
#[patch("/validation")]
pub async fn update_event_validation(
    state: web::Data<AppState>,
    auth_user: AuthenticatedUser,
    event_id: web::Path<u64>,
    view: JsonBody<UpdateEventValidationView>,
) -> Result<impl Responder, ApiError> {
    let event_id = event_id.into_inner();
    // The Responsable's rights (membership, group, pending status) are checked on the locked row,
    // in the transaction of the decision (MAIR-420).
    let mut tx = state.get_smart_db().begin().await.map_err(database_error)?;
    require_event_access_in(&mut tx, event_id, auth_user.id, EventAccess::can_validate).await?;

    // The update only applies to a still pending event: no row means a concurrent decision.
    match tx
        .fetch_scalar::<i32, _>(&SetEventApprovalQueryView::new(
            event_id,
            auth_user.id,
            view.status,
        ))
        .await
    {
        Ok(_) => {}
        Err(ApiLibError::Database(DbError::NotFound)) => return Err(ApiError::Conflict),
        Err(error) => return Err(database_error(error)),
    }
    tx.commit().await.map_err(database_error)?;
    Ok(HttpResponse::NoContent().finish())
}
