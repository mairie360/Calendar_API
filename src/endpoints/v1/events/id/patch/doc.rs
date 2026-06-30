use crate::endpoints::v1::events::id::patch::{endpoint::__path_patch_event, view::PatchEventView};
use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(paths(patch_event), components(schemas(PatchEventView)))]
pub struct PatchDoc;
