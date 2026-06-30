use crate::endpoints::v1::events::id::members::id::delete::doc::DeleteDoc;
use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(nest(
    (path = "/", api = DeleteDoc),
))]
pub struct IdDoc;
