use crate::endpoints::v1::events::id::members::post::endpoint::__path_add_event_member;
use crate::endpoints::v1::events::id::members::post::view::PostMemberView;
use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(paths(add_event_member), components(schemas(PostMemberView)))]
pub struct PostDoc;
