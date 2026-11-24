use std::fmt::Display;

use mairie360_api_lib::database::db_interface::{
    id_from_sql, id_to_sql, ApiRequestDto, QueryParam,
};

use crate::database::calendar::get::view::CALENDAR_CACHE_NAMESPACE;

/// Assigns user `$1` to event `$2`. A user already assigned fails with `DbError::UniqueViolation`
/// (unique index on `(event_id, user_id)`).
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct AddUserToEventQueryView {
    params: Vec<QueryParam>,
}

impl AddUserToEventQueryView {
    /// `caller_id` is who assigns: a user assigning themselves is `accepted`, others `pending`.
    pub fn new(user_id: u64, event_id: u64, caller_id: u64) -> Self {
        Self {
            params: vec![
                QueryParam::I32(id_to_sql(user_id)),
                QueryParam::I32(id_to_sql(event_id)),
                QueryParam::I32(id_to_sql(caller_id)),
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
        // A member assigning themselves takes part; anyone else is invited and has not answered.
        "INSERT INTO event_members (user_id, event_id, participation) \
         VALUES ($1, $2, CASE WHEN $1 = $3 THEN 'accepted' ELSE 'pending' END)"
    }

    fn query_params(&self) -> &[QueryParam] {
        &self.params
    }

    /// The calendar pages show this row (MAIR-524): invalidated once the write is committed.
    fn invalidated_read_caches(&self) -> &'static [&'static str] {
        &[CALENDAR_CACHE_NAMESPACE]
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
