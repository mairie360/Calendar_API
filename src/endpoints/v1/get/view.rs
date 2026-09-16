use chrono::{DateTime, Utc};
use utoipa::{IntoParams, ToSchema};

use crate::database::calendar::get::view::Event;
use crate::database::event::model::{EventCategory, EventRecurrence};

#[derive(Debug, Clone, serde::Deserialize, IntoParams)]
#[into_params(parameter_in = Query)]
pub struct GetCalendarParams {
    /// Début de la période (inclus). Obligatoire.
    #[param(value_type = String, format = DateTime, example = "2026-10-01T00:00:00Z")]
    pub start: DateTime<Utc>,
    /// Fin de la période (incluse). Obligatoire, et doit être postérieure ou égale à `start`.
    #[param(value_type = String, format = DateTime, example = "2026-10-31T23:59:59Z")]
    pub end: DateTime<Utc>,
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize, ToSchema)]
pub struct EventView {
    /// Identifiant de l'événement, à réutiliser dans `/api/v1/events/{event_id}/`.
    #[schema(example = 21)]
    pub id: u64,
    /// Intitulé de l'événement.
    #[schema(example = "Conseil municipal")]
    pub name: String,
    /// Début de l'événement.
    #[schema(value_type = String, format = DateTime, example = "2026-10-05T18:00:00Z")]
    pub start: DateTime<Utc>,
    /// Fin de l'événement, toujours strictement postérieure au début.
    #[schema(value_type = String, format = DateTime, example = "2026-10-05T20:00:00Z")]
    pub end: DateTime<Utc>,
    /// Vrai si l'appelant est assigné à l'événement (sinon il n'en est que propriétaire).
    /// À `false`, le détail via `GET /api/v1/events/{event_id}/` répondra `403`.
    #[schema(example = true)]
    pub is_member: bool,
    /// Catégorie de l'événement.
    pub category: EventCategory,
    /// Service organisateur, ou `null`.
    #[schema(example = "Secrétariat général")]
    pub service: Option<String>,
    /// Lieu, ou `null`.
    #[schema(example = "Salle du conseil")]
    pub location: Option<String>,
    /// Règle de répétition, ou `null` pour un événement ponctuel. Un événement récurrent
    /// apparaît dès que sa règle chevauche la période demandée.
    pub recurrence: Option<EventRecurrence>,
}

impl From<Event> for EventView {
    fn from(event: Event) -> Self {
        Self {
            id: event.id as u64,
            name: event.name,
            start: event.start_date,
            end: event.end_date,
            is_member: event.is_member,
            category: event.category,
            service: event.service,
            location: event.location,
            recurrence: event.recurrence,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize, ToSchema)]
pub struct GetCalendarResultView {
    /// Événements de l'appelant sur la période. Sans pagination ni limite de nombre.
    pub events: Vec<EventView>,
}
