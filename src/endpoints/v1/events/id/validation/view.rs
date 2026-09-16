use utoipa::ToSchema;

use crate::database::event::access::view::ApprovalStatus;

/// Statut de validation à appliquer à tous les participants de l'événement.
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Deserialize, ToSchema)]
pub struct UpdateEventValidationView {
    /// `approved` valide l'événement pour tous ses membres, `rejected` le refuse, `pending` le remet en attente.
    pub status: ApprovalStatus,
}
