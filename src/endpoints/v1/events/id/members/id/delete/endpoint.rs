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
    summary = "Retirer un participant d'un événement",
    description = "Désassigne un utilisateur de l'événement et recalcule son statut de validation \
                   global, le départ d'un participant pouvant suffire à le valider.\n\n\
                   Réservé à qui peut gérer les participants : le créateur, ou une personne \
                   habilitée à modifier l'événement.\n\n\
                   Opération non idempotente : retirer quelqu'un qui n'est pas assigné répond \
                   `404`, au même titre qu'un événement inexistant.",
    params(
        ("event_id" = u64, Path, description = "Identifiant de l'événement.", example = 21),
        ("member_id" = u64, Path, description = "Identifiant Core API du participant à retirer.", example = 51)
    ),
    responses(
        (
            status = 204,
            description = "Participant retiré et validation de l'événement recalculée. Corps vide.",
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
            description = "L'appelant ne peut pas gérer les participants de cet événement.",
            body = String,
            content_type = "text/plain",
            example = json!("Forbidden.")
        ),
        (
            status = 404,
            description = "Aucun événement ne porte cet identifiant, ou l'utilisateur n'y est pas assigné.",
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
