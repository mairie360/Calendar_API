use utoipa::ToSchema;

/// Member to assign to the event of the path. An unknown field is a `400`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Deserialize, serde::Serialize, ToSchema)]
#[serde(deny_unknown_fields)]
pub struct PostMemberView {
    /// Identifiant Core API de l'utilisateur à assigner. Il doit être dans le périmètre
    /// d'assignation de l'appelant, sans quoi la réponse est `403`.
    #[schema(example = 51)]
    pub user_id: u64,
}
