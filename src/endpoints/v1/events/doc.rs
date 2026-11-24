use utoipa::OpenApi;

use crate::database::event::access::view::ApprovalStatus;
use crate::database::event::get_event_members::view::{EventValidationStatus, Participation};
use crate::database::event::model::{
    EventCategory, EventRecurrence, EventVisibility, RecurrenceFrequency,
};
use crate::endpoints::v1::events::id::delete::endpoint::__path_delete_event;
use crate::endpoints::v1::events::id::get::endpoint::__path_get_event;
use crate::endpoints::v1::events::id::get::view::{
    EventPermissionsView, GetEventResultView, Member,
};
use crate::endpoints::v1::events::id::members::get::endpoint::__path_get_event_members;
use crate::endpoints::v1::events::id::members::get::view::GetMembersResultView;
use crate::endpoints::v1::events::id::members::id::delete::endpoint::__path_remove_event_member;
use crate::endpoints::v1::events::id::members::post::endpoint::__path_add_event_member;
use crate::endpoints::v1::events::id::members::post::view::PostMemberView;
use crate::endpoints::v1::events::id::participation::endpoint::__path_update_participation;
use crate::endpoints::v1::events::id::participation::view::{
    ParticipationAnswer, UpdateParticipationView,
};
use crate::endpoints::v1::events::id::patch::endpoint::__path_patch_event;
use crate::endpoints::v1::events::id::patch::view::PatchEventView;
use crate::endpoints::v1::events::id::validation::endpoint::__path_update_event_validation;
use crate::endpoints::v1::events::id::validation::view::UpdateEventValidationView;
use crate::endpoints::v1::events::post::endpoint::__path_create_event;
use crate::endpoints::v1::events::post::view::{PostEventResultView, PostEventView};

// One operation per path: utoipa replaces (instead of merging) two `nest` that lead to the same
// path, so the operations of one path are listed in a single document.
#[derive(OpenApi)]
#[openapi(nest(
    (path = "/", api = RootDoc),
    (path = "/{event_id}/", api = EventDoc),
    (path = "/{event_id}/members/", api = MembersDoc),
    (path = "/{event_id}/participation", api = ParticipationDoc),
    (path = "/{event_id}/members/{member_id}/", api = MemberDoc),
))]
pub struct EventsDoc;

#[derive(OpenApi)]
#[openapi(
    paths(create_event),
    components(schemas(
        PostEventView,
        PostEventResultView,
        EventCategory,
        EventVisibility,
        EventRecurrence,
        RecurrenceFrequency
    ))
)]
struct RootDoc;

#[derive(OpenApi)]
#[openapi(
    paths(get_event, patch_event, delete_event, update_event_validation),
    components(schemas(
        GetEventResultView,
        EventPermissionsView,
        Member,
        EventValidationStatus,
        Participation,
        ApprovalStatus,
        PatchEventView,
        UpdateEventValidationView
    ))
)]
struct EventDoc;

#[derive(OpenApi)]
#[openapi(
    paths(get_event_members, add_event_member),
    components(schemas(GetMembersResultView, PostMemberView))
)]
struct MembersDoc;

#[derive(OpenApi)]
#[openapi(paths(remove_event_member))]
struct MemberDoc;

#[derive(OpenApi)]
#[openapi(
    paths(update_participation),
    components(schemas(UpdateParticipationView, ParticipationAnswer))
)]
struct ParticipationDoc;
