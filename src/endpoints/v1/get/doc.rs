use crate::endpoints::v1::get::endpoint::__path_get_calendar;
use crate::endpoints::v1::get::view::{EventView, GetCalendarResultView};
use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    paths(get_calendar),
    components(schemas(GetCalendarResultView, EventView))
)]
pub struct GetDoc;
