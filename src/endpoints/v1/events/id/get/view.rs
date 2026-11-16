use chrono::{DateTime, Utc};
use utoipa::ToSchema;

use crate::database::event::access::view::ApprovalStatus;
use crate::database::event::get_event_members::view::EventValidationStatus;
use crate::database::event::model::{EventCategory, EventRecurrence, EventVisibility};

/// Member of an event.
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize, ToSchema)]
pub struct Member {
    /// Core API id of the member.
    #[schema(example = 42)]
    pub id: u64,
    /// Approval status of the event, repeated on every member (`approval_status` of the event,
    /// in the `validated` / `refused` / `pending` vocabulary).
    pub validation_status: EventValidationStatus,
}

/// Rights of the caller on the event.
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Deserialize, serde::Serialize, ToSchema)]
pub struct EventPermissionsView {
    /// The caller can edit the event: they are its assigned creator, or have the Responsable,
    /// Maire or Admin role.
    #[schema(example = true)]
    pub can_edit: bool,
    /// The caller can delete the event: they created it. An assigned Admin can edit it
    /// without being able to delete it.
    #[schema(example = true)]
    pub can_delete: bool,
    /// The caller can approve or reject the event: assigned Responsable sharing a group with the
    /// creator, on an event still pending.
    #[schema(example = false)]
    pub can_validate: bool,
}

/// Detail of an event.
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize, ToSchema)]
pub struct GetEventResultView {
    /// Id of the event.
    #[schema(example = 21)]
    pub id: u64,
    /// Title of the event.
    #[schema(example = "Town council")]
    pub name: String,
    /// Free description, or `null`.
    #[schema(example = "Agenda sent one week before")]
    pub description: Option<String>,
    /// Start of the event.
    #[schema(value_type = String, format = DateTime, example = "2026-10-05T18:00:00Z")]
    pub events_start_time: DateTime<Utc>,
    /// End of the event.
    #[schema(value_type = String, format = DateTime, example = "2026-10-05T20:00:00Z")]
    pub events_end_time: DateTime<Utc>,
    /// Visibility of the event.
    pub visibility: EventVisibility,
    /// Category of the event.
    pub category: EventCategory,
    /// Organising department, or `null`.
    #[schema(example = "General secretariat")]
    pub service: Option<String>,
    /// Location, or `null`.
    #[schema(example = "Council chamber")]
    pub location: Option<String>,
    /// Repetition rule, or `null` for a one-off event.
    pub recurrence: Option<EventRecurrence>,
    /// Core API id of the owner, or `null`.
    #[schema(example = 42)]
    pub owner: Option<u64>,
    /// Core API id of the creator: the only one who can delete the event.
    #[schema(example = 42)]
    pub created_by: Option<u64>,
    /// Assigned members, sorted by id.
    pub members: Vec<Member>,
    /// Approval of the event: `pending` while a Responsable has not decided (events created by a
    /// User or Guest, or moved by their creator since), `approved` or `rejected` otherwise.
    pub approval_status: ApprovalStatus,
    /// Rights of the caller on this event, to be used as is by clients.
    pub permissions: EventPermissionsView,
}
