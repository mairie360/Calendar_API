use utoipa::{IntoParams, ToSchema};

use crate::endpoints::v1::events::id::members::id::delete::endpoint::RemoveMemberError;

#[derive(Debug, Clone, ToSchema, serde::Deserialize)]
pub struct DeleteMemberParams {
    id: Option<String>,
    member_id: Option<String>,
}

impl IntoParams for DeleteMemberParams {
    fn into_params(
        parameter_in_provider: impl Fn() -> Option<utoipa::openapi::path::ParameterIn>,
    ) -> Vec<utoipa::openapi::path::Parameter> {
        vec![
            utoipa::openapi::path::ParameterBuilder::new()
                .name("id")
                .schema(Some(
                    utoipa::openapi::ObjectBuilder::new()
                        .schema_type(utoipa::openapi::schema::Type::String)
                        .format(Some(utoipa::openapi::SchemaFormat::KnownFormat(
                            utoipa::openapi::KnownFormat::Int64,
                        ))),
                ))
                .required(utoipa::openapi::Required::True)
                .parameter_in(parameter_in_provider().unwrap_or_default())
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
                .parameter_in(parameter_in_provider().unwrap_or_default())
                .build(),
        ]
    }
}

impl DeleteMemberParams {
    pub fn new(id: Option<String>, member_id: Option<String>) -> Self {
        Self { id, member_id }
    }

    pub fn id(&self) -> &Option<String> {
        &self.id
    }

    pub fn member_id(&self) -> &Option<String> {
        &self.member_id
    }
}

impl TryFrom<actix_web::web::Query<DeleteMemberParams>> for DeleteMemberParams {
    type Error = RemoveMemberError;

    fn try_from(params: actix_web::web::Query<DeleteMemberParams>) -> Result<Self, Self::Error> {
        if params.id.is_none() {
            return Err(RemoveMemberError::UnknownEvent);
        }
        if params.member_id.is_none() {
            return Err(RemoveMemberError::UnknownMember);
        }
        Ok(params.into_inner())
    }
}
