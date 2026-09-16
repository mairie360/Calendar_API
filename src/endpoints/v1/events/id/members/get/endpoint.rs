use actix_web::{get, web, HttpResponse, Responder};
use mairie360_api_lib::security::AuthenticatedUser;
use mairie360_api_lib::state::AppState;

use crate::database::event::access::view::EventAccess;
use crate::database::event::get_event_members::view::{GetEventMemberQueryView, Member};
use crate::endpoints::error::{database_error, require_event_access, ApiError};
use crate::endpoints::v1::events::id::get::view::Member as MemberView;
use crate::endpoints::v1::events::id::members::get::view::GetMembersResultView;

#[utoipa::path(
    get,
    path = "",
    summary = "Lister les participants d'un événement",
    description = "Renvoie les participants assignés à l'événement, chacun avec son statut de \
                   validation individuel. Réservé aux participants assignés.\n\n\
                   Seuls les identifiants Core API sont renvoyés : pour obtenir les noms, les \
                   repasser à `GET /api/v1/user/?ids=1,2,3` de Core API.\n\n\
                   `GET /api/v1/events/{event_id}/` renvoie déjà cette même liste avec le détail \
                   de l'événement : cet endpoint sert à la rafraîchir seule.",
    params(
        ("event_id" = u64, Path, description = "Identifiant de l'événement.", example = 21)
    ),
    responses(
        (
            status = 200,
            description = "Participants de l'événement et leur statut de validation.",
            body = GetMembersResultView,
            example = json!({
                "members": [
                    { "id": 42, "validation_status": "Validated" },
                    { "id": 51, "validation_status": "Pending" }
                ]
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
            description = "L'appelant n'est pas assigné à cet événement.",
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
pub async fn get_event_members(
    state: web::Data<AppState>,
    auth_user: AuthenticatedUser,
    event_id: web::Path<u64>,
) -> Result<impl Responder, ApiError> {
    let event_id = event_id.into_inner();
    require_event_access(&state, event_id, auth_user.id, EventAccess::can_view).await?;

    let members: Vec<Member> = state
        .get_smart_db()
        .fetch_all(&GetEventMemberQueryView::new(event_id))
        .await
        .map_err(database_error)?;

    Ok(HttpResponse::Ok().json(GetMembersResultView {
        members: members
            .into_iter()
            .map(|member| MemberView {
                id: member.user_id() as u64,
                validation_status: member.validation_status(),
            })
            .collect(),
    }))
}
