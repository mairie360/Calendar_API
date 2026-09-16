use mairie360_api_lib::database::db_interface::{ApiRequestDto, QueryParam};

use crate::database::event::model::EventInput;

/// Remplace les données d'un événement et crée, met à jour ou détache sa règle de répétition. Renvoie
/// `true` si l'événement existe. Une règle détachée reste en base : `DeleteOrphanRecurrenceQueryView`
/// la supprime ensuite (la supprimer dans la même requête entrerait en conflit avec le ON DELETE SET NULL).
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct EditEventQueryView {
    params: Vec<QueryParam>,
}

impl EditEventQueryView {
    pub fn new(event_id: u64, input: &EventInput) -> Self {
        let mut params = vec![QueryParam::I32(event_id as i32)];
        params.extend(input.query_params());
        Self { params }
    }

    pub fn id(&self) -> u64 {
        self.params[0].as_i32() as u64
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
                    is_exception = CASE WHEN $10 THEN false END \
                WHERE id = $1 RETURNING id \
             ) \
             SELECT EXISTS (SELECT 1 FROM updated)"
        )
    }

    fn query_params(&self) -> &[QueryParam] {
        &self.params
    }
}

/// Supprime une règle de répétition qui n'est plus rattachée à aucun événement.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct DeleteOrphanRecurrenceQueryView {
    params: Vec<QueryParam>,
}

impl DeleteOrphanRecurrenceQueryView {
    pub fn new(recurrence_id: u64) -> Self {
        Self {
            params: vec![QueryParam::I32(recurrence_id as i32)],
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
