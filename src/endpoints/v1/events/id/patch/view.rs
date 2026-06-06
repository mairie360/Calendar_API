use actix_web::web;
use chrono::{DateTime, Utc};
use utoipa::{IntoParams, ToSchema};

use crate::endpoints::v1::events::{id::patch::endpoint::PatchEventError, post::view::Visibility};

#[derive(Debug, Clone, ToSchema, serde::Deserialize)]
pub struct PatchEventParams {
    pub id: Option<u64>,
    pub reccurent: Option<bool>,
}

impl TryFrom<actix_web::web::Query<PatchEventParams>> for PatchEventParams {
    type Error = PatchEventError;

    fn try_from(params: actix_web::web::Query<PatchEventParams>) -> Result<Self, Self::Error> {
        let params = params.into_inner();
        if params.id.is_none() && params.reccurent.is_none() {
            return Err(PatchEventError::BadParams);
        }
        Ok(params)
    }
}

impl IntoParams for PatchEventParams {
    fn into_params(
        parameter_in_provider: impl Fn() -> Option<utoipa::openapi::path::ParameterIn>,
    ) -> Vec<utoipa::openapi::path::Parameter> {
        vec![
            utoipa::openapi::path::ParameterBuilder::new()
                .name("id")
                .schema(Some(
                    utoipa::openapi::ObjectBuilder::new()
                        .schema_type(utoipa::openapi::schema::Type::Integer)
                        .format(Some(utoipa::openapi::SchemaFormat::KnownFormat(
                            utoipa::openapi::KnownFormat::Int64,
                        ))),
                ))
                .required(utoipa::openapi::Required::True)
                .parameter_in(parameter_in_provider().unwrap_or_default())
                .build(),
            utoipa::openapi::path::ParameterBuilder::new()
                .name("reccurent")
                .schema(Some(
                    utoipa::openapi::ObjectBuilder::new()
                        .schema_type(utoipa::openapi::schema::Type::Boolean),
                ))
                .required(utoipa::openapi::Required::False)
                .parameter_in(parameter_in_provider().unwrap_or_default())
                .build(),
        ]
    }
}

#[derive(Debug, Clone, ToSchema, serde::Deserialize)]
pub struct PatchEventView {
    name: Option<String>,
    description: Option<String>,
    #[schema(value_type = String, format = DateTime)]
    event_start_time: Option<DateTime<Utc>>,
    #[schema(value_type = String, format = DateTime)]
    event_end_time: Option<DateTime<Utc>>,
    intervalle: Option<u64>,
    visibility: Option<Visibility>,
    #[schema(value_type = String, format = DateTime)]
    reccurence_end_date: Option<DateTime<Utc>>,
}

impl PatchEventView {
    pub fn new(
        name: Option<String>,
        description: Option<String>,
        event_start_time: Option<DateTime<Utc>>,
        event_end_time: Option<DateTime<Utc>>,
        intervalle: Option<u64>,
        visibility: Option<Visibility>,
        reccurence_end_date: Option<DateTime<Utc>>,
    ) -> Self {
        Self {
            name,
            description,
            event_start_time,
            event_end_time,
            intervalle,
            visibility,
            reccurence_end_date,
        }
    }

    pub fn name(&self) -> Option<&str> {
        self.name.as_deref()
    }

    pub fn description(&self) -> Option<&str> {
        self.description.as_deref()
    }

    pub fn event_start_time(&self) -> Option<&DateTime<Utc>> {
        self.event_start_time.as_ref()
    }

    pub fn event_end_time(&self) -> Option<&DateTime<Utc>> {
        self.event_end_time.as_ref()
    }

    pub fn intervalle(&self) -> Option<u64> {
        self.intervalle
    }

    pub fn visibility(&self) -> Option<&Visibility> {
        self.visibility.as_ref()
    }

    pub fn reccurence_end_date(&self) -> Option<&DateTime<Utc>> {
        self.reccurence_end_date.as_ref()
    }
}

impl TryFrom<web::Json<PatchEventView>> for PatchEventView {
    type Error = PatchEventError;

    fn try_from(params: web::Json<PatchEventView>) -> Result<PatchEventView, Self::Error> {
        Ok(params.into_inner())
    }
}
