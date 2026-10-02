use mairie360_api_lib::database::db_interface::{
    id_from_sql, id_to_sql, ApiRequestDto, QueryParam,
};

use crate::database::event::model::EventInput;

/// Replaces the data of an event and creates, updates or detaches its recurrence rule. Returns
/// `true` if the event exists. With `reset_approval` (`$15`), the event goes back to `pending` and
/// loses its decision. A detached rule stays in the database: `DeleteOrphanRecurrenceQueryView`
/// deletes it afterwards (deleting it in the same statement would conflict with the ON DELETE SET
/// NULL on the event row this statement updates). The PATCH handler runs both in one transaction.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct EditEventQueryView {
    params: Vec<QueryParam>,
}

impl EditEventQueryView {
    pub fn new(event_id: u64, input: &EventInput, reset_approval: bool) -> Self {
        let mut params = vec![QueryParam::I32(id_to_sql(event_id))];
        params.extend(input.query_params());
        params.push(QueryParam::Bool(reset_approval));
        Self { params }
    }

    pub fn id(&self) -> u64 {
        id_from_sql(self.params[0].as_i32())
    }

    pub fn name(&self) -> &str {
        self.params[1].as_text()
    }
}

impl ApiRequestDto for EditEventQueryView {
    fn query_sql(&self) -> &'static str {
        concat!(
            "WITH target AS ( \
                SELECT id, recurrence_id, owner_id, owner_group_id, created_by FROM events WHERE id = $1 \
             ), updated_rule AS ( \
                UPDATE recurrence_rules rr SET (type_recurrence, intervalle, days_of_week, start_date, \
                    end_date, start_time, duration, visibility) = (",
            crate::recurrence_rule_values_sql!(),
            ") FROM target WHERE rr.id = target.recurrence_id AND $10 RETURNING rr.id \
             ), inserted_rule AS ( \
                INSERT INTO recurrence_rules (type_recurrence, intervalle, days_of_week, start_date, \
                    end_date, start_time, duration, visibility, owner_id, owner_group_id) \
                SELECT ",
            crate::recurrence_rule_values_sql!(),
            ", CASE WHEN target.owner_group_id IS NULL THEN COALESCE(target.owner_id, target.created_by) END, \
                  target.owner_group_id \
                FROM target WHERE $10 AND target.recurrence_id IS NULL RETURNING id \
             ), updated AS ( \
                UPDATE events SET name = $2, description = NULLIF($3, ''), start_date = $4, end_date = $5, \
                    visibility = $6::event_visibility, category = $7, service_label = NULLIF($8, ''), \
                    location = NULLIF($9, ''), \
                    recurrence_id = CASE WHEN $10 THEN COALESCE((SELECT id FROM updated_rule), \
                        (SELECT id FROM inserted_rule)) END, \
                    is_exception = CASE WHEN $10 THEN false END, \
                    approval_status = CASE WHEN $15 THEN 'pending'::event_validation_status \
                        ELSE approval_status END, \
                    approval_decided_by = CASE WHEN $15 THEN NULL ELSE approval_decided_by END, \
                    approval_decided_at = CASE WHEN $15 THEN NULL ELSE approval_decided_at END \
                WHERE id = $1 RETURNING id \
             ) \
             SELECT EXISTS (SELECT 1 FROM updated)"
        )
    }

    fn query_params(&self) -> &[QueryParam] {
        &self.params
    }
}

/// Deletes a recurrence rule no event uses any more.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct DeleteOrphanRecurrenceQueryView {
    params: Vec<QueryParam>,
}

impl DeleteOrphanRecurrenceQueryView {
    pub fn new(recurrence_id: u64) -> Self {
        Self {
            params: vec![QueryParam::I32(id_to_sql(recurrence_id))],
        }
    }
}

impl ApiRequestDto for DeleteOrphanRecurrenceQueryView {
    fn query_sql(&self) -> &'static str {
        "DELETE FROM recurrence_rules rr WHERE rr.id = $1 \
         AND NOT EXISTS (SELECT 1 FROM events e WHERE e.recurrence_id = rr.id)"
    }

    fn query_params(&self) -> &[QueryParam] {
        &self.params
    }
}
