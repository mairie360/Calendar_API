use mairie360_api_lib::database::db_interface::{id_to_sql, ApiRequestDto, QueryParam};
use serde::{Deserialize, Serialize};
use std::fmt::Display;
use utoipa::ToSchema;

/// What a global search result is. Calendar finds events only.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, ToSchema, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum SearchKind {
    /// An event; `id` is reusable in `GET /api/v1/events/{event_id}/`.
    Event,
}

/// One result of the global search.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, PartialEq, Eq)]
pub struct SearchItem {
    pub kind: SearchKind,
    /// Id of the event.
    #[schema(example = 21)]
    pub id: i64,
    /// Name of the event.
    #[schema(example = "Town council")]
    pub title: String,
    /// First 120 characters of the description, `null` without one.
    #[schema(example = "Vote on the initial budget")]
    pub excerpt: Option<String>,
    /// Always `null` for an event (no parent item).
    #[schema(example = json!(null))]
    pub parent_id: Option<i64>,
    /// Start of the event (UTC, RFC 3339): events carry no modification date.
    #[schema(format = DateTime, example = "2026-12-05T18:00:00Z")]
    pub updated_at: String,
}

/// Events matching a global search (`GET /api/v1/search`), among those the caller sees in the
/// calendar: every public event, and the private ones they own or are a member of. An event
/// matches on its name through the trigram index. Names starting with the term come first, then
/// the latest start.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GlobalSearchQueryView {
    params: Vec<QueryParam>,
}

impl GlobalSearchQueryView {
    /// `pattern` is the term already escaped for `LIKE` (see `like_escape`), `limit` bounded by
    /// the endpoint.
    pub fn new(user_id: u64, pattern: &str, limit: u64) -> Self {
        Self {
            params: vec![
                QueryParam::I32(id_to_sql(user_id)),
                QueryParam::Text(pattern.to_string()),
                QueryParam::I64(i64::try_from(limit).unwrap_or(i64::MAX)),
            ],
        }
    }
}

impl ApiRequestDto for GlobalSearchQueryView {
    fn query_sql(&self) -> &'static str {
        "SELECT jsonb_build_object( \
            'kind', 'event', 'id', e.id, 'title', e.name, \
            'excerpt', NULLIF(left(btrim(COALESCE(e.description, '')), 120), ''), \
            'parent_id', NULL, \
            'updated_at', to_char(e.start_date AT TIME ZONE 'UTC', 'YYYY-MM-DD\"T\"HH24:MI:SS\"Z\"')) \
         FROM events e \
         WHERE e.name ILIKE '%' || $2 || '%' \
           AND (e.visibility = 'public' OR e.owner_id = $1 \
                OR EXISTS (SELECT 1 FROM event_members em \
                           WHERE em.event_id = e.id AND em.user_id = $1)) \
         ORDER BY (e.name ILIKE $2 || '%') DESC, e.start_date DESC, e.id DESC \
         LIMIT $3"
    }

    fn query_params(&self) -> &[QueryParam] {
        &self.params
    }
}

impl Display for GlobalSearchQueryView {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // The search term is not logged.
        write!(f, "GlobalSearchQueryView: limit={:?}", self.params[2])
    }
}
