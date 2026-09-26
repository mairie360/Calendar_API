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
    summary = "Modifier un événement",
    description = "Met à jour partiellement un événement : un champ absent reste inchangé.\n\n\
                   Les champs facultatifs distinguent l'absence du `null` : omettre `description`, \
                   `service`, `location` ou `recurrence` conserve la valeur actuelle, les envoyer à \
                   `null` l'efface. Retirer la récurrence de cette façon supprime aussi la règle \
                   devenue orpheline.\n\n\
                   Réservé au participant qui est créateur de l'événement, ou qui a le rôle \
                   Responsable, Maire ou Admin. Les mêmes contrôles qu'à la création s'appliquent, \
                   sur l'événement tel qu'il sera après modification.\n\n\
                   La réponse a un corps vide.",
    params(
        ("event_id" = u64, Path, description = "Identifiant de l'événement.", example = 21)
    ),
    request_body(
        content = PatchEventView,
        description = "Champs à modifier. Tous facultatifs ; `null` efface une valeur facultative.",
        example = json!({ "location": "Salle des mariages", "recurrence": null })
    ),
    responses(
        (
            status = 204,
            description = "Événement mis à jour. Corps vide.",
        ),
        (
            status = 400,
            description = "Malformed JSON body, `event_id` not an integer, or event invalid once updated: end not after start, inconsistent recurrence, or a text field breaking its rules: `name` 1 to 150 characters once trimmed, `service` at most 128 characters and `location` at most 255, all three without control characters nor `<` / `>`; `description` at most 5000 characters, no `<` / `>`, no control character other than line breaks and tabs.",
            body = String,
            content_type = "text/plain",
            example = json!("Bad request.")
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
            description = "L'appelant n'est pas un participant créateur de l'événement, ni Responsable, Maire ou Admin.",
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
