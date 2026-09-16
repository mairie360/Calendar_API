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
    summary = "Supprimer un événement",
    description = "Supprime définitivement un événement et les assignations de ses participants. \
                   Si l'événement portait une règle de répétition devenue orpheline, celle-ci est \
                   supprimée également.\n\n\
                   **Seul le créateur** peut supprimer un événement : c'est le droit le plus \
                   restrictif de cette API. Même un Admin ou un Maire assigné, qui peut le \
                   modifier, ne peut pas le supprimer.",
    params(
        ("event_id" = u64, Path, description = "Identifiant de l'événement.", example = 21)
    ),
    responses(
        (
            status = 204,
            description = "Événement supprimé. Corps vide.",
        ),
        (
            status = 400,
            description = "Un segment de l'URL n'est pas un entier, ou le corps JSON est malformé.",
            body = String,
            content_type = "text/plain",
            example = json!("Path deserialize error: can not parse `abc` to a u64")
        ),
        (
            status = 401,
            description = "En-tête `Authorization` absent, JWT invalide ou expiré, ou session révoquée.",
            body = String,
            content_type = "text/plain",
            example = json!("Jeton expiré")
        ),
        (
            status = 403,
            description = "L'appelant n'est pas le créateur de l'événement.",
            body = String,
            content_type = "text/plain",
            example = json!("Forbidden.")
        ),
        (
            status = 404,
            description = "Aucun événement ne porte cet identifiant.",
            body = String,
            content_type = "text/plain",
            example = json!("Unknown event.")
        ),
        (
            status = 500,
            description = "Erreur de base de données.",
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
