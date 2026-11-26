use crate::endpoints::v1::events::doc::EventsDoc;
use crate::endpoints::v1::get::doc::GetDoc;
use crate::endpoints::v1::search::doc::SearchDoc;
use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(nest(
    (path = "/events", api = EventsDoc),
    (path = "/search", api = SearchDoc),
    (path = "/", api = GetDoc),
))]
pub struct V1Doc;
