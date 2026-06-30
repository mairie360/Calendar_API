use crate::endpoints::v1::events::id::delete::endpoint::__path_delete_event;
use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(paths(delete_event))]
pub struct DeleteDoc;
