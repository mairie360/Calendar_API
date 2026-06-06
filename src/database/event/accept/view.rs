use std::fmt::Display;

use mairie360_api_lib::database::db_interface::DatabaseQueryView;

pub struct AcceptEventQueryView {
    user_id: u64,
    event_id: u64,
}

impl AcceptEventQueryView {
    pub fn new(user_id: u64, event_id: u64) -> Self {
        Self { user_id, event_id }
    }
}

impl DatabaseQueryView for AcceptEventQueryView {
    fn get_request(&self) -> String {
        "UPDATE event_members SET  WHERE group_id = $1 AND user_id = $2".to_string()
    }
}

impl Display for AcceptEventQueryView {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "user_id: {}, event_id: {}", self.user_id, self.event_id)
    }
}
