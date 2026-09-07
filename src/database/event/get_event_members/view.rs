use std::fmt::Display;

use mairie360_api_lib::database::db_interface::{ApiRequestDto, QueryParam};
use utoipa::ToSchema;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct GetEventMemberQueryView {
    params: Vec<QueryParam>,
}

impl GetEventMemberQueryView {
    pub fn new(event_id: u64) -> Self {
        Self {
            params: vec![QueryParam::I32(event_id as i32)],
        }
    }

    pub fn event_id(&self) -> u64 {
        self.params[0].as_i32() as u64
    }
}

impl ApiRequestDto for GetEventMemberQueryView {
    fn query_sql(&self) -> &'static str {
        "SELECT to_jsonb(t) FROM (
            SELECT user_id, validation_status FROM event_members WHERE event_id = $1
         ) t"
    }

    fn query_params(&self) -> &[QueryParam] {
        &self.params
    }
}

impl Display for GetEventMemberQueryView {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "event_id: {}", self.event_id())
    }
}

#[derive(Copy, Debug, PartialEq, Eq, Clone, serde::Deserialize, serde::Serialize, ToSchema)]
#[serde(rename_all = "snake_case")]
pub enum EventValidationStatus {
    Validated,
    Refused,
    Pending,
}

impl From<String> for EventValidationStatus {
    fn from(s: String) -> Self {
        match s.as_str() {
            "validated" => EventValidationStatus::Validated,
            "refused" => EventValidationStatus::Refused,
            "pending" => EventValidationStatus::Pending,
            _ => EventValidationStatus::Pending,
        }
    }
}

impl Display for EventValidationStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            EventValidationStatus::Validated => "validated",
            EventValidationStatus::Refused => "refused",
            EventValidationStatus::Pending => "pending",
        };
        write!(f, "{}", value)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct Member {
    user_id: i32,
    validation_status: EventValidationStatus,
}

impl Member {
    pub fn new(user_id: i32, validation_status: EventValidationStatus) -> Self {
        Self {
            user_id,
            validation_status,
        }
    }

    pub fn user_id(&self) -> i32 {
        self.user_id
    }

    pub fn validation_status(&self) -> EventValidationStatus {
        self.validation_status
    }
}
