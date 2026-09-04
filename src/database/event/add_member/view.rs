use std::fmt::Display;

use mairie360_api_lib::database::db_interface::{ApiRequestDto, QueryParam};

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct AddUserToEventQueryView {
    params: Vec<QueryParam>,
}

impl AddUserToEventQueryView {
    pub fn new(user_id: u64, event_id: u64) -> Self {
        Self {
            params: vec![
                QueryParam::I32(user_id as i32),
                QueryParam::I32(event_id as i32),
            ],
        }
    }

    pub fn user_id(&self) -> u64 {
        self.params[0].as_i32() as u64
    }

    pub fn event_id(&self) -> u64 {
        self.params[1].as_i32() as u64
    }
}

impl ApiRequestDto for AddUserToEventQueryView {
    fn query_sql(&self) -> &'static str {
        "INSERT INTO event_members (user_id, event_id) VALUES ($1, $2)"
    }

    fn query_params(&self) -> &[QueryParam] {
        &self.params
    }
}

impl Display for AddUserToEventQueryView {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "user_id: {}, event_id: {}",
            self.user_id(),
            self.event_id()
        )
    }
}
