use crate::endpoints::v1::get::endpoint::__path_get;
use crate::endpoints::v1::get::view::{GetCalendarParams, GetCalendarResultView};
use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    paths(get),
    components(schemas(GetCalendarParams, GetCalendarResultView))
)]
pub struct GetDoc;
