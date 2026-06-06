use crate::endpoints::v1::events::id::get::{endpoint::__path_get, view::GetEventResultView};
use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(paths(get), components(schemas(GetEventResultView)))]
pub struct GetDoc;
