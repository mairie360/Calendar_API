use utoipa::{IntoParams, ToSchema};

use crate::endpoints::v1::events::id::members::id::delete::endpoint::RemoveMemberError;

#[derive(Debug, Clone, ToSchema, serde::Deserialize)]
pub struct DeleteMemberParams {
    pub event_id: u64,
    pub member_id: u64,
}

impl IntoParams for DeleteMemberParams {
    fn into_params(
        _parameter_in_provider: impl Fn() -> Option<utoipa::openapi::path::ParameterIn>,
    ) -> Vec<utoipa::openapi::path::Parameter> {
        vec![
            utoipa::openapi::path::ParameterBuilder::new()
                .name("event_id")
                .schema(Some(
                    utoipa::openapi::ObjectBuilder::new()
                        .schema_type(utoipa::openapi::schema::Type::String)
                        .format(Some(utoipa::openapi::SchemaFormat::KnownFormat(
                            utoipa::openapi::KnownFormat::Int64,
                        ))),
                ))
                .required(utoipa::openapi::Required::True)
                // Force this parameter to be treated as a Path parameter
                .parameter_in(utoipa::openapi::path::ParameterIn::Path)
                .build(),
            utoipa::openapi::path::ParameterBuilder::new()
                .name("member_id")
                .schema(Some(
                    utoipa::openapi::ObjectBuilder::new()
                        .schema_type(utoipa::openapi::schema::Type::String)
                        .format(Some(utoipa::openapi::SchemaFormat::KnownFormat(
                            utoipa::openapi::KnownFormat::Int64,
                        ))),
                ))
                .required(utoipa::openapi::Required::True)
                // Force this parameter to be treated as a Path parameter
                .parameter_in(utoipa::openapi::path::ParameterIn::Path)
                .build(),
        ]
    }
}

impl DeleteMemberParams {
    pub fn new(event_id: u64, member_id: u64) -> Self {
        Self {
            event_id,
            member_id,
        }
    }

    pub fn event_id(&self) -> u64 {
        self.event_id
    }

    pub fn member_id(&self) -> u64 {
        self.member_id
    }
}

impl TryFrom<actix_web::web::Path<DeleteMemberParams>> for DeleteMemberParams {
    type Error = RemoveMemberError;

    fn try_from(params: actix_web::web::Path<DeleteMemberParams>) -> Result<Self, Self::Error> {
        Ok(params.into_inner())
    }
}
