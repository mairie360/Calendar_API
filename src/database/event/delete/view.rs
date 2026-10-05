use std::fmt::Display;

use mairie360_api_lib::database::db_interface::{
    id_from_sql, id_to_sql, ApiRequestDto, QueryParam,
};

/// Deletes event `$1` (its members cascade) and, in the same statement, its recurrence rule when
/// no other event uses it.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct DeleteEventQueryView {
    params: Vec<QueryParam>,
}

impl DeleteEventQueryView {
    pub fn new(event_id: u64) -> Self {
        Self {
            params: vec![QueryParam::I32(id_to_sql(event_id))],
        }
    }

    pub fn event_id(&self) -> u64 {
        id_from_sql(self.params[0].as_i32())
    }
}

impl Display for DeleteEventQueryView {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "event_id: {}", self.event_id())
    }
}

impl ApiRequestDto for DeleteEventQueryView {
    fn query_sql(&self) -> &'static str {
        // The rule's `ON DELETE SET NULL` on `events.recurrence_id` finds no row to update: the
        // only event pointing to it is the one deleted by this same statement.
        "WITH deleted AS (DELETE FROM events WHERE id = $1 RETURNING recurrence_id) \
         DELETE FROM recurrence_rules rr USING deleted \
         WHERE rr.id = deleted.recurrence_id \
           AND NOT EXISTS (SELECT 1 FROM events e WHERE e.recurrence_id = rr.id AND e.id <> $1)"
    }

    fn query_params(&self) -> &[QueryParam] {
        &self.params
    }
}
