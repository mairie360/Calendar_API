use crate::endpoints::v1::events::id::delete::endpoint::__path_delete_event;
use crate::endpoints::v1::events::id::get::{endpoint::__path_get_event, view::GetEventResultView};
use crate::endpoints::v1::events::id::members::doc::MembersDoc;
use crate::endpoints::v1::events::id::patch::{
    endpoint::__path_patch_event,
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
    paths(delete_event, get_event, patch_event),
    components(schemas(GetEventResultView, PatchEventParams, PatchEventView,))
)]
struct Doc;
