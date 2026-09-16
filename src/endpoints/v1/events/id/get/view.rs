use chrono::{DateTime, Utc};
use utoipa::ToSchema;

use crate::database::event::access::view::ApprovalStatus;
use crate::database::event::get_event_members::view::EventValidationStatus;
use crate::database::event::model::{EventCategory, EventRecurrence, EventVisibility};

#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize, ToSchema)]
pub struct Member {
    pub id: u64,
    pub validation_status: EventValidationStatus,
}

/// Droits de l'appelant sur l'événement.
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Deserialize, serde::Serialize, ToSchema)]
pub struct EventPermissionsView {
    pub can_edit: bool,
    pub can_delete: bool,
    pub can_validate: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize, ToSchema)]
pub struct GetEventResultView {
    pub id: u64,
    pub name: String,
    pub description: Option<String>,
    #[schema(value_type = String, format = DateTime)]
    pub events_start_time: DateTime<Utc>,
    #[schema(value_type = String, format = DateTime)]
    pub events_end_time: DateTime<Utc>,
    pub visibility: EventVisibility,
    pub category: EventCategory,
    pub service: Option<String>,
    pub location: Option<String>,
    pub recurrence: Option<EventRecurrence>,
    pub owner: Option<u64>,
    pub created_by: Option<u64>,
    pub members: Vec<Member>,
    pub approval_status: ApprovalStatus,
    pub permissions: EventPermissionsView,
}
