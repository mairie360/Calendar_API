use std::fmt::Display;

use mairie360_api_lib::database::db_interface::{ApiRequestDto, QueryParam};

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

impl Display for EventValidationStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            EventValidationStatus::Validated => "validated",
            EventValidationStatus::Refused => "refused",
        };
        write!(f, "{}", value)
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct EventStatusUpdateQueryView {
    params: Vec<QueryParam>,
}

impl EventStatusUpdateQueryView {
    pub fn new(user_id: u64, event_id: u64, status: EventValidationStatus) -> Self {
        Self {
            params: vec![
                QueryParam::Text(status.to_string()),
                QueryParam::I32(event_id as i32),
                QueryParam::I32(user_id as i32),
            ],
        }
    }

    pub fn status(&self) -> EventValidationStatus {
        EventValidationStatus::from(self.params[0].as_text().to_string())
    }

    pub fn event_id(&self) -> u64 {
        self.params[1].as_i32() as u64
    }

    pub fn user_id(&self) -> u64 {
        self.params[2].as_i32() as u64
    }
}

impl ApiRequestDto for EventStatusUpdateQueryView {
    fn query_sql(&self) -> &'static str {
        "UPDATE event_members SET validation_status = $1::event_validation_status \
         WHERE event_id = $2 AND user_id = $3"
    }

    fn query_params(&self) -> &[QueryParam] {
        &self.params
    }
}

impl Display for EventStatusUpdateQueryView {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "user_id: {}, event_id: {}, status: {:?}",
            self.user_id(),
            self.event_id(),
            self.status()
        )
    }
}
