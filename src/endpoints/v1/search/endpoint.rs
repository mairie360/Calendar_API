use crate::database::search::view::{GlobalSearchQueryView, SearchItem};
use crate::endpoints::error::{database_error, ApiError};
use crate::endpoints::v1::search::view::{like_escape, SearchQuery, SearchResultView};
use actix_web::{get, web, HttpResponse, Responder};
use mairie360_api_lib::security::AuthenticatedUser;
use mairie360_api_lib::state::AppState;

#[utoipa::path(
    get,
    path = "",
    summary = "Search the events (global search)",
    description = "Part of the global search of the shared header: every business API answers \
                   `GET /api/v1/search` with the same shape, and each BFF merges them. Calendar \
                   returns the **events** (`kind = event`) whose name contains `q`, \
                   case-insensitive, among those the caller sees in the calendar: every public \
                   event, and the private ones they own or are a member of. Names starting with \
                   `q` come first, then the latest start; `updated_at` is the start of the event \
                   (events carry no modification date).\n\n\
                   Not cached: the result depends on the term.",
    params(SearchQuery),
    responses(
        (
            status = 200,
            description = "At most `limit` events, best match first. Empty when nothing matches.",
            body = SearchResultView,
            example = json!({
                "items": [
                    {
                        "kind": "event",
                        "id": 21,
                        "title": "Town council",
                        "excerpt": "Vote on the initial budget",
                        "parent_id": null,
                        "updated_at": "2026-12-05T18:00:00Z"
                    }
                ]
            })
        ),
        (
            status = 400,
            description = "`q` is missing, shorter than 2 or longer than 100 characters once trimmed, or holds a control character; or `limit` is outside 1–20.",
            body = String,
            content_type = "text/plain",
            example = json!("Invalid `q`: must be at least 2 characters.")
        ),
        (
            status = 401,
            description = "Missing `Authorization` header, invalid or expired JWT, or revoked session.",
            body = String,
            content_type = "text/plain",
            example = json!("Jeton expiré")
        ),
        (
            status = 429,
            description = "The caller exceeded their request quota (`RATE_LIMIT_PER_SECOND` per second on \
                           average, bursts of `RATE_LIMIT_BURST`, counted per user). The \
                           `Retry-After` header gives the seconds to wait.",
            body = String,
            content_type = "text/plain",
            example = json!("Too many requests, retry in 1s.")
        ),
        (
            status = 500,
            description = "Database error while searching.",
            body = String,
            content_type = "text/plain",
            example = json!("An error occurred while accessing the database.")
        )
    ),
    tag = "Search",
    security(
        ("jwt" = [])
    )
)]
#[get("")]
pub async fn global_search(
    caller: AuthenticatedUser,
    state: web::Data<AppState>,
    query: web::Query<SearchQuery>,
) -> Result<impl Responder, ApiError> {
    query.validate()?;
    let view = GlobalSearchQueryView::new(caller.id, &like_escape(query.term()), query.limit());
    let items: Vec<SearchItem> = state
        .get_smart_db()
        .fetch_all(&view)
        .await
        .map_err(database_error)?;
    Ok(HttpResponse::Ok().json(SearchResultView { items }))
}
