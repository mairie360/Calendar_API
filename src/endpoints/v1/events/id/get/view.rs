use chrono::{DateTime, Utc};
use utoipa::ToSchema;

use crate::{
    database::event::get_event_members::view::EventValidationStatus,
    endpoints::v1::events::post::view::Visibility,
};

#[derive(Debug, Clone, PartialEq, serde::Deserialize, serde::Serialize, ToSchema)]
pub enum MemberType {
    Group,
    User,
    Error,
}

impl From<String> for MemberType {
    fn from(s: String) -> Self {
        match s.as_str() {
            "group" => Self::Group,
            "user" => Self::User,
            _ => Self::Error,
        }
    }
}

impl MemberType {
    pub fn to_string(&self) -> &str {
        match self {
            Self::Group => "group",
            Self::User => "user",
            Self::Error => "",
        }
    }
}

#[derive(Debug, Clone, PartialEq, serde::Deserialize, serde::Serialize, ToSchema)]
pub struct Member {
    id: u64,
    // member_type: MemberType,
    validation_status: EventValidationStatus,
}

impl Member {
    // pub fn new(id: u64, member_type: MemberType, regular: bool) -> Self {
    pub fn new(id: u64, validation_status: EventValidationStatus) -> Self {
        Self {
            id,
            // member_type,
            validation_status,
        }
    }

    pub fn id(&self) -> u64 {
        self.id
    }

    // pub fn member_type(&self) -> &str {
    //     self.member_type.to_string()
    // }

    pub fn validation_status(&self) -> EventValidationStatus {
        self.validation_status.clone()
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, ToSchema)]
pub struct GetEventResultView {
    id: u64,
    recurrence_id: Option<u64>,
    #[schema(value_type = String, format = DateTime)]
    events_start_time: DateTime<Utc>,
    #[schema(value_type = String, format = DateTime)]
    events_end_time: DateTime<Utc>,
    name: String,
    description: Option<String>,
    visibility: Option<Visibility>,
    owner: u64,
    members: Vec<Member>,
}

impl GetEventResultView {
    pub fn new(
        id: u64,
        recurrence_id: Option<u64>,
        events_start_time: DateTime<Utc>,
        events_end_time: DateTime<Utc>,
        name: &str,
        description: Option<&str>,
        visibility: Option<Visibility>,
        owner: u64,
        members: Vec<Member>,
    ) -> Self {
        Self {
            id,
            recurrence_id,
            events_start_time,
            events_end_time,
            name: name.to_string(),
            description: description.map(|d| d.to_string()),
            visibility,
            owner,
            members,
        }
    }
}
