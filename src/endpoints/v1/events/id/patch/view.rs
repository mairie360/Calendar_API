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
    pub name: Option<String>,
    #[serde(default, deserialize_with = "deserialize_present")]
    #[schema(value_type = Option<String>, nullable)]
    pub description: Option<Option<String>>,
    #[schema(value_type = Option<String>, format = DateTime)]
    pub event_start_time: Option<DateTime<Utc>>,
    #[schema(value_type = Option<String>, format = DateTime)]
    pub event_end_time: Option<DateTime<Utc>>,
    pub visibility: Option<EventVisibility>,
    pub category: Option<EventCategory>,
    #[serde(default, deserialize_with = "deserialize_present")]
    #[schema(value_type = Option<String>, nullable)]
    pub service: Option<Option<String>>,
    #[serde(default, deserialize_with = "deserialize_present")]
    #[schema(value_type = Option<String>, nullable)]
    pub location: Option<Option<String>>,
    /// `null` retire la répétition.
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
