use utoipa::ToSchema;

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Deserialize, serde::Serialize, ToSchema)]
pub struct PostMemberView {
    pub user_id: u64,
}
