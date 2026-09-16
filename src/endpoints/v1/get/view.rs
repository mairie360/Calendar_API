use chrono::{DateTime, Utc};
use utoipa::{IntoParams, ToSchema};

use crate::database::calendar::get::view::Event;
use crate::database::event::model::{EventCategory, EventRecurrence};

#[derive(Debug, Clone, serde::Deserialize, IntoParams)]
#[into_params(parameter_in = Query)]
pub struct GetCalendarParams {
    /// Début de la période (inclus).
    #[param(value_type = String, format = DateTime)]
    pub start: DateTime<Utc>,
    /// Fin de la période (incluse).
    #[param(value_type = String, format = DateTime)]
    pub end: DateTime<Utc>,
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize, ToSchema)]
pub struct EventView {
    pub id: u64,
    pub name: String,
    #[schema(value_type = String, format = DateTime)]
    pub start: DateTime<Utc>,
    #[schema(value_type = String, format = DateTime)]
    pub end: DateTime<Utc>,
    /// Vrai si l'appelant est assigné à l'événement (sinon il n'en est que propriétaire).
    pub is_member: bool,
    pub category: EventCategory,
    pub service: Option<String>,
    pub location: Option<String>,
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
    pub events: Vec<EventView>,
}
