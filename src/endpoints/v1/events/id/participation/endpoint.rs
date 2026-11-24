use actix_web::{put, web, HttpResponse, Responder};
use mairie360_api_lib::database::error::DbError;
use mairie360_api_lib::error::ApiLibError;
use mairie360_api_lib::security::AuthenticatedUser;
use mairie360_api_lib::state::AppState;

use crate::database::event::access::view::EventAccess;
use crate::database::event::participation::view::SetParticipationQueryView;
use crate::endpoints::error::{database_error, require_event_access_in, ApiError};
use crate::endpoints::json::JsonBody;
use crate::endpoints::v1::events::id::participation::view::UpdateParticipationView;

#[utoipa::path(
    put,
    path = "",
    summary = "Answer the invitation to an event",
    description = "Records the caller's own answer to the event: `accepted` (they take part) or \
                   `declined`. Only an assigned member answers, for themselves only; anyone else \
                   gets `403`. The answer can be changed at any time and does not touch the \
                   approval of the event.\n\n\
                   Read it back as `my_participation` of `GET /api/v1/events/{event_id}/` and of \
                   `GET /api/v1/calendar`, or as `participation` of each member. The response has \
                   an empty body.",
    params(
        ("event_id" = u64, Path, description = "Event id.", example = 21)
    ),
    request_body(
        content = UpdateParticipationView,
        description = "Answer of the caller.",
        example = json!({ "status": "accepted" })
    ),
    responses(
        (
            status = 204,
            description = "Answer recorded. Empty body.",
        ),
        (
            status = 400,
            description = "Malformed JSON body, unknown field, `event_id` not an integer, or unknown status. A body error names the faulty field (`body` for the whole object).",
            body = String,
            content_type = "text/plain",
            example = json!("Invalid `status`: unknown variant `maybe`, expected `accepted` or `declined`.")
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
            description = "The caller is not assigned to the event.",
            body = String,
            content_type = "text/plain",
            example = json!("Forbidden.")
        ),
        (
            status = 404,
            description = "No event has this id.",
            body = String,
            content_type = "text/plain",
            example = json!("Unknown event.")
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
            description = "Database error.",
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
#[put("/participation")]
pub async fn update_participation(
    state: web::Data<AppState>,
    auth_user: AuthenticatedUser,
    event_id: web::Path<u64>,
    view: JsonBody<UpdateParticipationView>,
) -> Result<impl Responder, ApiError> {
    let event_id = event_id.into_inner();
    // Membership checked on the locked event, in the transaction of the answer.
    let mut tx = state.get_smart_db().begin().await.map_err(database_error)?;
    require_event_access_in(&mut tx, event_id, auth_user.id, EventAccess::can_respond).await?;
    match tx
        .fetch_scalar::<i32, _>(&SetParticipationQueryView::new(
            event_id,
            auth_user.id,
            view.status.into(),
        ))
        .await
    {
        Ok(_) => {}
        Err(ApiLibError::Database(DbError::NotFound)) => return Err(ApiError::Forbidden),
        Err(error) => return Err(database_error(error)),
    }
    tx.commit().await.map_err(database_error)?;
    Ok(HttpResponse::NoContent().finish())
}
