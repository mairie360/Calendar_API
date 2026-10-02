use actix_web::{post, web, HttpResponse, Responder};
use mairie360_api_lib::database::error::DbError;
use mairie360_api_lib::error::ApiLibError;
use mairie360_api_lib::security::AuthenticatedUser;
use mairie360_api_lib::state::AppState;

use crate::database::event::access::view::EventAccess;
use crate::database::event::add_member::view::AddUserToEventQueryView;
use crate::database::event::validation::view::CanAssignUserQueryView;
use crate::endpoints::error::{database_error, require_event_access_in, ApiError};
use crate::endpoints::v1::events::id::members::post::view::PostMemberView;

#[utoipa::path(
    post,
    path = "",
    summary = "Assign a member to an event",
    description = "Assigns a user to the event. The approval of the event is not touched: a \
                   decision already taken (approved or rejected) stays, and a pending event stays \
                   pending.\n\n\
                   Two independent checks, both answered with `403`: the caller must be able to \
                   manage the members (creator, or someone allowed to edit the event), **and** the \
                   target user must be in their assignment scope (Admin and Maire: anyone, the \
                   others: themselves and the members of their groups). The body does not tell \
                   the two apart.\n\n\
                   This is how the creator adds themselves after `POST /api/v1/events/`. The \
                   response has an empty body.",
    params(
        ("event_id" = u64, Path, description = "Event id.", example = 21)
    ),
    request_body(
        content = PostMemberView,
        description = "Core API id of the user to assign.",
        example = json!({ "user_id": 51 })
    ),
    responses(
        (
            status = 201,
            description = "Member assigned. Empty body.",
        ),
        (
            status = 400,
            description = "Malformed JSON body, unknown field, `event_id` not an integer, or missing `user_id`.",
            body = String,
            content_type = "text/plain",
            example = json!("Json deserialize error: missing field `user_id`")
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
            status = 403,
            description = "The caller cannot manage the members of this event, or the target user is outside their assignment scope.",
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
            status = 409,
            description = "The user is already assigned to this event.",
            body = String,
            content_type = "text/plain",
            example = json!("Conflict.")
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
#[post("/")]
pub async fn add_event_member(
    state: web::Data<AppState>,
    auth_user: AuthenticatedUser,
    event_id: web::Path<u64>,
    view: web::Json<PostMemberView>,
) -> Result<impl Responder, ApiError> {
    let event_id = event_id.into_inner();
    // One transaction: the event row stays locked from the access check to the insertion, and the
    // assignment scope is read in the same snapshot (MAIR-420).
    let mut tx = state.get_smart_db().begin().await.map_err(database_error)?;
    require_event_access_in(
        &mut tx,
        event_id,
        auth_user.id,
        EventAccess::can_manage_members,
    )
    .await?;

    let assignable: bool = tx
        .fetch_scalar(&CanAssignUserQueryView::new(auth_user.id, view.user_id))
        .await
        .map_err(database_error)?;
    if !assignable {
        return Err(ApiError::Forbidden);
    }
    // Unique index on (event_id, user_id): only a duplicate is a conflict, any other failure is
    // a database error.
    match tx
        .execute(&AddUserToEventQueryView::new(view.user_id, event_id))
        .await
    {
        Ok(()) => {}
        Err(ApiLibError::Database(DbError::UniqueViolation(_))) => return Err(ApiError::Conflict),
        Err(error) => return Err(database_error(error)),
    }
    tx.commit().await.map_err(database_error)?;

    Ok(HttpResponse::Created().finish())
}
