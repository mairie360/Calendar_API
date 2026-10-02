use chrono::{DateTime, Utc};
use utoipa::{IntoParams, ToSchema};

use crate::database::calendar::get::view::Event;
use crate::database::event::model::{EventCategory, EventRecurrence, EventVisibility};

/// Widest period `GET /calendar` accepts, in days (a leap year).
pub const MAX_CALENDAR_RANGE_DAYS: i64 = 366;

#[derive(Debug, Clone, serde::Deserialize, IntoParams)]
#[into_params(parameter_in = Query)]
pub struct GetCalendarParams {
    /// Start of the period (inclusive). Required.
    #[param(value_type = String, format = DateTime, example = "2026-10-01T00:00:00Z")]
    pub start: DateTime<Utc>,
    /// End of the period (inclusive). Required, not before `start` and at most 366 days after it.
    #[param(value_type = String, format = DateTime, example = "2026-10-31T23:59:59Z")]
    pub end: DateTime<Utc>,
}

impl GetCalendarParams {
    /// `end` not before `start`, and the period at most `MAX_CALENDAR_RANGE_DAYS` wide.
    pub fn is_valid(&self) -> bool {
        self.end >= self.start
            && self.end - self.start <= chrono::Duration::days(MAX_CALENDAR_RANGE_DAYS)
    }
}

/// Événement du calendrier, sur la période demandée.
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
    /// True when the caller is assigned to the event. `false` for an event they only own, or a
    /// public event of someone else: both stay readable through `GET /api/v1/events/{event_id}/`.
    #[schema(example = true)]
    pub is_member: bool,
    /// `Public` events are listed in every user's calendar, `Private` ones only for their owner
    /// and members.
    pub visibility: EventVisibility,
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
        let visibility = event.visibility();
        Self {
            id: event.id as u64,
            name: event.name,
            start: event.start_date,
            end: event.end_date,
            is_member: event.is_member,
            visibility,
            category: event.category,
            service: event.service,
            location: event.location,
            recurrence: event.recurrence,
        }
    }
}

/// Événements du calendrier sur la période demandée.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize, ToSchema)]
pub struct GetCalendarResultView {
    /// Public events and the caller's events over the period, without pagination.
    pub events: Vec<EventView>,
}
