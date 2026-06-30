use crate::endpoints::v1::events::id::members::get::{
    endpoint::__path_get_event_members, view::GetMembersResultView,
};
use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(paths(get_event_members), components(schemas(GetMembersResultView)))]
pub struct GetDoc;
