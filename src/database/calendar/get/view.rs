use std::fmt::Display;

use mairie360_api_lib::database::db_interface::{
    id_from_sql, id_to_sql, ApiRequestDto, QueryParam,
};

use crate::database::event::model::{EventCategory, EventRecurrence, EventVisibility};

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct GetCalendarQueryView {
    params: Vec<QueryParam>,
}

impl GetCalendarQueryView {
    pub fn new(
        start: chrono::DateTime<chrono::Utc>,
        end: chrono::DateTime<chrono::Utc>,
        user_id: u64,
    ) -> Self {
        Self {
            params: vec![
                QueryParam::DateTime(start),
                QueryParam::DateTime(end),
                QueryParam::I32(id_to_sql(user_id)),
            ],
        }
    }

    pub fn start(&self) -> chrono::DateTime<chrono::Utc> {
        self.params[0].as_datetime()
    }

    pub fn end(&self) -> chrono::DateTime<chrono::Utc> {
        self.params[1].as_datetime()
    }

    pub fn user_id(&self) -> u64 {
        id_from_sql(self.params[2].as_i32())
    }
}

impl Display for GetCalendarQueryView {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "GetCalendarQueryView: start={} end={} user_id={}",
            self.start(),
            self.end(),
            self.user_id()
        )
    }
}

impl ApiRequestDto for GetCalendarQueryView {
    fn query_sql(&self) -> &'static str {
        // Public events and the events the user owns or is a member of, that overlap the period or
        // whose recurrence rule overlaps it (rule end exclusive).
        //
        // The candidates are collected first from indexed lookups (public events of the period or
        // with an overlapping rule, the user's own events, their memberships), then filtered on
        // the period. Written as one `visibility OR owner OR member` predicate ANDed with `dates OR
        // rule`, Postgres walked every event of the table and its estimated cost switched JIT on:
        // 180 ms per call on 50 000 events, 4 to 10 ms this way with the same rows (MAIR-474).
        concat!(
            "SELECT to_jsonb(t) FROM ( \
                SELECT e.id, e.name, e.start_date, e.end_date, e.category, \
                    e.service_label AS service, e.location, e.visibility, \
                    EXISTS (SELECT 1 FROM event_members em \
                        WHERE em.event_id = e.id AND em.user_id = $3) AS is_member, ",
            crate::recurrence_json_sql!(),
            " AS recurrence \
                FROM events e LEFT JOIN recurrence_rules rr ON rr.id = e.recurrence_id \
                WHERE e.id IN ( \
                        SELECT pub.id FROM events pub \
                        WHERE pub.visibility = 'public' AND pub.start_date <= $2 \
                          AND pub.end_date >= $1 \
                        UNION ALL \
                        SELECT pub.id FROM events pub \
                            JOIN recurrence_rules prr ON prr.id = pub.recurrence_id \
                        WHERE pub.visibility = 'public' AND prr.start_date <= $2 \
                          AND (prr.end_date IS NULL OR prr.end_date > $1) \
                        UNION ALL \
                        SELECT own.id FROM events own WHERE own.owner_id = $3 \
                        UNION ALL \
                        SELECT mine.event_id FROM event_members mine WHERE mine.user_id = $3) \
                  AND ((e.start_date <= $2 AND e.end_date >= $1) \
                    OR (rr.id IS NOT NULL AND rr.start_date <= $2 \
                        AND (rr.end_date IS NULL OR rr.end_date > $1))) \
                ORDER BY e.start_date, e.id \
             ) t"
        )
    }

    fn query_params(&self) -> &[QueryParam] {
        &self.params
    }
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct Event {
    pub id: i32,
    pub name: String,
    pub start_date: chrono::DateTime<chrono::Utc>,
    pub end_date: chrono::DateTime<chrono::Utc>,
    #[serde(default)]
    pub category: EventCategory,
    #[serde(default)]
    pub service: Option<String>,
    #[serde(default)]
    pub location: Option<String>,
    #[serde(default)]
    pub is_member: bool,
    /// `public` or `private`, as stored in `events.visibility`.
    #[serde(default)]
    pub visibility: String,
    #[serde(default)]
    pub recurrence: Option<EventRecurrence>,
}

impl Event {
    pub fn new(
        id: i32,
        name: &str,
        start_date: chrono::DateTime<chrono::Utc>,
        end_date: chrono::DateTime<chrono::Utc>,
    ) -> Self {
        Self {
            id,
            name: name.to_string(),
            start_date,
            end_date,
            category: EventCategory::Other,
            service: None,
            location: None,
            is_member: false,
            visibility: "public".to_string(),
            recurrence: None,
        }
    }

    pub fn visibility(&self) -> EventVisibility {
        EventVisibility::from_db(&self.visibility)
    }

    pub fn id(&self) -> i32 {
        self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn start_date(&self) -> &chrono::DateTime<chrono::Utc> {
        &self.start_date
    }

    pub fn end_date(&self) -> &chrono::DateTime<chrono::Utc> {
        &self.end_date
    }
}

impl Display for Event {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Event: id={} name={} start={} end={}",
            self.id, self.name, self.start_date, self.end_date
        )
    }
}
