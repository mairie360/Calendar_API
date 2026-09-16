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
    request_body = PostEventView,
    responses(
        (status = 201, description = "Event created; the caller is its creator and owner", body = PostEventResultView),
        (status = 400, description = "Bad request"),
        (status = 500, description = "Internal server error")
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
