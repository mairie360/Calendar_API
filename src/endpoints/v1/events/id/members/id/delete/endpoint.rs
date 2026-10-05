use actix_web::{delete, web, HttpResponse, Responder};
use mairie360_api_lib::database::error::DbError;
use mairie360_api_lib::error::ApiLibError;
use mairie360_api_lib::security::AuthenticatedUser;
use mairie360_api_lib::state::AppState;

use crate::database::event::access::view::EventAccess;
use crate::database::event::remove_member::view::RemoveUserFromEventQueryView;
use crate::endpoints::error::{database_error, require_event_access_in, ApiError};

#[utoipa::path(
    delete,
    path = "",
    summary = "Remove a member from an event",
    description = "Unassigns a user from the event. The approval of the event is not touched: \
                   removing the Responsable who rejected it leaves it rejected.\n\n\
                   Reserved to whoever can manage the members: the creator, or someone allowed to \
                   edit the event.\n\n\
                   Not idempotent: removing someone who is not assigned answers `404`, like an \
                   unknown event.",
    params(
        ("event_id" = u64, Path, description = "Event id.", example = 21),
        ("member_id" = u64, Path, description = "Core API id of the member to remove.", example = 51)
    ),
    responses(
        (
            status = 204,
            description = "Member removed. Empty body.",
        ),
        (
            status = 400,
            description = "A path segment is not an integer.",
            body = String,
            content_type = "text/plain",
            example = json!("Path deserialize error: can not parse `abc` to a u64")
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
            description = "The caller cannot manage the members of this event.",
            body = String,
            content_type = "text/plain",
            example = json!("Forbidden.")
        ),
        (
            status = 404,
            description = "No event has this id, or the user is not assigned to it.",
            body = String,
            content_type = "text/plain",
            example = json!("Unknown event.")
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
#[delete("/")]
pub async fn remove_event_member(
    state: web::Data<AppState>,
    auth_user: AuthenticatedUser,
    path: web::Path<(u64, u64)>,
) -> Result<impl Responder, ApiError> {
    let (event_id, member_id) = path.into_inner();
    let mut tx = state.get_smart_db().begin().await.map_err(database_error)?;
    require_event_access_in(
        &mut tx,
        event_id,
        auth_user.id,
        EventAccess::can_manage_members,
    )
    .await?;

    // `RETURNING user_id` yields a bare integer column: it must be read as a scalar
    // (`fetch_all` expects a JSON row and failed with a column decode error, hence a `500`).
    // No row means the user was not assigned to the event.
    match tx
        .fetch_scalar::<i32, _>(&RemoveUserFromEventQueryView::new(member_id, event_id))
        .await
    {
        Ok(_) => {}
        Err(ApiLibError::Database(DbError::NotFound)) => return Err(ApiError::NotFound),
        Err(error) => return Err(database_error(error)),
    }
    tx.commit().await.map_err(database_error)?;

    Ok(HttpResponse::NoContent().finish())
}
