use chrono::{DateTime, Utc};
use serde::{Deserialize, Deserializer};
use utoipa::ToSchema;

use crate::database::event::model::{EventCategory, EventInput, EventRecurrence, EventVisibility};

/// Distingue un champ absent (`None`) d'un champ explicitement `null` (`Some(None)`).
fn deserialize_present<'de, D, T>(deserializer: D) -> Result<Option<Option<T>>, D::Error>
where
    D: Deserializer<'de>,
    T: Deserialize<'de>,
{
    Option::<T>::deserialize(deserializer).map(Some)
}

/// Modification partielle : un champ absent est conservé ; `null` efface une valeur facultative.
#[derive(Debug, Clone, Default, PartialEq, Eq, Deserialize, ToSchema)]
pub struct PatchEventView {
    /// New title. Absent to leave it unchanged. Not blank, at most 150 characters, no control
    /// character, no `<` or `>`.
    #[schema(min_length = 1, max_length = 150, example = "Conseil municipal")]
    pub name: Option<String>,
    /// Nouvelle description. Absente pour ne pas y toucher ; `null` l'efface.
    #[serde(default, deserialize_with = "deserialize_present")]
    #[schema(value_type = Option<String>, nullable, max_length = 5000, example = "Ordre du jour envoyé une semaine avant")]
    pub description: Option<Option<String>>,
    /// Nouveau début. Absent pour ne pas y toucher.
    #[schema(value_type = Option<String>, format = DateTime, example = "2026-10-05T18:00:00Z")]
    pub event_start_time: Option<DateTime<Utc>>,
    /// Nouvelle fin. Doit rester strictement postérieure au début après modification.
    #[schema(value_type = Option<String>, format = DateTime, example = "2026-10-05T20:00:00Z")]
    pub event_end_time: Option<DateTime<Utc>>,
    /// Nouvelle visibilité. Absente pour ne pas y toucher.
    pub visibility: Option<EventVisibility>,
    /// Nouvelle catégorie. Absente pour ne pas y toucher.
    pub category: Option<EventCategory>,
    /// New organising service, at most 128 characters, no control character, no `<` or `>`.
    /// Absent to leave it unchanged; `null` clears it.
    #[serde(default, deserialize_with = "deserialize_present")]
    #[schema(value_type = Option<String>, nullable, max_length = 128, example = "Secrétariat général")]
    pub service: Option<Option<String>>,
    /// Nouveau lieu. Absent pour ne pas y toucher ; `null` l'efface.
    #[serde(default, deserialize_with = "deserialize_present")]
    #[schema(value_type = Option<String>, nullable, max_length = 255, example = "Salle du conseil")]
    pub location: Option<Option<String>>,
    /// Nouvelle règle de répétition. Absente pour ne pas y toucher ; `null` retire la
    /// répétition. Doit rester cohérente avec la date de début, sinon `400`.
    #[serde(default, deserialize_with = "deserialize_present")]
    #[schema(value_type = Option<EventRecurrence>, nullable)]
    pub recurrence: Option<Option<EventRecurrence>>,
}

impl PatchEventView {
    /// Applique la modification à l'état courant de l'événement.
    pub fn apply_to(self, mut input: EventInput) -> EventInput {
        if let Some(name) = self.name {
            input.name = name.trim().to_string();
        }
        if let Some(description) = self.description {
            input.description = description;
        }
        if let Some(start) = self.event_start_time {
            input.start = start;
        }
        if let Some(end) = self.event_end_time {
            input.end = end;
        }
        if let Some(visibility) = self.visibility {
            input.visibility = visibility;
        }
        if let Some(category) = self.category {
            input.category = category;
        }
        if let Some(service) = self.service {
            input.service = service;
        }
        if let Some(location) = self.location {
            input.location = location;
        }
        if let Some(recurrence) = self.recurrence {
            input.recurrence = recurrence;
        }
        input
    }
}
