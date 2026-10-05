use std::fmt::Display;

use mairie360_api_lib::database::db_interface::{
    id_from_sql, id_to_sql, ApiRequestDto, QueryParam,
};

use crate::database::event::model::EventInput;

/// Creates an event whose creator and owner is user `$1`, with its recurrence rule if any, and
/// returns its id. The event starts `pending` when its creator needs a Responsable's approval
/// (User or Guest only), `validated` otherwise.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CreateEventQueryView {
    params: Vec<QueryParam>,
}

impl CreateEventQueryView {
    pub fn new(creator_id: u64, input: &EventInput) -> Self {
        let mut params = vec![QueryParam::I32(id_to_sql(creator_id))];
        params.extend(input.query_params());
        Self { params }
    }

    pub fn creator_id(&self) -> u64 {
        id_from_sql(self.params[0].as_i32())
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
                service_label, location, created_by, owner_id, recurrence_id, is_exception, \
                approval_status) \
             VALUES ($2, NULLIF($3, ''), $4, $5, $6::event_visibility, $7, NULLIF($8, ''), \
                NULLIF($9, ''), $1, $1, (SELECT id FROM rule), CASE WHEN $10 THEN false END, \
                (CASE WHEN ",
            crate::creator_requires_approval_sql!("$1"),
            " THEN 'pending' ELSE 'validated' END)::event_validation_status) \
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
