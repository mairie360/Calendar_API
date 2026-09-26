use chrono::{DateTime, Utc};
use utoipa::ToSchema;

use crate::database::event::model::{EventCategory, EventInput, EventRecurrence, EventVisibility};

/// Événement à créer ; l'appelant en devient propriétaire, sans être ajouté aux participants.
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, ToSchema)]
pub struct PostEventView {
    /// Event title. Not blank once trimmed, at most 150 characters, no control character, no `<`
    /// or `>`.
    #[schema(min_length = 1, max_length = 150, example = "Conseil municipal")]
    pub name: String,
    /// Description libre, ou absente.
    #[schema(max_length = 5000, example = "Ordre du jour envoyé une semaine avant")]
    pub description: Option<String>,
    /// Début de l'événement.
    #[schema(value_type = String, format = DateTime, example = "2026-10-05T18:00:00Z")]
    pub events_start_time: DateTime<Utc>,
    /// Fin de l'événement. Doit être **strictement** postérieure au début.
    #[schema(value_type = String, format = DateTime, example = "2026-10-05T20:00:00Z")]
    pub events_end_time: DateTime<Utc>,
    /// `Public` par défaut.
    pub visibility: Option<EventVisibility>,
    /// `other` par défaut.
    pub category: Option<EventCategory>,
    /// Organising service, at most 128 characters, no control character, no `<` or `>`.
    #[schema(max_length = 128, example = "Secrétariat général")]
    pub service: Option<String>,
    /// Lieu de l'événement.
    #[schema(max_length = 255, example = "Salle du conseil")]
    pub location: Option<String>,
    /// Règle de répétition. Absente pour un événement ponctuel. Doit être cohérente avec la date
    /// de début, sans quoi la création échoue en `400`.
    pub recurrence: Option<EventRecurrence>,
}

impl From<PostEventView> for EventInput {
    fn from(view: PostEventView) -> Self {
        EventInput {
            name: view.name.trim().to_string(),
            description: view.description,
            start: view.events_start_time,
            end: view.events_end_time,
            visibility: view.visibility.unwrap_or_default(),
            category: view.category.unwrap_or_default(),
            service: view.service,
            location: view.location,
            recurrence: view.recurrence,
        }
    }
}

/// Événement créé.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize, ToSchema)]
pub struct PostEventResultView {
    /// Identifiant attribué à l'événement créé. L'appelant en est le créateur, mais pas encore
    /// un participant : s'assigner via `POST /api/v1/events/{event_id}/members/`.
    #[schema(example = 21)]
    pub event_id: u64,
}
