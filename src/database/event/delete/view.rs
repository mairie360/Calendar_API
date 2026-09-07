use std::fmt::Display;

use mairie360_api_lib::database::db_interface::{ApiRequestDto, QueryParam};

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct DeleteEventQueryView {
    params: Vec<QueryParam>,
}

impl DeleteEventQueryView {
    pub fn new(event_id: u64) -> Self {
        Self {
            params: vec![QueryParam::I32(event_id as i32)],
        }
    }

    pub fn event_id(&self) -> u64 {
        self.params[0].as_i32() as u64
    }
}

impl Display for DeleteEventQueryView {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "event_id: {}", self.event_id())
    }
}

impl ApiRequestDto for DeleteEventQueryView {
    fn query_sql(&self) -> &'static str {
        "DELETE FROM events WHERE id = $1"
    }

    fn query_params(&self) -> &[QueryParam] {
        &self.params
    }
}
