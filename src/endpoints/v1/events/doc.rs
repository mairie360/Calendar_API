use crate::endpoints::v1::events::id::doc::IdDoc;
use crate::endpoints::v1::events::post::doc::PostDoc;
use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(nest(
    (path = "/", api = PostDoc),
    (path = "/{event_id}", api = IdDoc),
))]
pub struct EventsDoc;
