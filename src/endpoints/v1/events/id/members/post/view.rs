use utoipa::ToSchema;

/// Participant à assigner à l'événement du chemin.
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Deserialize, serde::Serialize, ToSchema)]
pub struct PostMemberView {
    /// Identifiant Core API de l'utilisateur à assigner. Il doit être dans le périmètre
    /// d'assignation de l'appelant, sans quoi la réponse est `403`.
    #[schema(example = 51)]
    pub user_id: u64,
}
