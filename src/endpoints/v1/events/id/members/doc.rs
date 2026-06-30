use crate::endpoints::v1::events::id::members::get::{
    endpoint::__path_get_event_members, view::GetMembersResultView,
};
use crate::endpoints::v1::events::id::members::id::doc::IdDoc;
use crate::endpoints::v1::events::id::members::post::{
    endpoint::__path_add_event_member, view::PostMemberView,
};
use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(nest(
    (path = "/", api = Doc),
    (path = "/{member_id}", api = IdDoc),
))]
pub struct MembersDoc;

#[derive(OpenApi)]
#[openapi(
    paths(get_event_members, add_event_member),
    components(schemas(GetMembersResultView, PostMemberView,))
)]
struct Doc;
