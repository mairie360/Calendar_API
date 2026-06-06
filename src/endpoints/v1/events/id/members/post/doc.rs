use crate::endpoints::v1::events::id::members::post::endpoint::__path_post;
use crate::endpoints::v1::events::id::members::post::view::PostMemberView;
use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(paths(post), components(schemas(PostMemberView)))]
pub struct PostDoc;
