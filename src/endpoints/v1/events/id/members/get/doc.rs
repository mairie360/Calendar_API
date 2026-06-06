use crate::endpoints::v1::events::id::members::get::{
    endpoint::__path_get, view::GetMembersResultView,
};
use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(paths(get), components(schemas(GetMembersResultView)))]
pub struct GetDoc;
