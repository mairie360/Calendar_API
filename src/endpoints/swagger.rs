use crate::endpoints::health::HealthDoc;
use crate::endpoints::hello::HelloDoc;
use crate::endpoints::v1::doc::V1Doc;
use utoipa::openapi::security::{Http, HttpAuthScheme, SecurityScheme};
use utoipa::{Modify, OpenApi};

#[derive(OpenApi)]
#[openapi(
    info(
        title = "Calendar API — Mairie 360",
        version = "1.0.0",
        description = "\
API d'agenda de la plateforme **Mairie 360** : événements, participants, récurrences et circuit de \
validation. Les comptes, les rôles et les groupes vivent dans Core API.

## Deux façons de lire l'agenda

`GET /api/v1/calendar` renvoie la vue **calendrier** : les événements de l'appelant sur une \
période donnée, récurrences dépliées comprises. `GET /api/v1/events/{event_id}/` renvoie le \
**détail** d'un événement, avec ses participants, son statut de validation et les droits de \
l'appelant sur lui.

## Droits d'accès

Un événement n'est visible que de ses **participants** : un événement auquel l'appelant n'est pas \
assigné répond `403`, et non `404`, qui est réservé aux identifiants inexistants.

| Action | Qui peut la faire |
| --- | --- |
| Voir | tout participant assigné |
| Modifier | participant qui en est le créateur, ou qui a le rôle Responsable, Maire ou Admin |
| Supprimer | le créateur, et lui seul |
| Gérer les participants | le créateur, ou quiconque peut modifier l'événement |
| Valider | Responsable assigné, partageant un groupe avec le créateur, sur un événement en attente |

Le champ `permissions` de `GET /api/v1/events/{event_id}/` donne directement ces droits pour \
l'appelant, ce qui évite de les redériver côté client.

## Error format

Error responses (`4xx` and `5xx`) have a **`text/plain`** body holding the error message, not a \
JSON object. Every response carries `X-Content-Type-Options: nosniff`.

Statuses returned across the API, before the handler runs:

| Status | Meaning |
| --- | --- |
| `400` | URL segment that is not an integer, missing or malformed query parameter, or malformed JSON body. |
| `401` | `Authorization` header missing or malformed, invalid or expired JWT, or revoked session. |
| `500` | Database or Redis failure. |
",
        contact(
            name = "Équipe Mairie 360",
            url = "https://github.com/mairie360"
        ),
        license(
            name = "Propriétaire",
            identifier = "LicenseRef-mairie360-proprietary"
        )
    ),
    servers(
        (url = "http://localhost:3002", description = "Développement local (cargo run)"),
        (url = "http://development.mairie360.fr", description = "Pile Docker de développement (nginx)")
    ),
    tags(
        (name = "Calendar", description = "Vue calendrier : les événements de l'appelant sur une période, récurrences dépliées."),
        (name = "Events", description = "Événements : création, détail, modification, suppression, participants et validation."),
        (name = "Service", description = "Sondes techniques non authentifiées, utilisées par Docker et Kubernetes.")
    ),
    nest(
        (path = "/api/v1", api = V1Doc),
        (path = "/", api = HealthDoc),
        (path = "/", api = HelloDoc),
    ),
    modifiers(&SecurityAddon)
)]
pub struct ApiDoc;

/// Sans ce modifier, les opérations qui déclarent `security(("jwt" = []))` référencent un schéma
/// absent du contrat : Swagger UI n'offre pas de bouton « Authorize » et les clients générés
/// pointent dans le vide.
struct SecurityAddon;

impl Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        let components = openapi.components.as_mut().unwrap();
        components.add_security_scheme(
            "jwt",
            SecurityScheme::Http(
                Http::builder()
                    .scheme(HttpAuthScheme::Bearer)
                    .bearer_format("JWT")
                    .description(Some("JWT émis par Core API (`POST /api/v1/auth/login`)."))
                    .build(),
            ),
        )
    }
}
