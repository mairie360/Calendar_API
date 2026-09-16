use chrono::{DateTime, Utc};
use utoipa::ToSchema;

use crate::database::event::access::view::ApprovalStatus;
use crate::database::event::get_event_members::view::EventValidationStatus;
use crate::database::event::model::{EventCategory, EventRecurrence, EventVisibility};

/// Participant d'un événement et son statut de validation.
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize, ToSchema)]
pub struct Member {
    /// Identifiant Core API du participant.
    #[schema(example = 42)]
    pub id: u64,
    /// Statut de validation de ce participant, appliqué en bloc par
    /// `PATCH /api/v1/events/{event_id}/validation`.
    pub validation_status: EventValidationStatus,
}

/// Droits de l'appelant sur l'événement.
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Deserialize, serde::Serialize, ToSchema)]
pub struct EventPermissionsView {
    /// L'appelant peut modifier l'événement : il en est le créateur assigné, ou il a le rôle
    /// Responsable, Maire ou Admin.
    #[schema(example = true)]
    pub can_edit: bool,
    /// L'appelant peut supprimer l'événement : il en est le créateur. Un Admin assigné peut
    /// modifier sans pouvoir supprimer.
    #[schema(example = true)]
    pub can_delete: bool,
    /// L'appelant peut valider l'événement : Responsable assigné, partageant un groupe avec le
    /// créateur, sur un événement encore en attente.
    #[schema(example = false)]
    pub can_validate: bool,
}

/// Détail d'un événement.
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize, ToSchema)]
pub struct GetEventResultView {
    /// Identifiant de l'événement.
    #[schema(example = 21)]
    pub id: u64,
    /// Intitulé de l'événement.
    #[schema(example = "Conseil municipal")]
    pub name: String,
    /// Description libre, ou `null`.
    #[schema(example = "Ordre du jour envoyé une semaine avant")]
    pub description: Option<String>,
    /// Début de l'événement.
    #[schema(value_type = String, format = DateTime, example = "2026-10-05T18:00:00Z")]
    pub events_start_time: DateTime<Utc>,
    /// Fin de l'événement.
    #[schema(value_type = String, format = DateTime, example = "2026-10-05T20:00:00Z")]
    pub events_end_time: DateTime<Utc>,
    /// Visibilité de l'événement.
    pub visibility: EventVisibility,
    /// Catégorie de l'événement.
    pub category: EventCategory,
    /// Service organisateur, ou `null`.
    #[schema(example = "Secrétariat général")]
    pub service: Option<String>,
    /// Lieu, ou `null`.
    #[schema(example = "Salle du conseil")]
    pub location: Option<String>,
    /// Règle de répétition, ou `null` pour un événement ponctuel.
    pub recurrence: Option<EventRecurrence>,
    /// Identifiant Core API du propriétaire, ou `null`.
    #[schema(example = 42)]
    pub owner: Option<u64>,
    /// Identifiant Core API du créateur : le seul à pouvoir supprimer l'événement.
    #[schema(example = 42)]
    pub created_by: Option<u64>,
    /// Participants assignés et leur statut de validation individuel.
    pub members: Vec<Member>,
    /// Statut de validation global de l'événement.
    pub approval_status: ApprovalStatus,
    /// Droits de l'appelant sur cet événement, à consommer tels quels côté client.
    pub permissions: EventPermissionsView,
}
