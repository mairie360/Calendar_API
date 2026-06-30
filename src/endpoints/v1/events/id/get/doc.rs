use crate::endpoints::v1::events::id::get::{endpoint::__path_get_event, view::GetEventResultView};
use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(paths(get_event), components(schemas(GetEventResultView)))]
pub struct GetDoc;
