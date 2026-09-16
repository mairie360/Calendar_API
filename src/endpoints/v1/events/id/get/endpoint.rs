use actix_web::{get, web, HttpResponse, Responder};
use mairie360_api_lib::security::AuthenticatedUser;
use mairie360_api_lib::state::AppState;

use crate::database::event::access::view::EventAccess;
use crate::database::event::get::view::{GetEventQueryResultView, GetEventQueryView};
use crate::database::event::get_event_members::view::{GetEventMemberQueryView, Member};
use crate::endpoints::error::{database_error, require_event_access, ApiError};
use crate::endpoints::v1::events::id::get::view::{
    EventPermissionsView, GetEventResultView, Member as MemberView,
};

pub async fn load_event(
    state: &web::Data<AppState>,
    event_id: u64,
    access: &EventAccess,
) -> Result<GetEventResultView, ApiError> {
    let db = state.get_smart_db();
    let event_view = GetEventQueryView::new(event_id);
    let members_view = GetEventMemberQueryView::new(event_id);
    let (events, members) = futures_util::try_join!(
        db.fetch_all::<GetEventQueryResultView, _>(&event_view),
        db.fetch_all::<Member, _>(&members_view),
    )
    .map_err(database_error)?;
    let event = events.into_iter().next().ok_or(ApiError::NotFound)?;
    let input = event.to_input();

    Ok(GetEventResultView {
        id: event_id,
        name: input.name,
        description: input.description,
        events_start_time: input.start,
        events_end_time: input.end,
        visibility: input.visibility,
        category: input.category,
        service: input.service,
        location: input.location,
        recurrence: input.recurrence,
        owner: event.owner_id().map(|id| id as u64),
        created_by: event.created_by().map(|id| id as u64),
        members: members
            .into_iter()
            .map(|member| MemberView {
                id: member.user_id() as u64,
                validation_status: member.validation_status(),
            })
            .collect(),
        approval_status: access.approval_status,
        permissions: EventPermissionsView {
            can_edit: access.can_edit(),
            can_delete: access.can_delete(),
            can_validate: access.can_validate(),
        },
    })
}

#[utoipa::path(
    get,
    path = "",
    summary = "Consulter le détail d'un événement",
    description = "Renvoie un événement complet : description, visibilité, catégorie, lieu, \
                   récurrence, participants avec leur statut de validation, statut d'approbation \
                   global, et **droits de l'appelant** sur l'événement.\n\n\
                   Le champ `permissions` évite de redériver les règles d'accès côté client : il \
                   suffit de masquer les boutons dont le drapeau correspondant est `false`.\n\n\
                   Réservé aux participants assignés : un événement existant auquel l'appelant \
                   n'est pas assigné répond `403`, pas `404`.",
    params(
        ("event_id" = u64, Path, description = "Identifiant de l'événement.", example = 21)
    ),
    responses(
        (
            status = 200,
            description = "Détail de l'événement et droits de l'appelant.",
            body = GetEventResultView,
            example = json!({
                "id": 21,
                "name": "Conseil municipal",
                "description": "Ordre du jour envoyé une semaine avant",
                "events_start_time": "2026-10-05T18:00:00Z",
                "events_end_time": "2026-10-05T20:00:00Z",
                "visibility": "Public",
                "category": "Meeting",
                "service": "Secrétariat général",
                "location": "Salle du conseil",
                "recurrence": null,
                "owner": 42,
                "created_by": 42,
                "members": [
                    { "id": 42, "validation_status": "Validated" },
                    { "id": 51, "validation_status": "Pending" }
                ],
                "approval_status": "Pending",
                "permissions": { "can_edit": true, "can_delete": true, "can_validate": false }
            })
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
            description = "L'appelant n'est pas assigné à cet événement. Seuls ses participants peuvent le voir.",
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
#[get("/")]
pub async fn get_event(
    state: web::Data<AppState>,
    auth_user: AuthenticatedUser,
    event_id: web::Path<u64>,
) -> Result<impl Responder, ApiError> {
    let event_id = event_id.into_inner();
    let access =
        require_event_access(&state, event_id, auth_user.id, EventAccess::can_view).await?;
    let event = load_event(&state, event_id, &access).await?;
    Ok(HttpResponse::Ok().json(event))
}
