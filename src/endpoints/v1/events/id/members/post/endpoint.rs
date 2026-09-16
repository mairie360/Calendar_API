use actix_web::{post, web, HttpResponse, Responder};
use mairie360_api_lib::security::AuthenticatedUser;
use mairie360_api_lib::state::AppState;

use crate::database::event::access::view::EventAccess;
use crate::database::event::add_member::view::AddUserToEventQueryView;
use crate::database::event::validation::view::{
    CanAssignUserQueryView, RefreshEventValidationQueryView,
};
use crate::endpoints::error::{database_error, require_event_access, ApiError};
use crate::endpoints::v1::events::id::members::post::view::PostMemberView;

#[utoipa::path(
    post,
    path = "",
    summary = "Assigner un participant à un événement",
    description = "Assigne un utilisateur à l'événement et recalcule aussitôt son statut de \
                   validation global, l'arrivée d'un participant pouvant le remettre en attente.\n\n\
                   Deux contrôles indépendants se succèdent, tous deux rendus en `403` : \
                   l'appelant doit pouvoir gérer les participants (créateur, ou personne habilitée \
                   à modifier l'événement), **et** l'utilisateur visé doit être dans son périmètre \
                   d'assignation. Le message du corps ne distingue pas les deux cas.\n\n\
                   C'est par cet endpoint que le créateur s'ajoute lui-même après \
                   `POST /api/v1/events/`. La réponse a un corps vide.",
    params(
        ("event_id" = u64, Path, description = "Identifiant de l'événement.", example = 21)
    ),
    request_body(
        content = PostMemberView,
        description = "Identifiant Core API de l'utilisateur à assigner.",
        example = json!({ "user_id": 51 })
    ),
    responses(
        (
            status = 201,
            description = "Participant assigné et validation de l'événement recalculée. Corps vide.",
        ),
        (
            status = 400,
            description = "Corps JSON malformé, `event_id` non entier, ou champ `user_id` absent.",
            body = String,
            content_type = "text/plain",
            example = json!("Json deserialize error: missing field `user_id`")
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
            description = "L'appelant ne peut pas gérer les participants de cet événement, ou l'utilisateur visé est hors de son périmètre d'assignation.",
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
            status = 409,
            description = "L'utilisateur est déjà assigné à cet événement.",
            body = String,
            content_type = "text/plain",
            example = json!("Conflict.")
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
#[post("/")]
pub async fn add_event_member(
    state: web::Data<AppState>,
    auth_user: AuthenticatedUser,
    event_id: web::Path<u64>,
    view: web::Json<PostMemberView>,
) -> Result<impl Responder, ApiError> {
    let event_id = event_id.into_inner();
    require_event_access(
        &state,
        event_id,
        auth_user.id,
        EventAccess::can_manage_members,
    )
    .await?;

    let db = state.get_smart_db();
    let assignable: bool = db
        .fetch_scalar(&CanAssignUserQueryView::new(auth_user.id, view.user_id))
        .await
        .map_err(database_error)?;
    if !assignable {
        return Err(ApiError::Forbidden);
    }
    // Index unique (event_id, user_id) : un doublon échoue à l'insertion.
    db.execute(AddUserToEventQueryView::new(view.user_id, event_id))
        .await
        .map_err(|_| ApiError::Conflict)?;
    db.execute(RefreshEventValidationQueryView::new(event_id))
        .await
        .map_err(database_error)?;

    Ok(HttpResponse::Created().finish())
}
