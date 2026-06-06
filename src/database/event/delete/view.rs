use std::fmt::Display;

use mairie360_api_lib::database::db_interface::DatabaseQueryView;

pub struct DeleteEventQueryView {
    pub event_id: u64,
}

impl DeleteEventQueryView {
    pub fn new(event_id: u64) -> Self {
        Self { event_id }
    }

    pub fn event_id(&self) -> u64 {
        self.event_id
    }
}

impl Display for DeleteEventQueryView {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "event_id: {}", self.event_id)
    }
}

impl DatabaseQueryView for DeleteEventQueryView {
    fn get_request(&self) -> String {
        format!("DELETE FROM events WHERE id = $1")
    }
}
