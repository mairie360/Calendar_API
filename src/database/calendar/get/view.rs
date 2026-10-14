use std::fmt::Display;

use mairie360_api_lib::database::db_interface::{
    id_from_sql, id_to_sql, ApiRequestDto, QueryParam,
};
use mairie360_api_lib::smart_db::ReadCache;

use crate::database::event::get_event_members::view::Member;
use crate::database::event::model::{EventCategory, EventRecurrence, EventVisibility};

/// Read-cache namespace of `GET /calendar`: every write view that can change a calendar page
/// (event created, edited, deleted, member added or removed, approval decided) lists it in
/// `invalidated_read_caches`.
pub const CALENDAR_CACHE_NAMESPACE: &str = "calendar";

/// Time to live of a cached calendar page, in seconds. It bounds the staleness of what another
/// API changes behind this one (a user deleted in Core removes their memberships).
pub const CALENDAR_CACHE_TTL_SECONDS: u64 = 30;

/// Redis read cache of `GET /calendar` (MAIR-524), keyed by caller and period.
pub const CALENDAR_CACHE: ReadCache =
    ReadCache::new(CALENDAR_CACHE_NAMESPACE, CALENDAR_CACHE_TTL_SECONDS);

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
        //
        // The members of each listed event and the event's approval status (MAIR-524) come from
        // one lateral aggregate on `event_members (event_id, …)`, run on the filtered rows only,
        // which also gives `is_member`. They are exposed under the rule of `GET /events/{id}`
        // (`EventAccess::can_view`: public, member or **creator**); an event listed only because
        // the caller is its `owner_id` without being its creator nor a member gets `[]`, so the
        // list never shows members the detail route would refuse.
        concat!(
            "SELECT to_jsonb(t) FROM ( \
                SELECT e.id, e.name, e.start_date, e.end_date, e.category, \
                    e.service_label AS service, e.location, e.visibility, \
                    COALESCE(m.has_caller, false) AS is_member, \
                    CASE WHEN e.visibility = 'public' OR e.created_by = $3 \
                        OR COALESCE(m.has_caller, false) \
                    THEN COALESCE(m.members, '[]'::jsonb) ELSE '[]'::jsonb END AS members, ",
            crate::recurrence_json_sql!(),
            " AS recurrence \
                FROM events e LEFT JOIN recurrence_rules rr ON rr.id = e.recurrence_id \
                LEFT JOIN LATERAL ( \
                    SELECT jsonb_agg(jsonb_build_object('user_id', em.user_id, \
                            'validation_status', e.approval_status) ORDER BY em.user_id) AS members, \
                        bool_or(em.user_id = $3) AS has_caller \
                    FROM event_members em WHERE em.event_id = e.id \
                ) m ON true \
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
    /// Assigned members sorted by user id, each carrying the event's approval status; empty when
    /// the caller may not read the event's detail (see the query).
    #[serde(default)]
    pub members: Vec<Member>,
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
            members: Vec::new(),
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

    pub fn members(&self) -> &[Member] {
        &self.members
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
