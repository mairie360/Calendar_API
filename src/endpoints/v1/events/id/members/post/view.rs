use utoipa::ToSchema;

/// Member to assign to the event of the path. An unknown field is a `400`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Deserialize, serde::Serialize, ToSchema)]
#[serde(deny_unknown_fields)]
pub struct PostMemberView {
    /// Core API id of the user to assign. They must be within the caller's assignment
    /// scope, otherwise the answer is `403`.
    #[schema(example = 51)]
    pub user_id: u64,
}
