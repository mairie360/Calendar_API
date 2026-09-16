use chrono::{DateTime, Utc};
use utoipa::ToSchema;

use crate::database::event::model::{EventCategory, EventInput, EventRecurrence, EventVisibility};

#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, ToSchema)]
pub struct PostEventView {
    pub name: String,
    pub description: Option<String>,
    #[schema(value_type = String, format = DateTime)]
    pub events_start_time: DateTime<Utc>,
    #[schema(value_type = String, format = DateTime)]
    pub events_end_time: DateTime<Utc>,
    /// `Public` par défaut.
    pub visibility: Option<EventVisibility>,
    /// `other` par défaut.
    pub category: Option<EventCategory>,
    pub service: Option<String>,
    pub location: Option<String>,
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

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize, ToSchema)]
pub struct PostEventResultView {
    pub event_id: u64,
}
