use utoipa::ToSchema;

use crate::endpoints::v1::events::id::get::view::Member;

#[derive(Debug, Clone, ToSchema, serde::Serialize)]
pub struct GetMembersResultView {
    members: Vec<Member>,
}

impl GetMembersResultView {
    pub fn new(members: Vec<Member>) -> Self {
        Self { members }
    }

    pub fn members(&self) -> &[Member] {
        &self.members
    }
}
