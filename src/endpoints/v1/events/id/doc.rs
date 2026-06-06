use crate::endpoints::v1::events::id::delete::endpoint::__path_delete;
use crate::endpoints::v1::events::id::get::{endpoint::__path_get, view::GetEventResultView};
use crate::endpoints::v1::events::id::members::doc::MembersDoc;
use crate::endpoints::v1::events::id::patch::{
    endpoint::__path_patch,
    view::{PatchEventParams, PatchEventView},
};

use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(nest(
    (path = "/", api = Doc),
    (path = "/members", api = MembersDoc),
))]
pub struct IdDoc;

#[derive(OpenApi)]
#[openapi(
    paths(delete, get, patch),
    components(schemas(GetEventResultView, PatchEventParams, PatchEventView,))
)]
struct Doc;
