use actix_web::{delete, web, HttpResponse, Responder};
use mairie360_api_lib::security::AuthenticatedUser;
use mairie360_api_lib::state::AppState;

use crate::database::event::access::view::EventAccess;
use crate::database::event::delete::view::DeleteEventQueryView;
use crate::endpoints::error::{database_error, require_event_access_in, ApiError};

#[utoipa::path(
    delete,
    path = "",
    summary = "Delete an event",
    description = "Permanently deletes an event and its member assignments. When the event \
                   carried a recurrence rule no other event uses, the rule is deleted too, in the \
                   same statement.\n\n\
                   **Only the creator** can delete an event: the most restrictive right of this \
                   API. Even an assigned Admin or Maire, who can edit it, cannot delete it.",
    params(
        ("event_id" = u64, Path, description = "Event id.", example = 21)
    ),
    responses(
        (
            status = 204,
            description = "Event deleted. Empty body.",
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
            description = "The caller is not the creator of the event.",
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
#[delete("/")]
pub async fn delete_event(
    state: web::Data<AppState>,
    auth_user: AuthenticatedUser,
    event_id: web::Path<u64>,
) -> Result<impl Responder, ApiError> {
    let event_id = event_id.into_inner();
    let mut tx = state.get_smart_db().begin().await.map_err(database_error)?;
    require_event_access_in(&mut tx, event_id, auth_user.id, EventAccess::can_delete).await?;
    tx.execute(&DeleteEventQueryView::new(event_id))
        .await
        .map_err(database_error)?;
    tx.commit().await.map_err(database_error)?;

    Ok(HttpResponse::NoContent().finish())
}
