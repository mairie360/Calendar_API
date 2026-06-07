use crate::endpoints::v1::events::id::members::id::delete::endpoint::__path_remove_event_member;
use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(paths(remove_event_member))]
pub struct DeleteDoc;
