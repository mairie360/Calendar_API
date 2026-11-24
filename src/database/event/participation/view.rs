use mairie360_api_lib::database::db_interface::{id_to_sql, ApiRequestDto, QueryParam};

use crate::database::calendar::get::view::CALENDAR_CACHE_NAMESPACE;
use crate::database::event::get_event_members::view::Participation;

/// Records the answer `$3` of member `$2` to the invitation of event `$1` and returns the event id.
/// No row (`DbError::NotFound`) means the caller is no longer assigned (removed concurrently).
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SetParticipationQueryView {
    params: Vec<QueryParam>,
}

impl SetParticipationQueryView {
    pub fn new(event_id: u64, user_id: u64, participation: Participation) -> Self {
        Self {
            params: vec![
                QueryParam::I32(id_to_sql(event_id)),
                QueryParam::I32(id_to_sql(user_id)),
                QueryParam::Text(participation.as_db().to_string()),
            ],
        }
    }
}

impl ApiRequestDto for SetParticipationQueryView {
    fn query_sql(&self) -> &'static str {
        "UPDATE event_members SET participation = $3 \
         WHERE event_id = $1 AND user_id = $2 RETURNING event_id"
    }

    fn query_params(&self) -> &[QueryParam] {
        &self.params
    }

    /// The calendar pages show the caller's answer: invalidated once the write is committed.
    fn invalidated_read_caches(&self) -> &'static [&'static str] {
        &[CALENDAR_CACHE_NAMESPACE]
    }
}
