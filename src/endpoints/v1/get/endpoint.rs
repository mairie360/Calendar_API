use actix_web::{get, web, HttpResponse, Responder};
use mairie360_api_lib::security::AuthenticatedUser;
use mairie360_api_lib::state::AppState;

use crate::database::calendar::get::view::{Event, GetCalendarQueryView};
use crate::endpoints::error::{database_error, ApiError};
use crate::endpoints::v1::get::view::{GetCalendarParams, GetCalendarResultView};

#[utoipa::path(
    get,
    path = "calendar",
    summary = "Consulter son agenda sur une période",
    description = "Renvoie les événements de l'utilisateur porté par le JWT qui tombent dans la \
                   période demandée : ceux dont il est propriétaire et ceux auxquels il est \
                   assigné.\n\n\
                   Les événements récurrents dont la règle chevauche la période sont inclus, même \
                   si leur date de départ est antérieure : c'est ce qui permet d'afficher une \
                   semaine ou un mois sans rejouer les récurrences côté client.\n\n\
                   Les deux bornes sont incluses, et `end` doit être postérieure ou égale à \
                   `start`, sans quoi la réponse est `400`. Il n'y a pas de limite de largeur de \
                   période ni de pagination.\n\n\
                   Vue de liste : ni les participants ni le statut de validation ne sont inclus, \
                   il faut passer par `GET /api/v1/events/{event_id}/`.",
    params(GetCalendarParams),
    responses(
        (
            status = 200,
            description = "Événements de l'appelant sur la période, récurrences chevauchantes comprises.",
            body = GetCalendarResultView,
            example = json!({
                "events": [
                    {
                        "id": 21,
                        "name": "Conseil municipal",
                        "start": "2026-10-05T18:00:00Z",
                        "end": "2026-10-05T20:00:00Z",
                        "is_member": true,
                        "category": "Meeting",
                        "service": "Secrétariat général",
                        "location": "Salle du conseil",
                        "recurrence": { "frequency": "Monthly", "interval": 1, "days_of_week": null, "ends_on": "2027-06-30" }
                    }
                ]
            })
        ),
        (
            status = 400,
            description = "`start` ou `end` absent ou mal formé, ou `end` antérieure à `start`.",
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
    tag = "Calendar",
    security(
        ("jwt" = [])
    )
)]
#[get("/calendar")]
pub async fn get_calendar(
    state: web::Data<AppState>,
    auth_user: AuthenticatedUser,
    params: web::Query<GetCalendarParams>,
) -> Result<impl Responder, ApiError> {
    if params.end < params.start {
        return Err(ApiError::BadRequest);
    }
    let events: Vec<Event> = state
        .get_smart_db()
        .fetch_all(&GetCalendarQueryView::new(
            params.start,
            params.end,
            auth_user.id,
        ))
        .await
        .map_err(database_error)?;

    Ok(HttpResponse::Ok().json(GetCalendarResultView {
        events: events.into_iter().map(Into::into).collect(),
    }))
}
