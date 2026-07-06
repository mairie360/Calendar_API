use chrono::{DateTime, Utc};
use utoipa::{IntoParams, ToSchema};

use crate::database::calendar::get::view::Event;
use crate::endpoints::v1::get::endpoint::GetCalendarError;

#[derive(Debug, Clone, ToSchema, serde::Deserialize)]
pub struct GetCalendarParams {
    #[schema(value_type = String, format = DateTime)]
    start: Option<DateTime<Utc>>,
    #[schema(value_type = String, format = DateTime)]
    end: Option<DateTime<Utc>>,
}

impl IntoParams for GetCalendarParams {
    fn into_params(
        parameter_in_provider: impl Fn() -> Option<utoipa::openapi::path::ParameterIn>,
    ) -> Vec<utoipa::openapi::path::Parameter> {
        vec![
            utoipa::openapi::path::ParameterBuilder::new()
                .name("start")
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
                .name("end")
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
        ]
    }
}

impl GetCalendarParams {
    pub fn new(start: Option<DateTime<Utc>>, end: Option<DateTime<Utc>>) -> Self {
        Self { start, end }
    }

    pub fn start(&self) -> &Option<DateTime<Utc>> {
        &self.start
    }

    pub fn end(&self) -> &Option<DateTime<Utc>> {
        &self.end
    }
}

impl TryFrom<actix_web::web::Query<GetCalendarParams>> for GetCalendarParams {
    type Error = GetCalendarError;

    fn try_from(params: actix_web::web::Query<GetCalendarParams>) -> Result<Self, Self::Error> {
        if (params.end().is_none() && params.start().is_none()) || (params.end() < params.start()) {
            return Err(GetCalendarError::BadParams);
        }
        Ok(params.into_inner())
    }
}

#[derive(Debug, Clone, serde::Serialize, ToSchema)]
pub struct EventView {
    id: u64,
    name: String,
    #[schema(value_type = String, format = DateTime)]
    start: DateTime<Utc>,
    #[schema(value_type = String, format = DateTime)]
    end: DateTime<Utc>,
}

impl EventView {
    pub fn new(id: u64, name: String, start: DateTime<Utc>, end: DateTime<Utc>) -> Self {
        Self {
            id,
            name,
            start,
            end,
        }
    }

    pub fn id(&self) -> u64 {
        self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn start(&self) -> &DateTime<Utc> {
        &self.start
    }

    pub fn end(&self) -> &DateTime<Utc> {
        &self.end
    }
}

#[derive(Debug, Clone, serde::Serialize, ToSchema)]
pub struct GetCalendarResultView {
    events: Vec<EventView>,
}

impl GetCalendarResultView {
    pub fn new(events: Vec<EventView>) -> Self {
        Self { events }
    }

    pub fn events(&self) -> &[EventView] {
        &self.events
    }
}

impl From<Vec<Event>> for GetCalendarResultView {
    fn from(events: Vec<Event>) -> Self {
        Self {
            events: events
                .into_iter()
                .map(|e| {
                    EventView::new(
                        e.id() as u64,
                        e.name().to_string(),
                        *e.start_date(),
                        *e.end_date(),
                    )
                })
                .collect(),
        }
    }
}
