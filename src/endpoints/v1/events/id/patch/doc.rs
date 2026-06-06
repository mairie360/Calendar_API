use crate::endpoints::v1::events::id::patch::{endpoint::__path_patch, view::PatchEventView};
use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(paths(patch), components(schemas(PatchEventView)))]
pub struct PatchDoc;
