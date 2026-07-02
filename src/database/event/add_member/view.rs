use std::fmt::Display;

use mairie360_api_lib::database::db_interface::DatabaseQueryView;

pub struct AddUserToEventQueryView {
    user_id: u64,
    event_id: u64,
}

impl AddUserToEventQueryView {
    pub fn new(user_id: u64, event_id: u64) -> Self {
        Self { user_id, event_id }
    }

    pub fn user_id(&self) -> u64 {
        self.user_id
    }

    pub fn event_id(&self) -> u64 {
        self.event_id
    }
}

impl DatabaseQueryView for AddUserToEventQueryView {
    fn get_request(&self) -> String {
        "INSERT INTO event_members (user_id, event_id) VALUES ($1, $2)".to_string()
    }
}

impl Display for AddUserToEventQueryView {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "user_id: {}, event_id: {}", self.user_id, self.event_id)
    }
}
