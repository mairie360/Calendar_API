use actix_web::{post, web, HttpResponse, Responder};
use mairie360_api_lib::security::AuthenticatedUser;
use mairie360_api_lib::state::AppState;

use crate::database::event::create::view::CreateEventQueryView;
use crate::database::event::model::EventInput;
use crate::endpoints::error::{database_error, ApiError};
use crate::endpoints::v1::events::post::view::{PostEventResultView, PostEventView};
use crate::endpoints::v1::events::validate_event_input;

#[utoipa::path(
    post,
    path = "",
    summary = "Créer un événement",
    description = "Crée un événement dont l'appelant devient créateur et propriétaire. Aucun rôle \
                   particulier n'est exigé.\n\n\
                   L'appelant n'est **pas** ajouté aux participants : tant qu'il ne s'est pas \
                   assigné via `POST /api/v1/events/{event_id}/members/`, l'événement n'apparaît \
                   pas dans son `GET /api/v1/calendar` et il ne peut pas lire son détail. Il peut \
                   en revanche déjà gérer ses participants, en tant que créateur.\n\n\
                   Contrôles appliqués : nom non vide et d'au plus 255 caractères, \
                   `events_end_time` strictement postérieure à `events_start_time`, `service` d'au \
                   plus 255 caractères, et règle de répétition cohérente avec la date de début. \
                   Tous partagent le même `400`.\n\n\
                   La réponse ne contient que l'identifiant attribué.",
    request_body(
        content = PostEventView,
        description = "Définition de l'événement. `visibility` vaut `Public` et `category` vaut `Other` si absents.",
        example = json!({
            "name": "Conseil municipal",
            "description": "Ordre du jour envoyé une semaine avant",
            "events_start_time": "2026-10-05T18:00:00Z",
            "events_end_time": "2026-10-05T20:00:00Z",
            "visibility": "Public",
            "category": "Meeting",
            "service": "Secrétariat général",
            "location": "Salle du conseil",
            "recurrence": { "frequency": "Monthly", "interval": 1, "days_of_week": null, "ends_on": "2027-06-30" }
        })
    ),
    responses(
        (
            status = 201,
            description = "Événement créé. L'appelant en est le créateur, mais pas encore un participant.",
            body = PostEventResultView,
            example = json!({ "event_id": 21 })
        ),
        (
            status = 400,
            description = "Corps JSON malformé, nom vide ou de plus de 255 caractères, fin antérieure ou égale au début, `service` trop long, ou règle de répétition incohérente avec la date de début.",
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
pub async fn create_event(
    state: web::Data<AppState>,
    auth_user: AuthenticatedUser,
    view: web::Json<PostEventView>,
) -> Result<impl Responder, ApiError> {
    let input = EventInput::from(view.into_inner());
    validate_event_input(&input)?;

    let event_id: i32 = state
        .get_smart_db()
        .fetch_scalar(&CreateEventQueryView::new(auth_user.id, &input))
        .await
        .map_err(database_error)?;

    Ok(HttpResponse::Created().json(PostEventResultView {
        event_id: event_id as u64,
    }))
}
