use utoipa::ToSchema;

use crate::endpoints::v1::events::id::get::view::Member;

/// Members of an event.
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize, ToSchema)]
pub struct GetMembersResultView {
    /// Members assigned to the event, sorted by id.
    pub members: Vec<Member>,
}
