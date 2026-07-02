use actix_web::web;
use chrono::{DateTime, Utc};
use utoipa::ToSchema;

use crate::endpoints::v1::events::post::endpoint::PostEventError;

#[derive(Debug, Clone, PartialEq, ToSchema, serde::Deserialize, serde::Serialize)]
pub enum RecurrenceType {
    Daily,
    Weekly,
    Monthly,
    Error,
}

impl From<String> for RecurrenceType {
    fn from(s: String) -> Self {
        match s.as_str() {
            "daily" => RecurrenceType::Daily,
            "weekly" => RecurrenceType::Weekly,
            "monthly" => RecurrenceType::Monthly,
            _ => RecurrenceType::Error,
        }
    }
}

impl RecurrenceType {
    pub fn to_string(&self) -> &str {
        match self {
            RecurrenceType::Daily => "daily",
            RecurrenceType::Weekly => "weekly",
            RecurrenceType::Monthly => "monthly",
            RecurrenceType::Error => "",
        }
    }
}

#[derive(Debug, Clone, PartialEq, ToSchema, serde::Deserialize, serde::Serialize)]
pub enum Visibility {
    Public,
    Private,
    Error,
}

impl From<String> for Visibility {
    fn from(s: String) -> Self {
        match s.as_str() {
            "public" => Visibility::Public,
            "private" => Visibility::Private,
            _ => Visibility::Error,
        }
    }
}

impl Visibility {
    pub fn to_string(&self) -> &str {
        match self {
            Visibility::Public => "public",
            Visibility::Private => "private",
            Visibility::Error => "",
        }
    }
}

#[derive(Debug, Clone, PartialEq, ToSchema, serde::Deserialize, serde::Serialize)]
pub struct Recurrence {
    type_recurrence: RecurrenceType,
    intervalle: u64,
    #[schema(value_type = String, format = DateTime)]
    recurrence_end_date: Option<DateTime<Utc>>,
    name: String,
    description: Option<String>,
    visibility: Option<Visibility>,
    owner_group_id: Option<u64>,
}

impl Recurrence {
    pub fn new(
        type_recurrence: RecurrenceType,
        intervalle: u64,
        recurrence_end_date: Option<DateTime<Utc>>,
        name: String,
        description: Option<String>,
        visibility: Option<Visibility>,
        owner_group_id: Option<u64>,
    ) -> Self {
        Self {
            type_recurrence,
            intervalle,
            recurrence_end_date,
            name,
            description,
            visibility,
            owner_group_id,
        }
    }

    pub fn type_recurrence(&self) -> &RecurrenceType {
        &self.type_recurrence
    }

    pub fn intervalle(&self) -> u64 {
        self.intervalle
    }

    pub fn recurrence_end_date(&self) -> &Option<DateTime<Utc>> {
        &self.recurrence_end_date
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn description(&self) -> &Option<String> {
        &self.description
    }

    pub fn visibility(&self) -> &Option<Visibility> {
        &self.visibility
    }

    pub fn owner_group_id(&self) -> &Option<u64> {
        &self.owner_group_id
    }
}

#[derive(Debug, Clone, PartialEq, ToSchema, serde::Deserialize, serde::Serialize)]
pub struct PostEventView {
    recurrence: Option<Recurrence>,
    #[schema(value_type = String, format = DateTime)]
    events_start_time: DateTime<Utc>,
    #[schema(value_type = String, format = DateTime)]
    events_end_time: DateTime<Utc>,
    custom_name: Option<String>,
    custom_description: Option<String>,
    custom_visibility: Option<Visibility>,
    owner_group_id: Option<u64>,
}

impl PostEventView {
    pub fn new(
        recurrence: Option<Recurrence>,
        events_start_time: DateTime<Utc>,
        events_end_time: DateTime<Utc>,
        custom_name: Option<String>,
        custom_description: Option<String>,
        custom_visibility: Option<Visibility>,
        owner_group_id: Option<u64>,
    ) -> Self {
        Self {
            recurrence,
            events_start_time,
            events_end_time,
            custom_name,
            custom_description,
            custom_visibility,
            owner_group_id,
        }
    }

    pub fn recurrence(&self) -> &Option<Recurrence> {
        &self.recurrence
    }

    pub fn events_start_time(&self) -> &DateTime<Utc> {
        &self.events_start_time
    }

    pub fn events_end_time(&self) -> &DateTime<Utc> {
        &self.events_end_time
    }

    pub fn custom_name(&self) -> Option<String> {
        self.custom_name.clone()
    }

    pub fn custom_description(&self) -> &Option<String> {
        &self.custom_description
    }

    pub fn custom_visibility(&self) -> &Option<Visibility> {
        &self.custom_visibility
    }

    pub fn owner_group_id(&self) -> &Option<u64> {
        &self.owner_group_id
    }
}

impl TryFrom<web::Json<PostEventView>> for PostEventView {
    type Error = PostEventError;

    fn try_from(params: web::Json<PostEventView>) -> Result<PostEventView, Self::Error> {
        Ok(params.into_inner())
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, ToSchema)]
pub struct PostEventResultView {
    pub event_id: u64,
}

impl PostEventResultView {
    pub fn new(event_id: u64) -> Self {
        Self { event_id }
    }

    pub fn event_id(&self) -> &u64 {
        &self.event_id
    }
}
