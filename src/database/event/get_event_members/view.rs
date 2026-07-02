use std::fmt::Display;

use mairie360_api_lib::database::db_interface::DatabaseQueryView;
use utoipa::ToSchema;

pub struct GetEventMemberQueryView {
    event_id: u64,
}

impl GetEventMemberQueryView {
    pub fn new(event_id: u64) -> Self {
        Self { event_id }
    }

    pub fn event_id(&self) -> u64 {
        self.event_id
    }
}

impl DatabaseQueryView for GetEventMemberQueryView {
    fn get_request(&self) -> String {
        "SELECT * FROM event_members WHERE event_id = $1".to_string()
    }
}

impl Display for GetEventMemberQueryView {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "event_id: {}", self.event_id)
    }
}

#[derive(
    Copy, Debug, sqlx::Type, PartialEq, Eq, Clone, serde::Deserialize, serde::Serialize, ToSchema,
)]
#[sqlx(type_name = "event_validation_status", rename_all = "snake_case")]
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

impl ToString for EventValidationStatus {
    fn to_string(&self) -> String {
        match self {
            EventValidationStatus::Validated => "validated".to_string(),
            EventValidationStatus::Refused => "refused".to_string(),
            EventValidationStatus::Pending => "pending".to_string(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, sqlx::FromRow)]
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
