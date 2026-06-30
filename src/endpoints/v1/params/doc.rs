use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(nest())]
pub struct ParamsDoc;
