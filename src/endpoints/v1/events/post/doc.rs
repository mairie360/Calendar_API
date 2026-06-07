use crate::endpoints::v1::events::post::endpoint::__path_create_event;
use crate::endpoints::v1::events::post::view::{PostEventResultView, PostEventView};
use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    paths(create_event),
    components(schemas(PostEventView, PostEventResultView))
)]
pub struct PostDoc;
