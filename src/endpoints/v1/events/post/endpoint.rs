use std::collections::HashSet;

use actix_web::{post, web, HttpResponse, Responder};
use mairie360_api_lib::database::db_interface::id_from_sql;
use mairie360_api_lib::database::error::DbError;
use mairie360_api_lib::error::ApiLibError;
use mairie360_api_lib::security::AuthenticatedUser;
use mairie360_api_lib::state::AppState;

use crate::database::event::add_member::view::AddUserToEventQueryView;
use crate::database::event::create::view::CreateEventQueryView;
use crate::database::event::validation::view::CanAssignUserQueryView;
use crate::endpoints::error::{database_error, ApiError};
use crate::endpoints::json::JsonBody;
use crate::endpoints::v1::events::post::view::{PostEventResultView, PostEventView};
use crate::endpoints::v1::events::validate_event_input;

#[utoipa::path(
    post,
    path = "",
    summary = "Create an event",
    description = "Creates an event whose creator and owner is the caller. No particular role \
                   is required.\n\n\
                   `members` assigns users in the **same transaction** (MAIR-562): the event is \
                   created with all of them or not at all, so a refused or failed assignment \
                   leaves no event behind and needs no compensating `DELETE`. Each id must be in \
                   the caller's assignment scope (Admin and Maire: anyone, the others: themselves \
                   and the members of their groups), otherwise `403`. Prefer it to one \
                   `POST /api/v1/events/{event_id}/members/` per member, which costs a request \
                   of the rate limit each.\n\n\
                   The caller is **not** added to the members unless listed: they can read the \
                   event, manage its members and see it in their `GET /api/v1/calendar`, but must \
                   be a member to edit it. A \
                   `Public` event (the default) is readable and listed for every authenticated \
                   user.\n\n\
                   Approval: when the caller only has the User or Guest roles, the event is \
                   created `pending` and stays so until an assigned Responsable sharing a group \
                   with them approves or rejects it (`PATCH /api/v1/events/{event_id}/validation`). \
                   Other callers create it approved.\n\n\
                   Checks applied: `name` not blank and at most 150 characters, `service` at \
                   most 128 characters, `location` at most 255 characters (none of them with \
                   control characters), `description` at most 5000 characters, both dates between \
                   `1970-01-01T00:00:00Z` and `2999-12-31T23:59:59Z` with `events_end_time` \
                   strictly after `events_start_time`, a recurrence rule consistent with the start \
                   date, at most 500 distinct `members`, and no unknown field, also inside \
                   `recurrence`. Every refusal is a `400` \
                   whose text names the faulty field.\n\n\
                   The response only holds the new id.",
    request_body(
        content = PostEventView,
        description = "Event definition. `visibility` defaults to `Public` and `category` to `other`.",
        example = json!({
            "name": "Conseil municipal",
            "description": "Ordre du jour envoyé une semaine avant",
            "events_start_time": "2026-10-05T18:00:00Z",
            "events_end_time": "2026-10-05T20:00:00Z",
            "visibility": "Public",
            "category": "meeting",
            "service": "Secrétariat général",
            "location": "Salle du conseil",
            "recurrence": { "frequency": "monthly", "interval": 1, "days_of_week": null, "ends_on": "2027-06-30" },
            "members": [3, 51]
        })
    ),
    responses(
        (
            status = 201,
            description = "Event created with every user of `members` assigned. The caller is its creator, a member only if listed in `members`.",
            body = PostEventResultView,
            example = json!({ "event_id": 21 })
        ),
        (
            status = 400,
            description = "Invalid input. The text body names the faulty field as a JSON path and the rule it breaks (`Invalid `<field>`: <rule>.`); `body` designates the whole object (missing field, not an object). Causes: malformed JSON, wrong type, unknown field (also inside `recurrence`), a date outside `1970-01-01T00:00:00Z` – `2999-12-31T23:59:59Z`, end not after start, recurrence rule inconsistent with the start date (`interval` 1 to 365, `days_of_week` 1 to 7 distinct days 0–6, `ends_on` not before the start date nor after `2999-12-31`), more than 500 `members` or the same id listed twice (`members[<index>]`), or a text field breaking its rules: `name` 1 to 150 characters once trimmed, `service` at most 128 characters and `location` at most 255, all three without control characters; `description` at most 5000 characters, no control character other than line breaks and tabs. `<` and `>` are accepted everywhere (e.g. `budget > 10 000 €`) and returned as-is: the fronts escape what they display.",
            body = String,
            content_type = "text/plain",
            example = json!("Invalid `events_end_time`: must be strictly after `events_start_time`.")
        ),
        (
            status = 401,
            description = "Missing `Authorization` header, invalid or expired JWT, or revoked session.",
            body = String,
            content_type = "text/plain",
            example = json!("Jeton expiré")
        ),
        (
            status = 403,
            description = "A user of `members` is outside the caller's assignment scope (unknown or archived account included). Nothing is written: the event is not created.",
            body = String,
            content_type = "text/plain",
            example = json!("Forbidden.")
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
            description = "Database error. Nothing is written: the event is not created.",
            body = String,
            content_type = "text/plain",
            example = json!("An error occurred while accessing the database.")
        ),
    ),
    tag = "Events",
    security(
        ("jwt" = [])
    )
)]
#[post("/")]
pub async fn create_event(
    state: web::Data<AppState>,
    auth_user: AuthenticatedUser,
    view: JsonBody<PostEventView>,
) -> Result<impl Responder, ApiError> {
    let (input, members) = view.into_inner().into_parts();
    validate_event_input(&input)?;
    validate_members(&members)?;

    // One transaction: the event and its members are written together or not at all, so a
    // refused assignment never leaves an orphan event behind (MAIR-562).
    let mut tx = state.get_smart_db().begin().await.map_err(database_error)?;
    let event_id: i32 = tx
        .fetch_scalar(&CreateEventQueryView::new(auth_user.id, &input))
        .await
        .map_err(database_error)?;
    for &user_id in &members {
        let assignable: bool = tx
            .fetch_scalar(&CanAssignUserQueryView::new(auth_user.id, user_id))
            .await
            .map_err(database_error)?;
        if !assignable {
            return Err(ApiError::Forbidden);
        }
        match tx
            .execute(&AddUserToEventQueryView::new(
                user_id,
                id_from_sql(event_id),
            ))
            .await
        {
            Ok(()) => {}
            // Ids are distinct and the event is new: only a concurrent account change gets here.
            Err(ApiLibError::Database(DbError::ForeignKeyViolation(_))) => {
                return Err(ApiError::Forbidden)
            }
            Err(error) => return Err(database_error(error)),
        }
    }
    tx.commit().await.map_err(database_error)?;

    Ok(HttpResponse::Created().json(PostEventResultView {
        event_id: id_from_sql(event_id),
    }))
}

/// `members` of a creation body: at most [`MAX_CREATED_MEMBERS`] ids, none listed twice.
fn validate_members(members: &[u64]) -> Result<(), ApiError> {
    if members.len() > MAX_CREATED_MEMBERS {
        return Err(ApiError::invalid_field(
            "members",
            "must hold at most 500 user ids",
        ));
    }
    let mut seen = HashSet::with_capacity(members.len());
    for (index, user_id) in members.iter().enumerate() {
        if !seen.insert(user_id) {
            return Err(ApiError::invalid_field(
                &format!("members[{index}]"),
                "is listed twice",
            ));
        }
    }
    Ok(())
}

/// Users assignable in one creation: bounds the transaction (one query pair per member).
pub const MAX_CREATED_MEMBERS: usize = 500;
