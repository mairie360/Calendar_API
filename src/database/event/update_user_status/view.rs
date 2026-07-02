use std::fmt::Display;

use mairie360_api_lib::database::db_interface::DatabaseQueryView;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum EventValidationStatus {
    Validated,
    Refused,
}

impl From<String> for EventValidationStatus {
    fn from(s: String) -> Self {
        match s.as_str() {
            "validated" => EventValidationStatus::Validated,
            "refused" => EventValidationStatus::Refused,
            _ => EventValidationStatus::Validated,
        }
    }
}

impl ToString for EventValidationStatus {
    fn to_string(&self) -> String {
        match self {
            EventValidationStatus::Validated => "validated".to_string(),
            EventValidationStatus::Refused => "refused".to_string(),
        }
    }
}

pub struct EventStatusUpdateQueryView {
    user_id: u64,
    event_id: u64,
    status: EventValidationStatus,
}

impl EventStatusUpdateQueryView {
    pub fn new(user_id: u64, event_id: u64, status: EventValidationStatus) -> Self {
        Self {
            user_id,
            event_id,
            status,
        }
    }

    pub fn user_id(&self) -> u64 {
        self.user_id
    }

    pub fn event_id(&self) -> u64 {
        self.event_id
    }

    pub fn status(&self) -> EventValidationStatus {
        self.status
    }
}

impl DatabaseQueryView for EventStatusUpdateQueryView {
    fn get_request(&self) -> String {
        "UPDATE event_members SET validation_status = $1 WHERE group_id = $2 AND user_id = $3"
            .to_string()
    }
}

impl Display for EventStatusUpdateQueryView {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "user_id: {}, event_id: {}, status: {:?}",
            self.user_id, self.event_id, self.status
        )
    }
}
