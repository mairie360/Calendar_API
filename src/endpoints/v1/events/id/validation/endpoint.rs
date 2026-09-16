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
    summary = "Valider ou refuser un événement",
    description = "Applique un statut de validation à **tous** les participants de l'événement en \
                   une fois : `approved` le valide, `rejected` le refuse, `pending` le remet en \
                   attente. Il n'existe pas de validation participant par participant.\n\n\
                   C'est le droit le plus étroit de cette API. Il faut réunir toutes ces \
                   conditions : avoir le rôle Responsable, être assigné à l'événement, partager un \
                   groupe avec son créateur, et l'événement doit être soumis à validation et \
                   encore en attente. Sinon, la réponse est `403` — y compris sur un événement \
                   déjà validé ou déjà refusé, qu'on ne peut donc pas remettre en attente par ce \
                   biais.\n\n\
                   La réponse a un corps vide ; relire l'événement pour voir son nouveau statut.",
    params(
        ("event_id" = u64, Path, description = "Identifiant de l'événement.", example = 21)
    ),
    request_body(
        content = UpdateEventValidationView,
        description = "Statut à appliquer à tous les participants.",
        example = json!({ "status": "Approved" })
    ),
    responses(
        (
            status = 204,
            description = "Statut appliqué à tous les participants. Corps vide.",
        ),
        (
            status = 400,
            description = "Corps JSON malformé, `event_id` non entier, ou statut inconnu.",
            body = String,
            content_type = "text/plain",
            example = json!("Json deserialize error: unknown variant `Maybe`")
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
            description = "L'appelant n'est pas un Responsable assigné partageant un groupe avec le créateur, ou l'événement n'est pas (ou plus) en attente de validation.",
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
