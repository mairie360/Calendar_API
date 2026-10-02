use actix_web::{post, web, HttpResponse, Responder};
use mairie360_api_lib::security::AuthenticatedUser;
use mairie360_api_lib::state::AppState;

use crate::database::event::create::view::CreateEventQueryView;
use crate::database::event::model::EventInput;
use crate::endpoints::error::{database_error, ApiError};
use crate::endpoints::v1::events::post::view::{PostEventResultView, PostEventView};
use crate::endpoints::v1::events::validate_event_input;

#[utoipa::path(
    post,
    path = "",
    summary = "Create an event",
    description = "Creates an event whose creator and owner is the caller. No particular role \
                   is required.\n\n\
                   The caller is **not** added to the members: they can read the event, manage \
                   its members and see it in their `GET /api/v1/calendar`, but must assign \
                   themselves through `POST /api/v1/events/{event_id}/members/` to edit it. A \
                   `Public` event (the default) is readable and listed for every authenticated \
                   user.\n\n\
                   Approval: when the caller only has the User or Guest roles, the event is \
                   created `pending` and stays so until an assigned Responsable sharing a group \
                   with them approves or rejects it (`PATCH /api/v1/events/{event_id}/validation`). \
                   Other callers create it approved.\n\n\
                   Checks applied: `name` not blank and at most 150 characters, `service` at \
                   most 128 characters, `location` at most 255 characters (none of them with \
                   control characters or `<` / `>`), `description` at most 5000 characters \
                   without `<` / `>`, `events_end_time` strictly after `events_start_time`, a \
                   recurrence rule consistent with the start date, and no unknown field. They all \
                   share the same `400`.\n\n\
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
            "recurrence": { "frequency": "monthly", "interval": 1, "days_of_week": null, "ends_on": "2027-06-30" }
        })
    ),
    responses(
        (
            status = 201,
            description = "Event created. The caller is its creator, not yet a member.",
            body = PostEventResultView,
            example = json!({ "event_id": 21 })
        ),
        (
            status = 400,
            description = "Malformed JSON body, unknown field, end not after start, recurrence rule inconsistent with the start date, or a text field breaking its rules: `name` 1 to 150 characters once trimmed, `service` at most 128 characters and `location` at most 255, all three without control characters nor `<` / `>`; `description` at most 5000 characters, no `<` / `>`, no control character other than line breaks and tabs.",
            body = String,
            content_type = "text/plain",
            example = json!("Bad request.")
        ),
        (
            status = 401,
            description = "Missing `Authorization` header, invalid or expired JWT, or revoked session.",
            body = String,
            content_type = "text/plain",
            example = json!("Jeton expiré")
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
pub async fn create_event(
    state: web::Data<AppState>,
    auth_user: AuthenticatedUser,
    view: web::Json<PostEventView>,
) -> Result<impl Responder, ApiError> {
    let input = EventInput::from(view.into_inner());
    validate_event_input(&input)?;

    let event_id: i32 = state
        .get_smart_db()
        .fetch_scalar(&CreateEventQueryView::new(auth_user.id, &input))
        .await
        .map_err(database_error)?;

    Ok(HttpResponse::Created().json(PostEventResultView {
        event_id: event_id as u64,
    }))
}
