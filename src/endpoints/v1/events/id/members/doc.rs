use crate::endpoints::v1::events::id::members::get::{
    endpoint::__path_get, view::GetMembersResultView,
};
use crate::endpoints::v1::events::id::members::id::doc::IdDoc;
use crate::endpoints::v1::events::id::members::post::{
    endpoint::__path_post, view::PostMemberView,
};
use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(nest(
    (path = "/", api = Doc),
    (path = "/{id}", api = IdDoc),
))]
pub struct MembersDoc;

#[derive(OpenApi)]
#[openapi(
    paths(get, post),
    components(schemas(GetMembersResultView, PostMemberView,))
)]
struct Doc;
