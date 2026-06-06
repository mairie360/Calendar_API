use crate::endpoints::v1::events::post::endpoint::__path_post;
use crate::endpoints::v1::events::post::view::{PostEventResultView, PostEventView};
use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(paths(post), components(schemas(PostEventView, PostEventResultView)))]
pub struct PostDoc;
