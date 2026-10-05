use std::fmt::Display;

use mairie360_api_lib::database::db_interface::{
    id_from_sql, id_to_sql, ApiRequestDto, QueryParam,
};

/// Assigns user `$1` to event `$2`. A user already assigned fails with `DbError::UniqueViolation`
/// (unique index on `(event_id, user_id)`).
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct AddUserToEventQueryView {
    params: Vec<QueryParam>,
}

impl AddUserToEventQueryView {
    pub fn new(user_id: u64, event_id: u64) -> Self {
        Self {
            params: vec![
                QueryParam::I32(id_to_sql(user_id)),
                QueryParam::I32(id_to_sql(event_id)),
            ],
        }
    }

    pub fn user_id(&self) -> u64 {
        id_from_sql(self.params[0].as_i32())
    }

    pub fn event_id(&self) -> u64 {
        id_from_sql(self.params[1].as_i32())
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
