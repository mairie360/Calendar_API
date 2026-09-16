use utoipa::ToSchema;

use crate::endpoints::v1::events::id::get::view::Member;

#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize, ToSchema)]
pub struct GetMembersResultView {
    pub members: Vec<Member>,
}
