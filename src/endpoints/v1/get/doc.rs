use crate::endpoints::v1::get::endpoint::__path_get_calendar;
use crate::endpoints::v1::get::view::{GetCalendarParams, GetCalendarResultView};
use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    paths(get_calendar),
    components(schemas(GetCalendarParams, GetCalendarResultView))
)]
pub struct GetDoc;
