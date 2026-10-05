use actix_web::{patch, web, HttpResponse, Responder};
use mairie360_api_lib::security::AuthenticatedUser;
use mairie360_api_lib::state::AppState;

use crate::database::event::access::view::EventAccess;
use crate::database::event::edit::view::{DeleteOrphanRecurrenceQueryView, EditEventQueryView};
use crate::database::event::get::view::{GetEventQueryResultView, GetEventQueryView};
use crate::endpoints::error::{database_error, require_event_access_in, ApiError};
use crate::endpoints::v1::events::id::patch::view::PatchEventView;
use crate::endpoints::v1::events::validate_event_input;

#[utoipa::path(
    patch,
    path = "",
    summary = "Update an event",
    description = "Partially updates an event: an absent field is left unchanged.\n\n\
                   Optional fields tell absence from `null`: omitting `description`, `service`, \
                   `location` or `recurrence` keeps the current value, sending `null` clears it. \
                   Removing the recurrence this way also deletes the rule left orphan. The dates \
                   are `events_start_time` / `events_end_time`, as in `POST /api/v1/events/`; any \
                   unknown field is rejected with `400` instead of being ignored.\n\n\
                   Reserved to a member who is the creator of the event, or who has the \
                   Responsable, Maire or Admin role. The creation checks apply to the event as it \
                   will be once updated.\n\n\
                   Approval: when the creator needs a Responsable's approval (User or Guest only) \
                   and an editor without the Responsable, Maire or Admin role changes the dates, \
                   the recurrence or the location, the event goes back to `pending`, even if it \
                   was approved or rejected.\n\n\
                   The response has an empty body.",
    params(
        ("event_id" = u64, Path, description = "Event id.", example = 21)
    ),
    request_body(
        content = PatchEventView,
        description = "Fields to update. All optional; `null` clears an optional value.",
        example = json!({
            "events_start_time": "2026-10-12T18:00:00Z",
            "events_end_time": "2026-10-12T20:00:00Z",
            "location": "Salle des mariages",
            "recurrence": null
        })
    ),
    responses(
        (
            status = 204,
            description = "Event updated (and sent back to `pending` when the approval rule above applies). Empty body.",
        ),
        (
            status = 400,
            description = "Malformed JSON body, unknown field, `event_id` not an integer, or event invalid once updated: end not after start, inconsistent recurrence, or a text field breaking its rules: `name` 1 to 150 characters once trimmed, `service` at most 128 characters and `location` at most 255, all three without control characters nor `<` / `>`; `description` at most 5000 characters, no `<` / `>`, no control character other than line breaks and tabs.",
            body = String,
            content_type = "text/plain",
            example = json!("Bad request.")
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
            description = "The caller is not a member who is the creator of the event or has the Responsable, Maire or Admin role.",
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
#[patch("/")]
pub async fn patch_event(
    state: web::Data<AppState>,
    auth_user: AuthenticatedUser,
    event_id: web::Path<u64>,
    view: web::Json<PatchEventView>,
) -> Result<impl Responder, ApiError> {
    let event_id = event_id.into_inner();
    // One transaction: the access check, the update and the deletion of the detached recurrence
    // rule are applied together or not at all (MAIR-420).
    let mut tx = state.get_smart_db().begin().await.map_err(database_error)?;
    let access =
        require_event_access_in(&mut tx, event_id, auth_user.id, EventAccess::can_edit).await?;

    let current: Vec<GetEventQueryResultView> = tx
        .fetch_all(&GetEventQueryView::new(event_id))
        .await
        .map_err(database_error)?;
    let current = current.into_iter().next().ok_or(ApiError::NotFound)?;
    let current_input = current.to_input();
    let input = view.into_inner().apply_to(current_input.clone());
    validate_event_input(&input)?;

    let reset_approval = access.edit_needs_new_approval() && input.changes_schedule(&current_input);
    let updated: bool = tx
        .fetch_scalar(&EditEventQueryView::new(event_id, &input, reset_approval))
        .await
        .map_err(database_error)?;
    if !updated {
        return Err(ApiError::NotFound);
    }
    if let (Some(rule_id), None) = (current.recurrence_id(), &input.recurrence) {
        tx.execute(&DeleteOrphanRecurrenceQueryView::new(rule_id as u64))
            .await
            .map_err(database_error)?;
    }
    tx.commit().await.map_err(database_error)?;

    Ok(HttpResponse::NoContent().finish())
}
