use std::fmt::Display;

use mairie360_api_lib::database::db_interface::{ApiRequestDto, QueryParam};

use crate::database::event::model::EventInput;

/// Crée un événement dont l'utilisateur `$1` est créateur et propriétaire, avec sa règle de répétition
/// éventuelle, et renvoie son identifiant.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CreateEventQueryView {
    params: Vec<QueryParam>,
}

impl CreateEventQueryView {
    pub fn new(creator_id: u64, input: &EventInput) -> Self {
        let mut params = vec![QueryParam::I32(creator_id as i32)];
        params.extend(input.query_params());
        Self { params }
    }

    pub fn creator_id(&self) -> u64 {
        self.params[0].as_i32() as u64
    }

    pub fn name(&self) -> &str {
        self.params[1].as_text()
    }
}

impl ApiRequestDto for CreateEventQueryView {
    fn query_sql(&self) -> &'static str {
        concat!(
            "WITH rule AS ( \
                INSERT INTO recurrence_rules (type_recurrence, intervalle, days_of_week, start_date, \
                    end_date, start_time, duration, visibility, owner_id) \
                SELECT ",
            crate::recurrence_rule_values_sql!(),
            ", $1 WHERE $10 \
                RETURNING id \
             ) \
             INSERT INTO events (name, description, start_date, end_date, visibility, category, \
                service_label, location, created_by, owner_id, recurrence_id, is_exception) \
             VALUES ($2, NULLIF($3, ''), $4, $5, $6::event_visibility, $7, NULLIF($8, ''), \
                NULLIF($9, ''), $1, $1, (SELECT id FROM rule), CASE WHEN $10 THEN false END) \
             RETURNING id"
        )
    }

    fn query_params(&self) -> &[QueryParam] {
        &self.params
    }
}

impl Display for CreateEventQueryView {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "CreateEventQueryView: creator_id={} name={}",
            self.creator_id(),
            self.name()
        )
    }
}
