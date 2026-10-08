use chrono::{DateTime, Utc};
use serde::{Deserialize, Deserializer};
use utoipa::ToSchema;

use crate::database::event::model::{EventCategory, EventInput, EventRecurrence, EventVisibility};

/// Tells an absent field (`None`) from an explicit `null` (`Some(None)`).
fn deserialize_present<'de, D, T>(deserializer: D) -> Result<Option<Option<T>>, D::Error>
where
    D: Deserializer<'de>,
    T: Deserialize<'de>,
{
    Option::<T>::deserialize(deserializer).map(Some)
}

/// Partial update: an absent field is kept, `null` clears an optional value. Field names are the
/// ones of `POST /api/v1/events/` and `GET /api/v1/events/{event_id}/`; an unknown field is a `400`.
#[derive(Debug, Clone, Default, PartialEq, Eq, Deserialize, ToSchema)]
#[serde(deny_unknown_fields)]
pub struct PatchEventView {
    /// New title. Absent to leave it unchanged. Not blank, at most 150 characters, no control
    /// character.
    #[schema(min_length = 1, max_length = 150, example = "Conseil municipal")]
    pub name: Option<String>,
    /// New description, at most 5000 characters. Absent to leave it unchanged;
    /// `null` clears it.
    #[serde(default, deserialize_with = "deserialize_present")]
    #[schema(value_type = Option<String>, nullable, max_length = 5000, example = "Ordre du jour envoyé une semaine avant")]
    pub description: Option<Option<String>>,
    /// New start, between `1970-01-01T00:00:00Z` and `2999-12-31T23:59:59Z`. Absent to leave it
    /// unchanged. `event_start_time` is accepted as a legacy alias.
    #[serde(alias = "event_start_time")]
    #[schema(value_type = Option<String>, format = DateTime, example = "2026-10-05T18:00:00Z")]
    pub events_start_time: Option<DateTime<Utc>>,
    /// New end, at most `2999-12-31T23:59:59Z`. Must stay strictly after the start once updated. `event_end_time` is accepted as
    /// a legacy alias.
    #[serde(alias = "event_end_time")]
    #[schema(value_type = Option<String>, format = DateTime, example = "2026-10-05T20:00:00Z")]
    pub events_end_time: Option<DateTime<Utc>>,
    /// New visibility. Absent to leave it unchanged.
    pub visibility: Option<EventVisibility>,
    /// New category. Absent to leave it unchanged.
    pub category: Option<EventCategory>,
    /// New organising service, at most 128 characters, no control character.
    /// Absent to leave it unchanged; `null` clears it.
    #[serde(default, deserialize_with = "deserialize_present")]
    #[schema(value_type = Option<String>, nullable, max_length = 128, example = "Secrétariat général")]
    pub service: Option<Option<String>>,
    /// New location, at most 255 characters, no control character. Absent to
    /// leave it unchanged; `null` clears it.
    #[serde(default, deserialize_with = "deserialize_present")]
    #[schema(value_type = Option<String>, nullable, max_length = 255, example = "Salle du conseil")]
    pub location: Option<Option<String>>,
    /// New recurrence rule. Absent to leave it unchanged; `null` removes the recurrence. Must
    /// stay consistent with the start date, otherwise `400`.
    #[serde(default, deserialize_with = "deserialize_present")]
    #[schema(value_type = Option<EventRecurrence>, nullable)]
    pub recurrence: Option<Option<EventRecurrence>>,
}

impl PatchEventView {
    /// Applies the update to the current state of the event.
    pub fn apply_to(self, mut input: EventInput) -> EventInput {
        if let Some(name) = self.name {
            input.name = name.trim().to_string();
        }
        if let Some(description) = self.description {
            input.description = description;
        }
        if let Some(start) = self.events_start_time {
            input.start = start;
        }
        if let Some(end) = self.events_end_time {
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
