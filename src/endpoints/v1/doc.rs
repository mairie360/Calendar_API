use crate::endpoints::v1::events::doc::EventsDoc;
use crate::endpoints::v1::get::doc::GetDoc;
use crate::endpoints::v1::params::doc::ParamsDoc;
use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(nest(
    (path = "/events", api = EventsDoc),
    (path = "/params", api = ParamsDoc),
    (path = "/", api = GetDoc),
))]
pub struct V1Doc;
