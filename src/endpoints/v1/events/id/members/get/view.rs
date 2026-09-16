use utoipa::ToSchema;

use crate::endpoints::v1::events::id::get::view::Member;

/// Participants d'un événement.
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize, ToSchema)]
pub struct GetMembersResultView {
    /// Participants assignés à l'événement, avec leur statut de validation individuel.
    pub members: Vec<Member>,
}
